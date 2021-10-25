use std::cmp::{min, Ordering};

#[derive(Default)]
pub struct Process {
    // Vector queue containing all processing bursts (CPU and I/O)
    // When number of processes is odd, it is currently CPU burst
    // When number of processes is even, it is currently I/O burst
    pub process_bursts: std::collections::VecDeque<i32>,
    // Last time that this process had run according to global clock
    // Used to calculate time spend in waiting queue.
    pub last_accessed: i32,
    // Total time process spends in waiting queues.
    // Used to calculate total turnaround time.
    pub waiting_time: i32,
    // ID String for process
    pub name: String,
    // Time Point calculated against global clock for processes return from
    // IO burst state.
    pub return_from_io_time: i32,
    // Calculated at process creation time, total burst times summed up from process_bursts
    // used to calculate total turnaround time.
    pub total_process_time: i32,
}

impl Process {
    pub fn run(&mut self, time_quanta: i32, global_clock: i32) -> i32 {
        /* Counts down on the current process burst with the time-quanta that the process was alloted.
            Returns any un-used time-quanta. Updates total waiting time and time-last accessed.
            Time quanta must be a positive integer.
        */

        // Precondition, time_quanta cannot be a negative number.
        assert!(time_quanta >= 0);

        // Get current process_burst from VeqDeque
        let process_burst = match self.process_bursts.get_mut(0) {
            Some(x) => x,
            None => panic!("No burst found for Process"),
        };

        // Update waiting time
        self.waiting_time = self.waiting_time + (global_clock - self.last_accessed);

        // Subtract time_quanta from current process burst to get unused time.
        // Unused time is greater than zero when process burst is put into negative
        *process_burst = *process_burst - time_quanta;
        let unused_time: i32 = min(*process_burst, 0);

        // Update time last accessed as last bit of clock before process burst expires
        self.last_accessed = global_clock + (time_quanta - unused_time);

        return i32::abs(unused_time);
    }

    pub fn calc_return_time(&mut self, global_clock: i32) {
        // Get current CPU burst from top of Queue
        let process_burst = match self.process_bursts.get(0) {
            Some(num_reference) => num_reference,
            None => panic!("No bursts found for Process"),
        };

        // Save to struct member for later.
        self.return_from_io_time = global_clock + process_burst;
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.process_bursts
            .get(0)
            .unwrap()
            .partial_cmp(&other.process_bursts.get(0).unwrap())
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.process_bursts.get(0).unwrap() == other.process_bursts.get(0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn run_process() {
        let mut process_1 = Process {
            process_bursts: VecDeque::from(vec![5, 4, 3, 2, 1]),
            total_process_time: 15,
            name: "P1".to_string(),
            ..Default::default()
        };

        // Time quanta < Current CPU burst, no time is left over
        let left_over = process_1.run(4, 10);
        assert_eq!(left_over, 0);

        // Time quanta > current CPU burst, 1 unit is left over
        process_1.process_bursts.rotate_left(1);
        let left_over = process_1.run(5, 15);
        assert_eq!(left_over, 1)
    }

    #[test]
    fn run_return_calc() {
        let mut process = Process {
            process_bursts: VecDeque::from(vec![5, 4, 3]),
            total_process_time: 12,
            name: "P2".to_string(),
            ..Default::default()
        };

        // Calculate remaining time and access
        process.calc_return_time(10);
        assert_eq!(process.return_from_io_time, 15);

        // Rotate and calculate anew
        process.process_bursts.rotate_left(1);
        process.calc_return_time(15);
        assert_eq!(process.return_from_io_time, 19);
    }

    #[test]
    fn run_get_next() {
        let mut process = Process {
            process_bursts: VecDeque::from(vec![6, 2, 1]),
            total_process_time: 6 + 2 + 1,
            name: "P3".to_string(),
            ..Default::default()
        };

        process.run(6, 10);
        process.process_bursts.pop_front();
        process.calc_return_time(10);
        process.process_bursts.pop_front();
    }

    #[test]
    fn check_ordering() {
        let process = Process {
            process_bursts: VecDeque::from(vec![6, 2, 1]),
            total_process_time: 6 + 2 + 1,
            name: "P3".to_string(),
            ..Default::default()
        };

        let process_2 = Process {
            process_bursts: VecDeque::from(vec![7, 2, 1]),
            total_process_time: 6 + 2 + 1,
            name: "P3".to_string(),
            ..Default::default()
        };

        assert!(process < process_2);
    }
}
