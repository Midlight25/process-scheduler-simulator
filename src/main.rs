mod process;
mod scheduler;

use std::collections::VecDeque;

fn main() {
    let process = process::Process {
        process_bursts: VecDeque::from(vec![6, 2, 1]),
        total_process_time: 6 + 2 + 1,
        name: "P3".to_string(),
        ..Default::default()
    };

    let process_2 = process::Process {
        process_bursts: VecDeque::from(vec![7, 3, 1, 2, 6]),
        total_process_time: 6 + 2 + 1,
        name: "P5".to_string(),
        ..Default::default()
    };

    let process_3 = process::Process {
        process_bursts: VecDeque::from(vec![5, 3, 4]),
        total_process_time: 6 + 2 + 1,
        name: "P3".to_string(),
        ..Default::default()
    };

    let processes = VecDeque::from(vec![process, process_2, process_3]);

    scheduler::fcfs_scheduler(processes);
}
