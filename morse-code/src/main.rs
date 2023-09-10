#![no_std]
#![no_main]

use arduino_hal::{
    clock::MHz16,
    delay_ms,
    hal::{
        port::{PB5, PD0, PD1},
        Usart,
    },
    pac::USART0,
    port::{
        mode::{Input, Output},
        Pin,
    },
    prelude::_void_ResultVoidExt,
};
use embedded_hal::serial::Read;
use panic_halt as _;

const UNIT_DURATION_MS: u16 = 200;
const DOT_DURATION_MS: u16 = UNIT_DURATION_MS;
const DASH_DURATION_MS: u16 = UNIT_DURATION_MS * 3;
const DEFAULT_SPACE_MS: u16 = UNIT_DURATION_MS;
const LETTER_SPACE_MS: u16 = UNIT_DURATION_MS * 3;
const WORD_SPACE_MS: u16 = UNIT_DURATION_MS * 7;

const SENTINEL: u8 = b'!';

enum CodeBlip {
    Dot,
    Dash,
}

struct Code {
    blips: [CodeBlip; 5],
    num_blips: u8,
}

const A: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 2,
};
const B: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const C: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const D: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 3,
};
const E: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 1,
};
const F: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const G: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 3,
};
const H: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const I: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 2,
};
const J: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const K: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 3,
};
const L: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const M: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 2,
};
const N: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 2,
};
const O: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 3,
};
const P: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const Q: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const R: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 3,
};
const S: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 3,
};
const T: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 1,
};
const U: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 3,
};
const V: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const W: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 3,
};
const X: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const Y: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};
const Z: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 4,
};

const ONE: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
    ],
    num_blips: 5,
};
const TWO: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
    ],
    num_blips: 5,
};
const THREE: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dash,
        CodeBlip::Dash,
    ],
    num_blips: 5,
};
const FOUR: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dash,
    ],
    num_blips: 5,
};
const FIVE: Code = Code {
    blips: [
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 5,
};
const SIX: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 5,
};
const SEVEN: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 5,
};
const EIGHT: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
        CodeBlip::Dot,
    ],
    num_blips: 5,
};
const NINE: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dot,
    ],
    num_blips: 5,
};
const ZERO: Code = Code {
    blips: [
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
        CodeBlip::Dash,
    ],
    num_blips: 5,
};

const CODES: [&Code; 36] = [
    &A, &B, &C, &D, &E, &F, &G, &H, &I, &J, &K, &L, &M, &N, &O, &P, &Q, &R, &S, &T, &U, &V, &W, &X,
    &Y, &Z, &ZERO, &ONE, &TWO, &THREE, &FOUR, &FIVE, &SIX, &SEVEN, &EIGHT, &NINE,
];
type Led = Pin<Output, PB5>;
type Serial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>;

fn dot(led: &mut Led, serial: &mut Serial) {
    ufmt::uwriteln!(serial, "DOT").void_unwrap();
    led.set_high();
    delay_ms(DOT_DURATION_MS);
}

fn dash(led: &mut Led, serial: &mut Serial) {
    ufmt::uwriteln!(serial, "DASH").void_unwrap();
    led.set_high();
    delay_ms(DASH_DURATION_MS);
}

fn default_space(led: &mut Led) {
    led.set_low();
    delay_ms(DEFAULT_SPACE_MS);
}

fn letter_space(led: &mut Led, serial: &mut Serial) {
    ufmt::uwriteln!(serial, "LETTER SPACE").void_unwrap();
    led.set_low();
    delay_ms(LETTER_SPACE_MS);
}

fn word_space(led: &mut Led, serial: &mut Serial) {
    ufmt::uwriteln!(serial, "WORD SPACE").void_unwrap();
    led.set_low();
    delay_ms(WORD_SPACE_MS);
}

fn next_letter(
    char_index: usize,
    input_len: usize,
    user_input: &[u8; 128]
) -> Option<&'static Code> {
    if char_index >= input_len {
        return None;
    }

    let c: u8 = user_input[char_index];

    // Lower Case
    if c > 96 && c < 123 {
        return Some(CODES[(c - 97) as usize]);
    }
    // Upper Case
    else if c > 64 && c < 91 {
        return Some(CODES[(c - 65) as usize]);
    }
    // Numbers
    else if c > 47 && c < 58 {
        return Some(CODES[(c - 22) as usize]);
    }
    
    None
}

fn do_blink(
    led: &mut Led,
    char_index: &mut usize,
    code_index: &mut usize,
    input_len: usize,
    user_input: &[u8; 128],
    current_code: &mut Option<&Code>,
    serial: &mut Serial,
) {
    if *char_index >= input_len {
        *char_index = 0;
        *code_index = 0;
        *current_code = next_letter(*char_index, input_len, user_input);

        ufmt::uwriteln!(serial, "\n--- Resetting ---\n").void_unwrap();

        word_space(led, serial);
        return;
    }

    if let Some(code) = *current_code {
        match code.blips[*code_index] {
            CodeBlip::Dot => dot(led, serial),
            CodeBlip::Dash => dash(led, serial),
        }

        *code_index += 1;

        if *code_index >= code.num_blips as usize {
            *char_index += 1;
            *code_index = 0;
            *current_code = next_letter(*char_index, input_len, user_input);

            if (*current_code).is_none() {
                word_space(led, serial);
            } else {
                letter_space(led, serial);
            }
        } else {
            default_space(led);
        }
    } else {
        *char_index += 1;
        *code_index = 0;
        *current_code = next_letter(*char_index, input_len, user_input);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let mut user_input: [u8; 128] = [0; 128];
    let mut input_len: usize = 0;
    let mut blinking = false;

    let mut char_index: usize = 0;
    let mut code_index: usize = 0;
    let mut current_code: Option<&Code> = None;

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {

        if let Ok(b) = serial.read() {
            if b == SENTINEL {
                input_len = 0;
                blinking = false;
                ufmt::uwriteln!(&mut serial, "Received sentinel").void_unwrap();
                continue;
            }

            if b == b'\n' || b == b'\r' {
                continue;
            }

            if input_len < 128 {
                user_input[input_len] = b;
                input_len += 1;
            }
            if !blinking {
                blinking = true;
            }

            char_index = 0;
            code_index = 0;
            current_code = next_letter(char_index, input_len, &user_input);

            loop {
                match nb::block!(serial.read()) {
                    Ok(b) => {
                        if b == SENTINEL {
                            input_len = 0;
                            blinking = false;
                            break;
                        }

                        if b == b'\n' || b == b'\r' {
                            break;
                        }

                        if input_len < 128 {
                            user_input[input_len] = b;
                            input_len += 1;
                        }
                    }
                    Err(_) => break,
                }
            }
        }

        if blinking {
            do_blink(
                &mut led,
                &mut char_index,
                &mut code_index,
                input_len,
                &user_input,
                &mut current_code,
                &mut serial,
            );
        }
    }
}
