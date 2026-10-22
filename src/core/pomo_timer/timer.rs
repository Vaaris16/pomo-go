use std::time::Instant;

use crate::{FOCUS_STATE_TIME, PomoState, PomoTimerState, REST_STATE_TIME};

pub struct Timer {
    remaining_time: f64,
    last_time: Instant,
    previous_pomostate: PomoState,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            remaining_time: REST_STATE_TIME,
            last_time: Instant::now(),
            previous_pomostate: PomoState::Rest,
        }
    }

    pub fn set_timer(&mut self, pomo_state: PomoState) {
        if self.previous_pomostate != pomo_state {
            self.remaining_time = match pomo_state {
                PomoState::Focus => FOCUS_STATE_TIME,
                PomoState::Rest => REST_STATE_TIME,
            };

            self.previous_pomostate = pomo_state;
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
