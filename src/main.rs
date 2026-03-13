#![no_std]
#![no_main]

use cortex_m_rt::entry;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use microbit::hal::twim;
use microbit::{
    board,
    hal::{Timer, twim::Frequency},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use led_flashing_message_lib::{LedDisplayDirection, VDir, integrator};

const NEGATIVE_ACCEL_THRESHOLD: i32 = -5;
const POSITIVE_ACCEL_THRESHOLD: i32 = 5;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = board::Board::take().unwrap();

    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), Frequency::K100);

    let mut timer = Timer::new(board.TIMER0);
    let mut accel = Lsm303agr::new_with_i2c(i2c);

    accel.init().unwrap();
    accel
        .set_accel_mode_and_odr(&mut timer, AccelMode::Normal, AccelOutputDataRate::Hz100)
        .unwrap();
    let _ = accel.set_accel_scale(lsm303agr::AccelScale::G16);

    let mut integrator = integrator::Integrator::<i32, 32>::new(0);
    let mut reading;

    let mut past_vdir: Option<VDir> = None;
    let mut current_vdir: Option<VDir> = None;

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
            match edge {
                VDir::Positive => {
                    rprintln!("Positive Edge!");
                    led_display_direction = Some(LedDisplayDirection(
                        VDir::Positive,
                    ));
                }
                VDir::Negative => {
                    rprintln!("Negative Edge!");
                    led_display_direction = Some(LedDisplayDirection(
                        VDir::Negative,
                    ));
                }
            }
        }

        past_vdir.clone_from(&current_vdir);
    }
}
