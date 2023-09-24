#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

use arduino_hal::{
    clock::{Clock, MHz16},
    delay_ms,
    hal::port::PB3,
    hal::{
        port::{PD0, PD1},
        Usart,
    },
    pac::USART0,
    port::mode::OpenDrain,
    port::{
        mode::{Input, Output},
        Pin,
    },
    Delay,
};
use avr_device::{interrupt::free, atmega328p::{tc1::tccr1b::CS1_A, TC1}};
use core::sync::atomic::AtomicBool;
use dht11::{Dht11, Measurement};


// Convenience type aliases.
type Serial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>;
type Dht11Sensor = Dht11<Pin<OpenDrain, PB3>>;

// Constants and globals.
const INTERVAL: f32 = 3.33333;
const PRESCALER: u32 = 1024;

const CALIBRATION_OFFSET: f32 = 0f32;
const CALIBRATION_FACTOR: f32 = 1f32;

static mut MILLIS_INCREMENT: u32 = 0u32;
static mut MILLIS_COUNTER: u32 = 0u32;
static mut INTERRUPT_COUNTER: u8 = 0;

// Global variables for storing the sensor data and timestamp
static mut TEMPERATURE: i16 = 0;
static mut TIMESTAMP: u32 = 0;

// Recommended to use AtomicBool rather than read_volatile/write_volatile -
//      https://docs.rust-embedded.org/book/c-tips/index.html#volatile-access
// Previously had issues with compiler optimizing away the read of
// SENSOR_READY since according to the compiler, the value never changes.
static mut SENSOR_READY: AtomicBool = AtomicBool::new(false);

// This is bad practice but doing it anyway since this program is so simple.
// Should really be singletons rather than just global variables.
static mut SENSOR: Option<Sensor> = None;
static mut SERIAL: Option<Serial> = None;

// TIMER1 interrupt for triggering the read from the DHT11.
#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    unsafe {
        // Need to track millis ourselves bc no default `millis()` function
        MILLIS_COUNTER += MILLIS_INCREMENT;

        if let Some(sensor) = &mut SENSOR {
            // Timer1 max interval is only ~4 seconds and we want measurements every
            // 10 seconds, so interval is 3.333 seconds and we measure only every 3rd interrupt.

            if INTERRUPT_COUNTER < 2 {
                INTERRUPT_COUNTER += 1;
            } else {
                // Take the measurement and set the SENSOR_READY flag for task code.
                match sensor.perform_measurement() {
                    Ok(Measurement {
                        temperature,
                        humidity: _,
                    }) => {
                        TEMPERATURE = temperature;  // Save the temperature reading
                        TIMESTAMP = MILLIS_COUNTER; // Get the current time in milliseconds

                        set_sensor_ready(true);
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


// NOTE: Running `cargo build` for the atmega328p WILL FAIL. Must build using `cargo build --release`.
#[arduino_hal::entry]
fn main() -> ! {
    // Initialize the Arduino peripherals.
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Get timer 1 and the default serial port.
    let tmr1: TC1 = dp.TC1;
    let mut serial: Serial = arduino_hal::default_serial!(dp, pins, 9600);

    // Configure the pin the sensor is connected to and delay to make sure it's ready.
    {
        let d11 = pins.d11.into_opendrain_high();

        unsafe {
            SENSOR = Some(Sensor {
                sensor: Dht11::new(d11),
                delay: Delay::new(),
            });
        }

        delay_ms(1000);
    }

    // Setup the timer interval to trigger interrupts.
    setup_timer(&tmr1);

    // Enable global interrupts.
    unsafe { avr_device::interrupt::enable() };

    let mut timestamp = 0u32;
    let mut temperature = 0i16;
    let mut temp_f32: f32;
    let mut temp_x100: i32;
    let mut whole: i16;
    let mut frac: i16;

    loop {
        // Check if the sensor data is ready.
        if sensor_ready() {
            // Reset the flag.
            set_sensor_ready(false);

            // Read the temperature and timestamp.
            free(|_cs| {
                timestamp = unsafe { TIMESTAMP };
                temperature = unsafe { TEMPERATURE };
            });

            // Convert to Fahrenheit.
            temp_f32 = ((((temperature as f32) / 10f32) * 1.8f32) + 32f32) * CALIBRATION_FACTOR + CALIBRATION_OFFSET;

            // Multiply by 100 to get 2 decimal places.
            temp_x100 = (temp_f32 * 100f32) as i32;

            // Get whole and fractional parts for formatting the string.
            whole = (temp_x100 / 100) as i16;
            frac = (temp_x100 % 100) as i16;

            // Send the data over the serial port in the required format - <timestamp>,<temperature>.
            let _ = ufmt::uwriteln!(&mut serial, "{},{}.{}", timestamp, whole, frac);
        }
    }
}

pub fn setup_timer(timer: &TC1) {
    // Configure Timer1 to trigger an interrupt
    const CLOCK_SOURCE: CS1_A = CS1_A::PRESCALE_1024;

    // ticks should be ~51563 for 3.3 seconds
    let ticks: u16 =
        ((INTERVAL / (PRESCALER as f32) * (arduino_hal::DefaultClock::FREQ as f32)) - 1f32) as u16;

    unsafe { 
        MILLIS_INCREMENT = PRESCALER * ticks as u32 / 16000u32;
        MILLIS_COUNTER = 0;
    };

    // Write the registers to configure the interrupt interval.
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
}

#[inline]
fn sensor_ready() -> bool {
    unsafe { SENSOR_READY.load(core::sync::atomic::Ordering::Relaxed) }
}

#[inline]
fn set_sensor_ready(ready: bool) {
    unsafe {
        SENSOR_READY.store(ready, core::sync::atomic::Ordering::Relaxed);
    }
}

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
