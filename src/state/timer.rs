use std::time::SystemTime;

use chrono::{DateTime, Local, Timelike};

#[derive(Copy, Clone)]
pub struct Timer {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hours(mut self, h: u8) -> Self {
        self.hours = h;
        self
    }

    pub fn minutes(mut self, m: u8) -> Self {
        self.minutes = m;
        self
    }

    pub fn seconds(mut self, s: u8) -> Self {
        self.seconds = s;
        self
    }
}

impl Default for Timer {
    fn default() -> Self {
        let system_time = SystemTime::now();
        let datetime: DateTime<Local> = system_time.into();
        Self {
            hours: datetime.hour() as u8,
            minutes: datetime.minute() as u8,
            seconds: datetime.second() as u8,
        }
    }
}

