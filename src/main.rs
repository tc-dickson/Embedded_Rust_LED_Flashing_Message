#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit;

#[entry]
fn main() -> ! {
    loop{}
}
