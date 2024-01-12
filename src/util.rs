use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub fn waiting_time_to_sync() {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let millis = since_the_epoch.as_millis();
    let millis_until_next_second = 1000 - (millis % 1000);
    sleep(Duration::from_millis(millis_until_next_second as u64));
}
