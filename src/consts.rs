// Address of PseudoRegister Program Counter.
pub const PC_ADDRESS: u16 = 0;

pub const EVENT_TIME_NS: u32 = 1000; // The timing for pulses to external IOT devices. 
pub const CYCLE_TIME_NS: u32 = 6 * EVENT_TIME_NS; /* We use nanoseconds not microseconds so we can easily accelerate the simulation for test and debug purposes. */
