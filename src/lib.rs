#![cfg_attr(not(test), no_std)]

pub mod integrator;

#[derive(Clone, Debug)]
pub enum VDir {
    Positive,
    Negative,
}
