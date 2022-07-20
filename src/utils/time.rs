use std::thread;
use std::time::{Duration, Instant};

pub struct TimeFrame {
    pub frame_duration: Duration,
    pub last_time: Instant,
    pub target_time: Instant,
}

impl TimeFrame {
    pub fn new(frame_duration: Duration) -> TimeFrame {
        let now = Instant::now();
        TimeFrame {
            frame_duration,
            last_time: now,
            target_time: now + frame_duration,
        }
    }
    pub fn update(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now - self.last_time;
        self.last_time = now;
        self.target_time += self.frame_duration;
        delta
    }
    pub fn wait(&self) {
        let now = Instant::now();
        if now < self.target_time {
            thread::sleep(self.target_time - now);
        }
    }
}
