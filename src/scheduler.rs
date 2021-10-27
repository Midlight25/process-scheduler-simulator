use crate::process;
use sorts::quick_sort::quick_sort;
use std::collections::VecDeque;
use tabular::{Row, Table};

pub fn fcfs_scheduler(mut processes: VecDeque<process::Process>) {
    let mut global_clock = 0;
    let mut io_queue: VecDeque<process::Process> = VecDeque::new();
    let mut graveyard: VecDeque<process::Process> = VecDeque::new();

    while !processes.is_empty() || !io_queue.is_empty() {
        
        println!("Global Clock is {}", global_clock);
        println!("Current Process Queue:");
        print_queue(&processes);
        println!("Current IO Queue:");
        print_queue(&io_queue);
        println!("Global Clock is {}", global_clock);

        // Run process at front of queue
        match processes.pop_front() {
            // If there is a process in the queue.
            Some(mut process) => {
                let process_quanta = match process.process_bursts.get(0) {
                    Some(number) => *number,
                    None => panic!("Process burst not found"),
                };

                process.run(process_quanta, global_clock);
                process.process_bursts.pop_front();

                // Advance global clock by number of units needed to complete this process
                global_clock += process_quanta;

                // Check if process has IO burst
                if process.process_bursts.len() > 0 {
                    process.calc_return_time(global_clock);
                    process.process_bursts.pop_front();

                    io_queue.push_back(process);
                // If process has no IO burst, then it is completed.
                } else {
                    graveyard.push_back(process);
                }
            }
            // There are no processes in the queue.
            None => {
                global_clock += 1;
            }
        }

        // See if IO_queue processes are done and send them into the process queue.
        for _ in 0..io_queue.len() {
            let process = io_queue.pop_front().unwrap();
            if process.return_from_io_time <= global_clock {
                processes.push_back(process);
            } else {
                io_queue.push_back(process);
            }
        }
    }

    println!("First-Come-First-Serve Results");
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

        // Print Current State of the Execution.
        println!("Global Clock is {} ---------------------------", global_clock);
        println!("Current Process Queue:");
        print_queue(&processes);
        println!("Current IO Queue:");
        print_queue(&io_queue);
        println!("Global Clock is {} ---------------------------", global_clock);

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
            let process = io_queue.pop_front().unwrap();
            if process.return_from_io_time <= global_clock {
                processes.push_back(process);
            } else {
                io_queue.push_back(process);
            }
        }
    }

    println!("Shortest Job First Results");
    println!("Global Clock: {}", global_clock);
    print_processes(graveyard);
}

pub fn mlfq_scheduler(processes: VecDeque<process::Process>) {

    let mut global_clock:i32 = 0;

    // Highest Priority Queue, RR time quanta of 5
    let mut level_one: VecDeque<process::Process> = processes;
    // Second Priority Queue, RR time quanta of 10
    let mut level_two: VecDeque<process::Process> = VecDeque::new();
    // Last queue: to save resources, only gets sorted when something is inserted.
    let mut sjf_queue: VecDeque<process::Process> = VecDeque::new();

    // Processes in IO state are saved here
    let mut io_queue: VecDeque<process::Process> = VecDeque::new();

    // Processes that have completed are stored here.
    let mut graveyard: Vec<process::Process> = Vec::new();

    // Loop ends when all processes have been placed in the graveyard, or no processes were supplied.
    while !level_one.is_empty() || !level_two.is_empty() || !sjf_queue.is_empty() || !io_queue.is_empty() {

        // Checking Queues by priority.
        if let Some(process) = level_one.pop_front() {

        } else if let Some(process) = level_two.pop_front() {

        // Should be pre-sorted at insertion time, so popping item here should be shortest item.
        } else if let Some(process) = sjf_queue.pop_front() {

        // No processes in ready queues, but IO queue is still filled
        } else {
            global_clock += 1;
        }
    }
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
        let waiting_time = process.last_accessed - process.total_process_time;

        response_avg += process.first_accessed.unwrap();
        waiting_avg += waiting_time;
        turnaround_avg += process.last_accessed;

        table.add_row(
            Row::new()
                .with_cell(&process.name)
                .with_cell(process.first_accessed.unwrap())
                .with_cell(waiting_time)
                .with_cell(process.last_accessed),
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

fn print_queue(process_queue: &VecDeque<process::Process>) {
    for process in process_queue.iter() {
        println!("{}", process);
    }
}
