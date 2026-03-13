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

use led_flashing_message_lib::integrator;
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

    let mut integrator = integrator::Integrator::<i32, 32>::new(0);

    loop {
        let _ = integrator.insert(accel.acceleration().unwrap().x_mg());
    }
}
