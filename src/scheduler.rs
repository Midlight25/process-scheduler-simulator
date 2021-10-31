use crate::process;
use sorts::quick_sort::quick_sort;
use std::collections::VecDeque;
use tabular::{Row, Table};

pub fn fcfs_scheduler(mut processes: VecDeque<process::Process>) {
    /*
       Run processes in <processes> using First Come First Serve Strategy,
       print Context Switch information to the console and report the results
       of the simulation to the console at the end.
    */
    let mut global_clock = 0;
    let mut wait_count = 0;
    let mut io_queue: VecDeque<process::Process> = VecDeque::new();
    let mut graveyard: VecDeque<process::Process> = VecDeque::new();

    // Run while there are processes in the ready queue or the IO queue
    // This ends when all processes have been transfered to the graveyard.
    while !processes.is_empty() || !io_queue.is_empty() {
        // Print Context Switch info
        println!("Global Clock is {}", global_clock);
        println!("Current Process Queue:");
        print_queue(&processes);
        println!("Current IO Queue:");
        print_queue(&io_queue);
        println!("Global Clock is {}", global_clock);

        // Run process at front of ready queue if there is one.
        match processes.pop_front() {
            // There is a process in ready queue.
            Some(mut process) => {
                // Get time units required to run current burst to 0, panic if there isn't.
                let process_quanta = match process.process_bursts.get(0) {
                    Some(number) => *number,
                    None => panic!("Process burst not found"),
                };

                process.run(process_quanta, global_clock);
                process.ready_next_io();

                // Advance global clock
                global_clock += process_quanta;

                // Check if this process has IO burst
                if process.process_bursts.len() > 0 {
                    process.calc_return_time(global_clock);
                    process.ready_next_cpu();

                    io_queue.push_back(process);
                // If process has no IO burst, then it is completed.
                } else {
                    println!(
                        "Process {} is done at {} units!",
                        process.name, global_clock
                    );
                    graveyard.push_back(process);
                }
            }
            // There are no processes in the queue.
            None => {
                global_clock += 1;
                wait_count += 1;
            }
        }

        // See if IO_queue processes are done and send them into the ready queue.
        for _ in 0..io_queue.len() {
            let process = io_queue.pop_front().unwrap();
            if process.return_from_io_time <= global_clock {
                processes.push_back(process);
            } else {
                io_queue.push_back(process);
            }
        }
    }

    let cpu_count = global_clock - wait_count;
    let cpu_util: f32 = cpu_count as f32 / global_clock as f32;

    // Print Final Results
    println!("First-Come-First-Serve Results");
    println!("Global Clock: {}", global_clock);
    print_processes(graveyard, cpu_util);
}

pub fn sjf_scheduler(mut processes: VecDeque<process::Process>) {
    /*
       Run processes in <processes> using Shortest Job First strategy,
       print Context Switch information to the console and report the results
       of the simulation to the console at the end.

       Note, invective of this function is that <processes> is always sorted.
       Ready queue is sorted once at the beginning of the simulation, and any
       time a new process is inserted into the ready queue.
    */
    let mut global_clock = 0;
    let mut wait_count = 0;
    let mut io_queue: VecDeque<process::Process> = VecDeque::new();
    let mut graveyard: VecDeque<process::Process> = VecDeque::new();

    quick_sort(&mut processes.make_contiguous());

    while !processes.is_empty() || !io_queue.is_empty() {
        // Print Context Switch Information.
        println!(
            "Global Clock is {} ---------------------------",
            global_clock
        );
        println!("Current Process Queue:");
        print_queue(&processes);
        println!("Current IO Queue:");
        print_queue(&io_queue);
        println!(
            "Global Clock is {} ---------------------------",
            global_clock
        );

        // Run process at front of ready queue, if there is one.
        match processes.pop_front() {
            // There is a process in the ready queue.
            Some(mut process) => {
                // Get time units required to run current burst to 0, panic if there isn't.
                let process_quanta = match process.process_bursts.get(0) {
                    Some(number) => *number,
                    None => panic!("Process burst not found"),
                };

                process.run(process_quanta, global_clock);
                process.ready_next_io();

                global_clock += process_quanta;

                // Place into IO queue if there is process has an IO burst (which should have just been loaded.)
                if process.process_bursts.len() > 0 {
                    process.calc_return_time(global_clock);
                    process.ready_next_cpu();
                    io_queue.push_back(process);
                } else {
                    println!(
                        "Process {} is done at {} units!",
                        process.name, global_clock
                    );
                    graveyard.push_back(process);
                }
            }

            // No process in the queue.
            None => {
                global_clock += 1;
                wait_count += 1;
            }
        }

        // See if processes are done with IO and send them into the waiting queue.
        for _ in 0..io_queue.len() {
            let process = io_queue.pop_front().unwrap();

            if process.return_from_io_time <= global_clock {
                // Sort processes by CPU burst size to maintain invective.
                processes.push_back(process);
                quick_sort(&mut processes.make_contiguous());
            } else {
                io_queue.push_back(process);
            }
        }
    }

    let cpu_count = global_clock - wait_count;
    let cpu_util: f32 = cpu_count as f32 / global_clock as f32;

    println!("\nShortest Job First Results");
    println!("Global Clock: {}", global_clock);
    print_processes(graveyard, cpu_util);
}

pub fn mlfq_scheduler(processes: VecDeque<process::Process>) {
    /*
        Run processes in <processes> using Multi-Level Feedback Queue,
       print Context Switch information to the console and report the results
       of the simulation to the console at the end.

       Note, invective of this function is that sjf_queue is always sorted.
       Queue is sorted whenever a process is inserted (it starts out empty).

       Invective of this function is that IO_queue is always sorted.
       Queue is sorted whenever a process is inserted (it starts out empty).
       Therefore, a process that is guaranteed to leave the queue before any
       of the others is always at the front of the IO_queue.
    */

    let mut global_clock: i32 = 0;
    let mut wait_count : i32 = 0;
    // Highest Priority Queue, RR time quanta of 5
    let mut level_one: VecDeque<process::Process> = processes;
    // Second Priority Queue, RR time quanta of 10
    let mut level_two: VecDeque<process::Process> = VecDeque::new();
    // Last queue: to save resources, only gets sorted when something is inserted.
    let mut sjf_queue: VecDeque<process::Process> = VecDeque::new();

    // Processes in IO state are saved here
    let mut io_queue: VecDeque<process::Process> = VecDeque::new();

    // Processes that have completed are stored here.
    let mut graveyard: VecDeque<process::Process> = VecDeque::new();

    // Loop ends when all processes have been placed in the graveyard, or no processes were supplied.
    while !level_one.is_empty()
        || !level_two.is_empty()
        || !sjf_queue.is_empty()
        || !io_queue.is_empty()
    {
        // Print Context Switch Information.
        println!(
            "Global Clock is {} ---------------------------",
            global_clock
        );
        println!("Current Level One Queue:");
        print_queue(&level_one);
        println!("Current Level Two Queue:");
        print_queue(&level_two);
        println!("Current SJF Queue:");
        print_queue(&sjf_queue);
        println!(
            "Global Clock is {} ---------------------------",
            global_clock
        );

        // Checking Queues by priority: LEVEL ONE RR 5
        if let Some(mut process) = level_one.pop_front() {
            // Run process using time-quanta and advance global clock.
            let left_over = process.run(5, global_clock);
            global_clock += 5 - left_over;

            // Process is not guaranteed to have completed, therefore we must check
            // before moving process to IO queue or Graveyard
            if process.burst_completed {
                process.ready_next_io();
                // Checking for IO burst or send process to graveyard.
                if process.process_bursts.len() > 0 {
                    process.calc_return_time(global_clock);
                    process.ready_next_cpu();
                    io_queue.push_back(process);
                    quick_sort(&mut io_queue.make_contiguous());
                } else {
                    println!(
                        "Process {} is done at {} units!",
                        process.name, global_clock
                    );
                    graveyard.push_back(process);
                }
            // If process burst did not complete, send to level two queue. (No preemption possible for
            // processes in level one.)
            } else {
                level_two.push_back(process);
            }
        // LEVEL TWO RR 10
        } else if let Some(mut process) = level_two.pop_front() {
            // Run process using time-quanta and advance global clock.
            let left_over = process.run(10, global_clock);
            global_clock += 10 - left_over;

            // Process is not guaranteed to have completed, therefore we must check
            // before moving process to IO queue or Graveyard
            if process.burst_completed {
                process.ready_next_io();
                // CHecking for IO burst or send process to graveyard
                if process.process_bursts.len() > 0 {
                    process.calc_return_time(global_clock);
                    process.ready_next_cpu();
                    io_queue.push_back(process);
                    quick_sort(&mut io_queue.make_contiguous());
                } else {
                    println!(
                        "Process {} is done at {} units!",
                        process.name, global_clock
                    );
                    graveyard.push_back(process);
                }

            // If process did not complete, process can be downgraded unless a new process has entered
            // level one to preempt the execution.
            } else {
                // Peak top of IO queue to check for any processes that will enter Level One at end
                // of this loop.
                if let Some(io_process) = io_queue.get(0) {
                    if io_process.return_from_io_time <= global_clock {
                        level_two.push_back(process);
                    } else {
                        sjf_queue.push_back(process);
                        quick_sort(&mut sjf_queue.make_contiguous());
                    }
                // No Process at top of IO queue, there will be no preemption.
                } else {
                    sjf_queue.push_back(process);
                    quick_sort(&mut sjf_queue.make_contiguous());
                }
            }

        // Should be pre-sorted at insertion time, so popping item here should be shortest item.
        } else if let Some(mut process) = sjf_queue.pop_front() {
            // Get time units required to run current burst to 0, panic if there isn't.
            let process_quanta = match process.process_bursts.get(0) {
                Some(number) => *number,
                None => panic!("Could not find process burst for this process."),
            };

            process.run(process_quanta, global_clock);
            process.ready_next_io();

            global_clock += process_quanta;

            // Check process for IO burst or send to graveyard.
            if process.process_bursts.len() > 0 {
                process.calc_return_time(global_clock);
                process.ready_next_cpu();
                io_queue.push_back(process);
                quick_sort(&mut io_queue.make_contiguous());
            } else {
                println!(
                    "Process {} is done at {} units!",
                    process.name, global_clock
                );
                graveyard.push_back(process);
            }

        // No processes in ready queues, but IO queue is still filled
        } else {
            global_clock += 1;
            wait_count += 1;
        }

        // See if processes are done with IO and send them into the waiting queue.
        for _ in 0..io_queue.len() {
            let process = io_queue.pop_front().unwrap();
            if process.return_from_io_time <= global_clock {
                level_one.push_back(process);
            } else {
                io_queue.push_back(process);
            }
        }
    }

    let cpu_count = global_clock - wait_count;
    let cpu_util = cpu_count as f32 / global_clock as f32;

    println!("\nMLFQ Job First Results");
    println!("Global Clock: {}", global_clock);
    print_processes(graveyard, cpu_util);
}

fn print_processes(mut processes: VecDeque<process::Process>, cpu_util: f32) {
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
    let mut response_avg: f32 = 0.0;
    let mut waiting_avg: f32 = 0.0;
    let mut turnaround_avg: f32 = 0.0;

    // Sort vector of processes by name
    processes
        .make_contiguous()
        .sort_by_key(|process| process.name.clone());

    for process in processes.iter() {
        let waiting_time = process.last_accessed - process.total_process_time;

        response_avg += process.first_accessed.unwrap() as f32;
        waiting_avg += waiting_time as f32;
        turnaround_avg += process.last_accessed as f32;

        table.add_row(
            Row::new()
                .with_cell(&process.name)
                .with_cell(process.first_accessed.unwrap())
                .with_cell(waiting_time)
                .with_cell(process.last_accessed),
        );
    }

    let num_processes = processes.len();

    response_avg /= num_processes as f32;
    waiting_avg /= num_processes as f32;
    turnaround_avg /= num_processes as f32;

    table.add_row(
        Row::new()
            .with_cell("Averages")
            .with_cell(response_avg)
            .with_cell(waiting_avg)
            .with_cell(turnaround_avg),
    );

    println!("{}", table);
    println!("CPU Utilization: {}%", cpu_util * 100f32);
}

fn print_queue(process_queue: &VecDeque<process::Process>) {
    for process in process_queue.iter() {
        println!("{}", process);
    }
}
