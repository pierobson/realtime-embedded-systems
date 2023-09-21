#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

use core::cell;
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
    Delay
};
use avr_device::{
    atmega328p::{tc1::tccr1b::CS1_A, TC1},
    interrupt,
};
use dht_sensor::*;
use embedded_hal::blocking::delay::DelayMs;
use heapless::String;

type Serial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>;

const PRESCALER: u32 = 256;
const TIMER_COUNTS: u32 = 250;

const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

// Global variables for storing the sensor data and timestamp
static mut SENSOR_READY: bool = false;
static mut TEMPERATURE: f32 = 0.0;
static mut TIMESTAMP: u32 = 0;

static mut INTERRUPT_COUNTER: u8 = 0;

// Define a timer interrupt handler
#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    unsafe {
        // Initialize the Arduino Uno peripherals
        let dp = arduino_hal::Peripherals::steal();
        let pins = arduino_hal::pins!(dp);
        let mut delay = arduino_hal::Delay::new();

        // Initialize the DHT11 sensor on a digital pin
        let mut sensor_pin = pins.d2.into_opendrain_high();

        // Read data from the sensor every third interrupt
        if INTERRUPT_COUNTER < 2 {
            INTERRUPT_COUNTER += 1;
        } else {

            match dht11::Reading::read(&mut delay, &mut sensor_pin) {
                Ok(dht11::Reading {
                    temperature,
                    relative_humidity: _
                }) => {
                    TEMPERATURE = temperature as f32 * 1.8 + 32.0; // Convert to Fahrenheit
                    TIMESTAMP = millis(); // Get the current time in milliseconds

                    SENSOR_READY = true;
                    INTERRUPT_COUNTER = 0; // Reset the counter
                },
                Err(_) => (),
            }
        }
    }
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

fn millis_init(tc0: arduino_hal::pac::TC0) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(TIMER_COUNTS as u8));
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

#[arduino_hal::entry]
fn main() -> ! {
    // Initialize the Arduino Uno peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let tmr1: TC1 = dp.TC1;
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    millis_init(dp.TC0);
    setup_timer(&tmr1);

    // Enable global interrupts
    unsafe { avr_device::interrupt::enable() };

    loop {
        // Check if the sensor data is ready
        if unsafe { SENSOR_READY } {
            let temperature = unsafe { TEMPERATURE };
            let timestamp = unsafe { TIMESTAMP };

            // Send the data over the serial port in the required format
            let mut format_str = String::<20>::new();
            if core::fmt::write(&mut format_str, format_args!("{},{:.2}", timestamp, temperature)).is_ok() {
                // Successfully formatted, now write it to the serial port
                let _ = ufmt::uwriteln!(serial, "{}", format_str.as_str());
            }
            // Reset the flag
            unsafe {
                SENSOR_READY = false;
            }
        }
    }
}

pub const fn calc_overflow(clock_hz: u32, target_hz: u32, prescale: u32) -> u32 {
    /*
    https://github.com/Rahix/avr-hal/issues/75
    reversing the formula F = 16 MHz / (256 * (1 + 15624)) = 4 Hz
     */
    clock_hz / target_hz / prescale - 1
}

pub fn setup_timer(timer: &TC1) {
    // Configure Timer1 to trigger an interrupt every 3.3 seconds
    const CLOCK_FREQ_HZ: u32 = arduino_hal::DefaultClock::FREQ;
    const CLOCK_SOURCE: CS1_A = CS1_A::PRESCALE_1024;
    let clock_divisor = 1024;

    let ticks = calc_overflow(CLOCK_FREQ_HZ, 1, clock_divisor) as u16;

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
