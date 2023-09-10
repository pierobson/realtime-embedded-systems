#![no_std]
#![no_main]

use embedded_hal::serial::Read;
use panic_halt as _;
use arduino_hal::{prelude::_void_ResultVoidExt, hal::{Atmega, port::{PD0, PD1}}, pac::USART0, port::{Pin, mode::{Input, Output}}};
use ufmt::uWrite;
use core::str::from_utf8;


const UNIT_DURATION_MS: u16 = 300;
const DOT_DURATION_MS: u16 = UNIT_DURATION_MS;
const DASH_DURATION_MS: u16 = UNIT_DURATION_MS * 3;
const DEFAULT_SPACE_MS: u16 = UNIT_DURATION_MS;
const LETTER_SPACE_MS: u16 = UNIT_DURATION_MS * 3;
const WORD_SPACE_MS: u16 = UNIT_DURATION_MS * 7;

const SENTINAL: char = '!';

enum Code {
    Dot,
    Dash,
}

const A: [Code; 2] = [ Code::Dot, Code::Dash ];                         // .-
const B: [Code; 4] = [ Code::Dash, Code::Dot, Code::Dot, Code::Dot ];   // -...
const C: [Code; 4] = [ Code::Dash, Code::Dot, Code::Dash, Code::Dot ];  // -.-.
const D: [Code; 3] = [ Code::Dash, Code::Dot, Code::Dot ];              // -..
const E: [Code; 1] = [ Code::Dot ];                                     // .
const F: [Code; 4] = [ Code::Dot, Code::Dot, Code::Dash, Code::Dot ];   // ..-.
const G: [Code; 3] = [ Code::Dash, Code::Dash, Code::Dot ];             // --.
const H: [Code; 4] = [ Code::Dot, Code::Dot, Code::Dot, Code::Dot ];    // ....
const I: [Code; 2] = [ Code::Dot, Code::Dot ];                          // ..
const J: [Code; 4] = [ Code::Dot, Code::Dash, Code::Dash, Code::Dash ]; // .---
const K: [Code; 3] = [ Code::Dash, Code::Dot, Code::Dash ];             // -.-
const L: [Code; 4] = [ Code::Dot, Code::Dash, Code::Dot, Code::Dot ];   // .-..
const M: [Code; 2] = [ Code::Dash, Code::Dash ];                        // --
const N: [Code; 2] = [ Code::Dash, Code::Dot ];                         // -.
const O: [Code; 3] = [ Code::Dash, Code::Dash, Code::Dash ];            // ---
const P: [Code; 4] = [ Code::Dot, Code::Dash, Code::Dash, Code::Dot ];  // .--.
const Q: [Code; 4] = [ Code::Dash, Code::Dash, Code::Dot, Code::Dash ]; // --.-
const R: [Code; 3] = [ Code::Dot, Code::Dash, Code::Dot ];              // .-.
const S: [Code; 3] = [ Code::Dot, Code::Dot, Code::Dot ];               // ...
const T: [Code; 1] = [ Code::Dash ];                                    // -
const U: [Code; 3] = [ Code::Dot, Code::Dot, Code::Dash ];              // ..-
const V: [Code; 4] = [ Code::Dot, Code::Dot, Code::Dot, Code::Dash ];   // ...- 
const W: [Code; 3] = [ Code::Dot, Code::Dash, Code::Dash ];             // .--
const X: [Code; 4] = [ Code::Dash, Code::Dot, Code::Dot, Code::Dash ];  // -..-
const Y: [Code; 4] = [ Code::Dash, Code::Dot, Code::Dash, Code::Dash ]; // -.--
const Z: [Code; 4] = [ Code::Dash, Code::Dash, Code::Dot, Code::Dot ];  // --..

const ONE: [Code; 5] = [ Code::Dot, Code::Dash, Code::Dash, Code::Dash, Code::Dash ];   // .----
const TWO: [Code; 5] = [ Code::Dot, Code::Dot, Code::Dash, Code::Dash, Code::Dash ];   // ..---
const THREE: [Code; 5] = [ Code::Dot, Code::Dot, Code::Dot, Code::Dash, Code::Dash ];   // ...--
const FOUR: [Code; 5] = [ Code::Dot, Code::Dot, Code::Dot, Code::Dot, Code::Dash ];   // ....-
const FIVE: [Code; 5] = [ Code::Dot, Code::Dot, Code::Dot, Code::Dot, Code::Dot ];   // .....
const SIX: [Code; 5] = [ Code::Dash, Code::Dot, Code::Dot, Code::Dot, Code::Dot ];   // -....
const SEVEN: [Code; 5] = [ Code::Dash, Code::Dash, Code::Dot, Code::Dot, Code::Dot ];   // --...
const EIGHT: [Code; 5] = [ Code::Dash, Code::Dash, Code::Dash, Code::Dot, Code::Dot ];   // ---..
const NINE: [Code; 5] = [ Code::Dash, Code::Dash, Code::Dash, Code::Dash, Code::Dot ];   // ----.
const ZERO: [Code; 5] = [ Code::Dash, Code::Dash, Code::Dash, Code::Dash, Code::Dash ]; // -----


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 2400);

    let mut user_input: [u8; 128] = [0; 128];
    let mut input_len: usize = 0;
    let mut char_index: u16 = 0;
    let mut code_index: u8 = 0;
    let mut did_read = false;

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
            match serial.read() {
                Ok(b) => {
                    if input_len < 128 {
                        user_input[input_len] = b;
                        input_len += 1;
                    }
                    if !did_read {
                        did_read = true;
                    }
                },
                Err(_) => (),
            }

        if did_read {
            if input_len > 0 {
                ufmt::uwriteln!(&mut serial, "{}", core::str::from_utf8(&user_input[0..input_len]).unwrap()).void_unwrap();
            }
        }
    }
}
