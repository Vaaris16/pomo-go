use std::time::Instant;

use crate::PomoTimerState;

pub struct Timer {
    remaining_time: f64,
    last_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            remaining_time: 25. * 60.,
            last_time: Instant::now(),
        }
    }

    pub fn timer(&mut self, is_running: &PomoTimerState) -> (u64, u64) {
        match is_running {
            PomoTimerState::Running => {
                let now = Instant::now();
                let elapsed = now.duration_since(self.last_time).as_secs_f64();

                self.remaining_time -= elapsed;
                self.last_time = now;
            }
            PomoTimerState::Paused => {
                self.last_time = Instant::now();
            }
            _ => (),
        }
        let minutes = self.remaining_time as u64 / 60;
        let seconds = self.remaining_time as u64 % 60;

        (minutes, seconds)
    }
}
