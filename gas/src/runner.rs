use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

use crate::callback;

pub fn start(f: fn()) {
    let cb = callback::Processor{ callback: f };
    let mut sched = JobScheduler::new();

    sched.add(Job::new("* * * * * *".parse().unwrap(), || {
        cb.process_events();
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}