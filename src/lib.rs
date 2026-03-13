#![cfg_attr(not(test), no_std)]

pub mod integrator;

#[derive(Clone, Debug)]
pub enum VDir {
    Positive,
    Negative,
}

pub struct LedDisplayDirection(pub VDir);

#[must_use]
pub fn edge_detector(past_vel: &Option<VDir>, current_vel: &Option<VDir>) -> Option<VDir> {
    match (past_vel, current_vel) {
        (Some(VDir::Positive), Some(VDir::Negative)) => { Some(VDir::Negative)}
        (Some(VDir::Negative), Some(VDir::Positive)) => { Some(VDir::Positive)}
        _ => {None}
    }
}
