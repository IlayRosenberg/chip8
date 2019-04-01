use std::time::{Duration, Instant};

pub struct Timer {
    duration: Duration,
    initial: Instant
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            duration: Duration::from_secs(0),
            initial: Instant::now()
        }
    }    

    pub fn set(&mut self, time: u64) {
        self.duration = Duration::from_millis(hz_to_millis(time));
        self.initial = Instant::now();
    }

    pub fn get(&self) -> u64 {
        if let Some(elapsed_time) = self.duration.checked_sub(Instant::now().duration_since(self.initial)) {
            millis_to_hz(elapsed_time.as_millis() as u64)
        } else {
            0
        }
    }
}

fn hz_to_millis(hz: u64) -> u64 {
    hz * 60 / 1000
}

fn millis_to_hz(hz: u64) -> u64 {
    hz * 1000 / 60
}