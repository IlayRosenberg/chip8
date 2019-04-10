use rodio::source::{self, Source};
use std::time::{Duration, Instant};

pub struct SoundTimer {
    output_device: rodio::Device,
    current_sound: Option<rodio::Sink>,
}

impl SoundTimer {
    pub fn new() -> SoundTimer {
        SoundTimer {
            output_device: rodio::default_output_device().unwrap(),
            current_sound: None
        }
    }

    pub fn set(&mut self, time: u64) {
        let source = source::SineWave::new(280u32)
                .amplify(0.25)
                .repeat_infinite()
                .take_duration(Duration::from_millis(hz_to_millis(time as f64) as u64));

        let sink = self.current_sound.get_or_insert(rodio::Sink::new(&self.output_device));
        if !sink.empty() {
            *sink = rodio::Sink::new(&self.output_device);
        }
        sink.append(source);
        sink.play();
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
        self.duration = Duration::from_millis(hz_to_millis(time as f64) as u64);
        self.initial = Instant::now();
    }

    pub fn get(&self) -> u64 {
        if let Some(elapsed_time) = self
            .duration
            .checked_sub(Instant::now().duration_since(self.initial))
        {
            millis_to_hz(elapsed_time.as_micros() as f64) as u64
        } else {
            0
        }
    }
}

fn hz_to_millis(hz: f64) -> f64 {
    hz / 60.0 * 1000.0
}

fn millis_to_hz(hz: f64) -> f64 {
    hz / 1000.0 * 60.0
}
