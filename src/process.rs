use std::collections::VecDeque;

mod processes {
    pub struct process<T> {
        process_bursts: VecDeque<T>,
        last_accessed: T,
        waiting_time: T,
        name: String,
    }
}
