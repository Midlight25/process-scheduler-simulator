use crate::process;
use std::collections::VecDeque;

pub fn fcfs_scheduler(mut processes: VecDeque<process::Process>) {
    let mut global_clock = 0;
    let mut io_queue: VecDeque<process::Process> = VecDeque::new();
    let mut graveyard: VecDeque<process::Process> = VecDeque::new();

    while !processes.is_empty() || !io_queue.is_empty() {
        // Run process at front of queue
        match processes.pop_front() {
            Some(mut process) => {
                let process_quanta = match process.process_bursts.get(0) {
                    Some(number) => *number,
                    None => panic!("Process burst not found"),
                };

                process.run(process_quanta, global_clock);
                process.process_bursts.pop_front();

                global_clock += process_quanta;

                // Run if there is an IO burst that comes right after
                if process.process_bursts.len() > 0 {
                    process.calc_return_time(global_clock);
                    process.process_bursts.pop_front();
                    io_queue.push_back(process);
                } else {
                    graveyard.push_back(process);
                }
            }
            None => {
                global_clock += 1;
            }
        }

        // See if processes are done with IO and send them into the waiting queue.
        for _ in 0..io_queue.len() {
            let process = io_queue.pop_front().unwrap();

            if process.return_from_io_time <= global_clock {
                processes.push_back(process);
            } else {
                io_queue.push_back(process);
            }
        }
    }

    println!("Global Clock: {}", global_clock);
}
