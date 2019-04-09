use std::time::{Duration, Instant};
use rodio::source::{self, Source};

pub struct SoundTimer {
    sink: rodio::Sink
}

impl SoundTimer {
    pub fn new() -> SoundTimer {
        SoundTimer {
            sink: rodio::Sink::new(&rodio::default_output_device().unwrap())
        }
    }

    pub fn set(&mut self, time: u64) {
        self.sink.stop();
        self.sink.append(source::SineWave::new(280).amplify(0.25).take_duration(Duration::from_millis(hz_to_millis(time))));
        self.sink.play();
    }
}

pub struct DelayTimer {
    duration: Duration,
    initial: Instant,
}

impl DelayTimer {
    pub fn new() -> DelayTimer {
        DelayTimer {
            duration: Duration::from_secs(0),
            initial: Instant::now(),
        }
    }

    pub fn set(&mut self, time: u64) {
        self.duration = Duration::from_millis(hz_to_millis(time));
        self.initial = Instant::now();
    }

    pub fn get(&self) -> u64 {
        if let Some(elapsed_time) = self
            .duration
            .checked_sub(Instant::now().duration_since(self.initial))
        {
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
