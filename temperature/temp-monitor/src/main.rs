#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::hal::port::PB3;
use arduino_hal::port::mode::OpenDrain;
use arduino_hal::{delay_ms, Delay};
use avr_device::interrupt::free;
use panic_halt as _;

use arduino_hal::clock::Clock;
use arduino_hal::{
    clock::MHz16,
    hal::{
        port::{PD0, PD1},
        Usart,
    },
    pac::USART0,
    port::{
        mode::{Input, Output},
        Pin,
    },
};
use avr_device::atmega328p::{tc1::tccr1b::CS1_A, TC1};
use dht11::{Dht11, Measurement};

type Serial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>;
type Dht11Sensor = Dht11<Pin<OpenDrain, PB3>>;

const INTERVAL: f32 = 3.33333;
const PRESCALER: u32 = 1024;
const TIMER_COUNTS: u32 = 51563;

const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000;

static mut MILLIS_COUNTER: u32 = 0u32;

// Global variables for storing the sensor data and timestamp
static mut SENSOR_READY: bool = false;
static mut TEMPERATURE: i16 = 0;
static mut TIMESTAMP: u32 = 0;

static mut INTERRUPT_COUNTER: u8 = 0;

struct Sensor {
    sensor: Dht11Sensor,
    delay: Delay,
}

impl Sensor {
    fn perform_measurement(
        &mut self,
    ) -> Result<Measurement, dht11::Error<core::convert::Infallible>> {
        self.sensor.perform_measurement(&mut self.delay)
    }
}

static mut SENSOR: Option<Sensor> = None;
static mut SERIAL: Option<Serial> = None;

// Define a timer interrupt handler
#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    unsafe {
        MILLIS_COUNTER += MILLIS_INCREMENT;

        // Initialize the Arduino Uno peripherals
        if let Some(sensor) = &mut SENSOR {
            // Read data from the sensor every third interrupt
            if INTERRUPT_COUNTER < 2 {
                INTERRUPT_COUNTER += 1;
            } else {
                match sensor.perform_measurement() {
                    Ok(Measurement {
                        temperature,
                        humidity: _,
                    }) => {
                        TEMPERATURE = temperature; // Save the temperature reading
                        TIMESTAMP = MILLIS_COUNTER; // Get the current time in milliseconds

                        SENSOR_READY = true;
                    }
                    Err(e) => {
                        if let Some(serial) = &mut SERIAL {
                            let _ = match e {
                                dht11::Error::Gpio(_) => ufmt::uwriteln!(serial, "Pin Error!"),
                                dht11::Error::CrcMismatch => {
                                    ufmt::uwriteln!(serial, "Checksum Mismatch!")
                                }
                                dht11::Error::Timeout => ufmt::uwriteln!(serial, "Timeout!"),
                            };
                        }
                    }
                };

                INTERRUPT_COUNTER = 0; // Reset the counter
            }
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    // Initialize the Arduino Uno peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let tmr1: TC1 = dp.TC1;
    let mut serial: Serial = arduino_hal::default_serial!(dp, pins, 9600);

    // Configure the pin the sensor is connected to and delay to make sure it's ready.
    {
        let mut d11 = pins.d11.into_opendrain_high();
        d11.set_high();

        unsafe {
            SENSOR = Some(Sensor {
                sensor: Dht11::new(d11),
                delay: Delay::new(),
            });
        }

        delay_ms(1000);
    }

    let _ticks = setup_timer(&tmr1);

    // Enable global interrupts
    unsafe { avr_device::interrupt::enable() };

    let mut timestamp = 0u32;
    let mut temperature = 0i16;
    let mut temp_f32: f32;
    let mut temp_x100: i32;
    let mut whole: i16;
    let mut frac: i16;

    loop {
        // Check if the sensor data is ready
        if free(|_cs| unsafe { SENSOR_READY }) {

            // Reset the flag.
            unsafe {
                SENSOR_READY = false;
            }

            // Read the temperature and timestamp.
            free(|_cs| {
                timestamp = unsafe { TIMESTAMP };
                temperature = unsafe { TEMPERATURE };
            });

            // Convert to Fahrenheit.
            temp_f32 = (((temperature as f32) / 10f32) * 1.8f32) + 32f32;

            // Multiply by 100 to get 2 decimal places.
            temp_x100 = (temp_f32 * 100f32) as i32;

            // Get whole and fractional parts for formatting the string.
            whole = (temp_x100 / 100) as i16;
            frac = (temp_x100 % 100) as i16;

            // Send the data over the serial port in the required format
            let _ = ufmt::uwriteln!(&mut serial, "{},{}.{}", timestamp, whole, frac);
        }
    }
}

pub fn setup_timer(timer: &TC1) -> u16 {
    // Configure Timer1 to trigger an interrupt every 3.3 seconds
    const CLOCK_SOURCE: CS1_A = CS1_A::PRESCALE_1024;

    // ticks should be 51563 for 3.3 seconds
    let ticks: u16 =
        ((INTERVAL / (PRESCALER as f32) * (arduino_hal::DefaultClock::FREQ as f32)) - 1f32) as u16;

    unsafe { MILLIS_COUNTER = 0 };

    timer.tccr1a.write(|w| w.wgm1().bits(0b00));
    timer.tccr1b.write(|w| {
        w.cs1()
            //.prescale_1024()
            .variant(CLOCK_SOURCE)
            .wgm1()
            .bits(0b01)
    });
    timer.ocr1a.write(|w| w.bits(ticks));
    timer.timsk1.write(|w| w.ocie1a().set_bit());

    ticks
}
