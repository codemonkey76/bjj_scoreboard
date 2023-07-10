use std::time::{Duration, SystemTime};

#[derive(Default, Debug)]
pub struct MatchTime {
    pub duration_millis: usize,
    pub last_started: Option<SystemTime>,
    pub time_elapsed_millis: usize,
    pub running: bool
}

impl MatchTime {
    pub fn get_remaining_time_string(&self) -> String {
        let millis = self.get_remaining_time_milliseconds();
        let hours = millis / 3_600_000;
        let minutes = (millis % 3_600_000) / 60_000;
        let seconds = (millis % 60_000) / 1_000;
        let milliseconds = millis % 1_000;

        if hours > 0 {
            format!("{:01}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds)
        } else {
            format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
        }
    }

    pub fn get_remaining_time_milliseconds(&self) -> usize {
        let elapsed = match &self.last_started {
            Some(start_time) => {
                match self.running {
                    true => {
                        self.time_elapsed_millis + SystemTime::now().duration_since(*start_time).unwrap_or(Duration::new(0, 0)).as_millis() as usize
                    },
                    false => {
                        self.time_elapsed_millis
                    }
                }
            },
            None => {
                self.time_elapsed_millis
            }
        };

        self.duration_millis.saturating_sub(elapsed)
    }

    pub fn toggle_start_stop(&mut self) {
        if self.running {
            self.stop();
        } else {
            self.start();
        }
    }

    pub fn start(&mut self) {
        if self.running {
            return;
        }

        self.running = true;
        self.last_started = Some(SystemTime::now());
    }

    pub fn stop(&mut self) {
        if !self.running {
            return;
        }

        let elapsed = match &self.last_started {
            Some(start_time) => {
                SystemTime::now().duration_since(*start_time).unwrap_or(Duration::new(0,0)).as_millis() as usize
            },
            None => 0
        };

        self.running = false;
        self.time_elapsed_millis += elapsed;
    }
}