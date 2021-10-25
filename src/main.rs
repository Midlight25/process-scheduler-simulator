mod process;
mod scheduler;

use std::collections::VecDeque;

fn main() {
    let process = process::Process {
        process_bursts: VecDeque::from(vec![5, 27, 3, 31, 5, 43, 4, 18, 6, 22, 4, 26, 3, 24, 5]),
        total_process_time: 5 + 27 + 3 + 31 + 5 + 43 + 4 + 18 + 6 + 22 + 4 + 26 + 3 + 24 + 5,
        name: "P1".to_string(),
        ..Default::default()
    };

    let process_2 = process::Process {
        process_bursts: VecDeque::from(vec![
            4, 48, 5, 44, 7, 42, 12, 37, 9, 76, 4, 41, 9, 31, 7, 43, 8,
        ]),
        total_process_time: 4
            + 48
            + 5
            + 44
            + 7
            + 42
            + 12
            + 37
            + 9
            + 76
            + 4
            + 41
            + 9
            + 31
            + 7
            + 43
            + 8,
        name: "P2".to_string(),
        ..Default::default()
    };

    let process_3 = process::Process {
        process_bursts: VecDeque::from(vec![
            8, 33, 12, 41, 18, 65, 14, 21, 4, 61, 15, 18, 14, 26, 5, 31, 6,
        ]),
        total_process_time: 8
            + 33
            + 12
            + 41
            + 18
            + 65
            + 14
            + 21
            + 4
            + 61
            + 15
            + 18
            + 14
            + 26
            + 5
            + 31
            + 6,
        name: "P3".to_string(),
        ..Default::default()
    };

    let process_4 = process::Process {
        process_bursts: VecDeque::from(vec![
            3, 35, 4, 41, 5, 45, 3, 51, 4, 61, 5, 54, 6, 82, 5, 77, 3,
        ]),
        total_process_time: 3
            + 35
            + 4
            + 41
            + 5
            + 45
            + 3
            + 51
            + 4
            + 61
            + 5
            + 54
            + 6
            + 82
            + 5
            + 77
            + 3,
        name: "P4".to_string(),
        ..Default::default()
    };

    let process_5 = process::Process {
        process_bursts: VecDeque::from(vec![
            16, 24, 17, 21, 5, 36, 16, 26, 7, 31, 13, 28, 11, 21, 6, 13, 3, 11, 4,
        ]),
        total_process_time: 16
            + 24
            + 17
            + 21
            + 5
            + 36
            + 16
            + 26
            + 7
            + 31
            + 13
            + 28
            + 11
            + 21
            + 6
            + 13
            + 3
            + 11
            + 4,
        name: "P5".to_string(),
        ..Default::default()
    };

    let process_6 = process::Process {
        process_bursts: VecDeque::from(vec![
            11, 22, 4, 8, 5, 10, 6, 12, 7, 14, 9, 18, 12, 24, 15, 30, 8,
        ]),
        total_process_time: 11
            + 22
            + 4
            + 8
            + 5
            + 10
            + 6
            + 12
            + 7
            + 14
            + 9
            + 18
            + 12
            + 24
            + 15
            + 30
            + 8,
        name: "P6".to_string(),
        ..Default::default()
    };

    let process_7 = process::Process {
        process_bursts: VecDeque::from(vec![
            14, 46, 17, 41, 11, 42, 15, 21, 4, 32, 7, 19, 16, 33, 10,
        ]),
        total_process_time: 14 + 46 + 17 + 41 + 11 + 42 + 15 + 21 + 4 + 32 + 7 + 19 + 16 + 33 + 10,
        name: "P7".to_string(),
        ..Default::default()
    };

    let process_8 = process::Process {
        process_bursts: VecDeque::from(vec![4, 14, 5, 33, 6, 51, 14, 73, 16, 87, 6]),
        total_process_time: 4 + 14 + 5 + 33 + 6 + 51 + 14 + 73 + 16 + 87 + 6,
        name: "P8".to_string(),
        ..Default::default()
    };

    let processes = VecDeque::from(vec![
        process, process_2, process_3, process_4, process_5, process_6, process_7, process_8,
    ]);

    scheduler::fcfs_scheduler(processes);
}
