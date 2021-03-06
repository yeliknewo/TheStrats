use crossbeam::channel::{Sender, Receiver, unbounded};

use super::{BrainError, input::KeyCode};

pub fn new_wombo() -> (FrontChannel, BrainChannel) {
    let (send_to_brain, recv_from_front) = unbounded();
    let (send_to_front, recv_from_brain) = unbounded();
    (FrontChannel{
        recv_from_brain,
        send_to_brain,
    }, BrainChannel {
        recv_from_front,
        send_to_front,
    })
}

pub struct BrainChannel {
    recv_from_front: Receiver<FrontToBrain>,
    send_to_front: Sender<BrainToFront>,
}

impl BrainChannel {
    pub fn get_event(&mut self) -> Result<FrontToBrain, BrainError> {
        self.recv_from_front.recv().map_err(|error| BrainError::RecvError(error, stack!()))
    }

    pub fn try_drain(&mut self) -> Vec<FrontToBrain> {
        self.recv_from_front.try_iter().collect()
    }

    pub fn send_event(&mut self, event: BrainToFront) -> Result<(), BrainError> {
        self.send_to_front.send(event).map_err(|error| BrainError::SendError(error, stack!()))
    }
}

pub struct FrontChannel {
    recv_from_brain: Receiver<BrainToFront>,
    send_to_brain: Sender<FrontToBrain>,
}

impl FrontChannel {
    pub fn try_drain(&mut self) -> Vec<BrainToFront> {
        self.recv_from_brain.try_iter().collect()
    }

    pub fn send(&mut self, event: FrontToBrain) -> Result<(), BrainError> {
        self.send_to_brain.try_send(event).map_err(|error| BrainError::TrySendError(error, stack!()))
    }
}

pub enum FrontToBrain {
    Init,
    Exit,
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    Delta(f64),
}

pub enum BrainToFront {
    Log(String),
}