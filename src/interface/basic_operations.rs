use std::thread::sleep;
use std::time::Duration;

pub fn clearTerminal() {
    clearscreen::clear().expect("Terminal cleaning error...");
}

pub fn wait(seconds: u64) {
    sleep(Duration::from_secs(seconds));
}
