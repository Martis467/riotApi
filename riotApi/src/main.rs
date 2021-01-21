extern crate chrono;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{Scheduler, TimeUnits};
// Import week days and WeekDay
use clokwerk::Interval::*;
use std::thread;
use std::time::Duration;

mod responseModels;

fn main() {
    let puuid = String::from("jduVd_VZnTFQ5TOU_2EuEo8kqP5WmlymN9RQcLxQdkrMUkynR7l-QDGWHLEJvtDd048fLeB21WUMWw");
    let id = String::from("lc3ZgX5R_JNyrkChhcpP80n1KbNzkVz6I2p5kN-23z2t");

    // Create a new scheduler
    let mut scheduler = Scheduler::new();
    // or a scheduler with a given timezone
    let mut scheduler = Scheduler::with_tz(chrono::Utc);
    scheduler.every(10.seconds()).run(|| println!("Running task"));

    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    thread::sleep(Duration::from_millis(60000))
}
