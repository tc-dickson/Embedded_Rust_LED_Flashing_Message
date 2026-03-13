#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use microbit::hal::twim;
use microbit::{
    board,
    hal::{Timer, twim::Frequency},
};
use panic_rtt_target as _;
// use rtt_target::{rprintln, rtt_init_print};

use led_flashing_message_lib::{LedDisplayDirection, VDir, integrator};

const NEGATIVE_ACCEL_THRESHOLD: i32 = -5;
const POSITIVE_ACCEL_THRESHOLD: i32 = 5;

// const LED_MESSAGE_LEN: usize = 35;
//
// #[rustfmt::skip]
// const LED_MESSAGE: [[i32; LED_MESSAGE_LEN]; 5] = [
//     [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 1, 1, 1, 0,   0, 1, 0, 1, 0,   0, 1, 0, 1, 0,   0, 0, 0, 0, 0,   0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 0, 1, 0, 0,   1, 0, 1, 0, 1,   0, 1, 0, 1, 0,   0, 0, 0, 0, 0,   0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 0, 1, 0, 0,   1, 0, 0, 0, 1,   0, 1, 0, 1, 0,   0, 0, 0, 0, 0,   0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 0, 1, 0, 0,   0, 1, 0, 1, 0,   0, 1, 0, 1, 0,   0, 0, 0, 0, 0,   0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 1, 1, 1, 0,   0, 0, 1, 0, 0,   0, 1, 1, 1, 0,   0, 0, 0, 0, 0,   0, 0, 0, 0, 0],
// ]; // A message for my wife

const LED_MESSAGE_LEN: usize = 40;

#[rustfmt::skip]
const LED_MESSAGE: [[i32; LED_MESSAGE_LEN]; 5] = [
    [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 1, 1, 1, 0,   0, 1, 0, 1, 0,   0, 1, 1, 1, 0,   0, 1, 1, 1, 0,   0, 0, 0, 0, 0,    0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 1, 0, 1, 0,   0, 1, 0, 1, 0,   0, 1, 0, 0, 0,   0, 0, 1, 0, 0,   0, 0, 0, 0, 0,    0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 1, 1, 0, 0,   0, 1, 0, 1, 0,   0, 1, 1, 1, 0,   0, 0, 1, 0, 0,   0, 0, 0, 0, 0,    0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 1, 0, 1, 0,   0, 1, 0, 1, 0,   0, 0, 0, 1, 0,   0, 0, 1, 0, 0,   0, 0, 0, 0, 0,    0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,   0, 1, 0, 1, 0,   0, 1, 1, 1, 0,   0, 1, 1, 1, 0,   0, 0, 1, 0, 0,   0, 0, 0, 0, 0,    0, 0, 0, 0, 0],
];

#[entry]
fn main() -> ! {
    // rtt_init_print!();
    let mut board = board::Board::take().unwrap();

    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), Frequency::K100);

    let mut timer = Timer::new(board.TIMER0);
    let mut accel = Lsm303agr::new_with_i2c(i2c);

    accel.init().unwrap();
    accel
        .set_accel_mode_and_odr(&mut timer, AccelMode::Normal, AccelOutputDataRate::Hz100)
        .unwrap();
    let _ = accel.set_accel_scale(lsm303agr::AccelScale::G16);

    let mut timer = Timer::new(board.TIMER1);
    let mut current_time = 0;

    let mut integrator = integrator::Integrator::<i32, 32>::new(0);
    let mut reading;

    let mut past_vdir: Option<VDir> = None;
    let mut current_vdir: Option<VDir> = None;

    let mut led_display_direction: Option<LedDisplayDirection> = None;
    let mut led_column_lit_time: u32;
    let mut current_led_column = 0;

    // There has to be a better way of doing this
    let _ = board.display_pins.col3.set_low();
    let mut led_0 = board.display_pins.row5;
    let mut led_1 = board.display_pins.row4;
    let mut led_2 = board.display_pins.row3;
    let mut led_3 = board.display_pins.row2;
    let mut led_4 = board.display_pins.row1;

    loop {
        let _ = integrator.insert(accel.acceleration().unwrap().x_mg());
        reading = integrator.read();

        match reading {
            NEGATIVE_ACCEL_THRESHOLD..POSITIVE_ACCEL_THRESHOLD => (),
            i32::MIN..NEGATIVE_ACCEL_THRESHOLD => {
                current_vdir = Some(VDir::Negative);
                // rprintln!("Negative!");
            }
            POSITIVE_ACCEL_THRESHOLD..=i32::MAX => {
                current_vdir = Some(VDir::Positive);
                // rprintln!("Positive!");
            }
        }

        if let Some(edge) = led_flashing_message_lib::edge_detector(&past_vdir, &current_vdir) {
            // Read the current value of the timer from the last loop before restarting it
            current_time = timer.read();
            timer.start(u32::MAX);

            match edge {
                VDir::Positive => {
                    // rprintln!("Positive Edge!");
                    led_display_direction = Some(LedDisplayDirection(
                        VDir::Positive,
                    ));
                }
                VDir::Negative => {
                    // rprintln!("Negative Edge!");
                    led_display_direction = Some(LedDisplayDirection(
                        VDir::Negative,
                    ));
                }
            }
        }

        // Safe to unwrap because LED_MESSAGE_LEN is a constant within the bounds of a u32
        led_column_lit_time = current_time / u32::try_from(LED_MESSAGE_LEN).unwrap();

        // Prevent divide by 0
        if led_column_lit_time == 0 {
            continue;
        }

        match led_display_direction {
            Some(LedDisplayDirection(VDir::Positive)) => {
                current_led_column = (timer.read() / led_column_lit_time) as usize;
            }
            Some(LedDisplayDirection(VDir::Negative)) => {
                current_led_column =
                    LED_MESSAGE_LEN.saturating_sub((timer.read() / led_column_lit_time) as usize);
            }
            None => (),
        }

        let i32_to_pin_state = |x: i32| match x {
            1 => PinState::High,
            _ => PinState::Low,
        };

        // Bounds check to ensure current_led_column is always a valid variable
        if current_led_column > 0 && current_led_column < LED_MESSAGE_LEN {
            led_0.set_state(i32_to_pin_state(LED_MESSAGE[0][current_led_column]));
            led_1.set_state(i32_to_pin_state(LED_MESSAGE[1][current_led_column]));
            led_2.set_state(i32_to_pin_state(LED_MESSAGE[2][current_led_column]));
            led_3.set_state(i32_to_pin_state(LED_MESSAGE[3][current_led_column]));
            led_4.set_state(i32_to_pin_state(LED_MESSAGE[4][current_led_column]));
        } else {
            led_0.set_state(PinState::Low);
            led_1.set_state(PinState::Low);
            led_2.set_state(PinState::Low);
            led_3.set_state(PinState::Low);
            led_4.set_state(PinState::Low);
        }

        past_vdir.clone_from(&current_vdir);
    }
}
