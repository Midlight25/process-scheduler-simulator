use crate::process;
use sorts::quick_sort::quick_sort;
use std::collections::VecDeque;
use tabular::{Row, Table};

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
            let mut process = io_queue.pop_front().unwrap();

            if process.return_from_io_time <= global_clock {
                process.last_accessed = global_clock;
                processes.push_back(process);
            } else {
                io_queue.push_back(process);
            }
        }
    }

    println!("Global Clock: {}", global_clock);
    print_processes(graveyard);
}

pub fn sjf_scheduler(mut processes: VecDeque<process::Process>) {
    let mut global_clock = 0;
    let mut io_queue: VecDeque<process::Process> = VecDeque::new();
    let mut graveyard: VecDeque<process::Process> = VecDeque::new();

    while !processes.is_empty() || !io_queue.is_empty() {
        // Sort waiting queue if there are processes there
        if processes.len() > 0 {
            quick_sort(&mut processes.make_contiguous());
        }

        // Run process at front of queue if there is one.
        match processes.pop_front() {
            Some(mut process) => {
                let process_quanta = match process.process_bursts.get(0) {
                    Some(number) => *number,
                    None => panic!("Process burst not found"),
                };

                process.run(process_quanta, global_clock);
                process.process_bursts.pop_front();

                global_clock += process_quanta;

                // Place into IO queue if there is an IO burst that comes right after
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
            let mut process = io_queue.pop_front().unwrap();

            if process.return_from_io_time <= global_clock {
                process.last_accessed = global_clock;
                processes.push_back(process);
            } else {
                io_queue.push_back(process);
            }
        }
    }

    println!("Global Clock: {}", global_clock);
    print_processes(graveyard);
}

fn print_processes(mut processes: VecDeque<process::Process>) {
    // Init a new table object for printing with four columns
    // Process Name, Response Time, Wait Time, Turnaround Time.
    let mut table = Table::new("{:<} {:<} {:<} {:<}");
    table.add_heading("Process Scheduler Results");
    table.add_row(
        Row::new()
            .with_cell("Name")
            .with_cell("Tr")
            .with_cell("Tw")
            .with_cell("Ttr"),
    );

    // Set up variables for calculating averages later
    let mut response_avg: i32 = 0;
    let mut waiting_avg: i32 = 0;
    let mut turnaround_avg: i32 = 0;

    // Sort vector of processes by name
    processes
        .make_contiguous()
        .sort_by_key(|process| process.name.clone());

    for process in processes.iter() {
        let turnaround_time = process.waiting_time + process.total_process_time;

        response_avg += process.first_accessed.unwrap();
        waiting_avg += process.waiting_time;
        turnaround_avg += turnaround_time;

        table.add_row(
            Row::new()
                .with_cell(&process.name)
                .with_cell(process.first_accessed.unwrap())
                .with_cell(process.waiting_time)
                .with_cell(turnaround_time),
        );
    }

    let num_processes = processes.len();

    response_avg /= num_processes as i32;
    waiting_avg /= num_processes as i32;
    turnaround_avg /= num_processes as i32;

    table.add_row(
        Row::new()
            .with_cell("Averages")
            .with_cell(response_avg)
            .with_cell(waiting_avg)
            .with_cell(turnaround_avg),
    );

    print!("{}", table);
}
