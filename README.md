# Process Scheduler Simulator

This is a program written in Rust intended to simulate three process-scheduling algorithms:

- Non-premptive First Come First Serve
- Non-preemptive Shortest Job First
- Multilevel Feedback Queue
  - Queue One uses Round-Robin scheduling with time-quanta of 5.
  - Queue Two uses Round-Robin scheduling with time-quanta of 10.
  - Queue Three uses First Come First Serve.
  - All processes enter the scheduler at Queue One(RR5)
  - Processes that do not finish their CPU bursts within their time-quanta are downgraded a level.
  - There is no mechanism for upgrading to higher priority queues.

There are a few assumptions for this simulator:

1. All processes are activated at time 0
2. Assume that no process waits on I/O devices.
3. After completing an I/O event, a process is transferred to the ready queue.
4. Waiting time is accumulated while a process waits in the ready queue.
5. Turnaround time is a total of (Waiting time) + (CPU burst time) + (I/O time)6. Response time is the first measure of waiting time from arrival at time 0 until the first time on the CPU.

## Dependencies

### Sorts

A collection of sorting functions for collections. In this project, I used the Quick Sort function for efficient sorting of the many process queues.

### Tabular

A package that generates tables for use on CLI.