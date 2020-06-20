#![forbid(unsafe_code)]

use crossbeam::channel::{TrySendError, RecvError};
use std::{fmt, any::Any};

mod brain_thread;
mod brain_bunch;
mod brain_channel;
mod components;

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
}

impl fmt::Display for BrainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrainError::RecvError(error, msg) => write!(f, "({:?} ||| {})", error, msg),
            BrainError::BoxError(error, msg) => write!(f, "({:?} ||| {})", error, msg),
            BrainError::TrySendError(error, msg) => write!(f, "({:?} ||| {})", error, msg),
            BrainError::BrainError(error, msg) => write!(f, "({} ||| {})", error, msg),
        }
    }
}