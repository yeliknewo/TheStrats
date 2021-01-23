#![forbid(unsafe_code)]

use crossbeam::channel::{TrySendError, SendError, RecvError};
use std::{fmt, any::Any};

mod brain_thread;
mod brain_bunch;
mod brain_channel;
pub mod components;
pub mod resources;
pub mod input;
mod entities;
mod systems;

pub use brain_bunch::BrainBunch;
pub use brain_channel::{FrontToBrain, BrainToFront};

pub fn start() -> BrainBunch {
    brain_bunch::start()
}

pub enum BrainError {
    RecvError(RecvError, String),
    BoxError(Box<dyn Any + Send + 'static>, String),
    BrainError(Box<BrainError>, String),
    TrySendError(TrySendError<FrontToBrain>, String),
    SendError(SendError<BrainToFront>, String),
}

impl fmt::Display for BrainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrainError::RecvError(error, msg) => writeln!(f, "({:?} ||| {})", error, msg),
            BrainError::BoxError(error, msg) => writeln!(f, "({:?} ||| {})", error, msg),
            BrainError::TrySendError(error, msg) => writeln!(f, "({:?} ||| {})", error, msg),
            BrainError::SendError(error, msg) => writeln!(f, "({:?} ||| {})", error, msg),
            BrainError::BrainError(error, msg) => writeln!(f, "({} Called By ||| {})", error, msg),
        }
    }
}