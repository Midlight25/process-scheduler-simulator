use std::cmp::min;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Default)]
pub struct Process<T: PartialOrd + Sub<Output = T> + AddAssign + Ord> {
    // Vector queue containing all processing bursts (CPU and I/O)
    // When number of processes is odd, it is currently CPU burst
    // When number of processes is even, it is currently I/O burst
    process_bursts: std::collections::VecDeque<T>,
    // Last time that this process had run according to global clock
    // Used to calculate time spend in waiting queue.
    last_accessed: T,
    // Total time process spends in waiting queues.
    // Used to calculate total turnaround time.
    waiting_time: T,
    // ID String for process
    name: String,
    // Time Point calculated against global clock for processes return from
    // IO burst state.
    return_from_IO_time: T,
    // Calculated at process creation time, total burst times summed up from process_bursts
    // used to calculate total turnaround time.
    total_process_time: T,
}

impl<T: PartialOrd + Sub<Output = T> + AddAssign + SubAssign + Add<Output = T> + Ord> Process<T> {
    pub fn run(&self, time_quanta: T, global_clock: T) -> T {
        /* Counts down on the current process burst with the time-quanta that the process was alloted.
            Returns any un-used time-quanta. Updates total waiting time and time-last accessed.
            Time quanta must be a positive integer.
        */
        
        let zero: T = 0;

        // Precondition, time_quanta cannot be a negative number.
        assert!(time_quanta >= zero);

        // Update waiting time
        self.waiting_time += global_clock - self.last_accessed;

        // Subtract time_quanta from current process burst to get unused time.
        self.process_bursts[0] -= time_quanta;
        let unused_time: T = min(self.process_bursts[0], 0);

        // Update time last accessed
        self.last_accessed = global_clock + (time_quanta - unused_time);

        return unused_time.abs();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn run_process() {
        let mut bursts = VecDeque::<i32>::new();

        bursts.push_back(5);
        bursts.push_back(4);

        let mut process_1 = super::Process {
            process_bursts: bursts,
            total_process_time: 9,
            name: "P1".to_string(),
            ..Default::default()
        };

        let left_over = process_1.run(4, 10);
        assert_eq!(left_over, 1);
    }
}
