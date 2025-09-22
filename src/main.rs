mod memory;
mod unsigned_integer_12;
mod instruction;

use crate::memory::Memory;
use crate::unsigned_integer_12::u12;

const EVENT_TIME_NS: u32 = 1000; // The timing for pulses to external IOT devices. 
const CYCLE_TIME_NS: u32 = 6 * EVENT_TIME_NS; /* We use nanoseconds not microseconds so we can easily
accelerate the simulation for test and debug purposes. */



struct HWRegisters {
    AC: u12, // 12 bit Accumulator
    L: u8,   // 1 bit Link, carry register for accumulator, to simplify 2's complement arithmetic.
    MB: u12, // 12 bit Memory Buffer register, used for buffering between registers and memory.
    MA: u12, // 12 bit, Memory Address register, Currently selected address for reading or writing into memory.
    IR: u8,  // 4 bit Instruction Register, Information is loaded into the IR from the
             // memory buffer register during a Fetch cycle.
}

/// The following are all not real registers but rather are special locations in memory.
/// Corresponding to Memory Address 0, 1 and 2 respectively. 
struct PseudoRegisters {
    PC: u16, // 12 bit Program Counter register, This is actually memory location 0 so can be manipulated
    // by the program.(And must subsequently be loaded from MB as this buffers between registers and memory
    // which is what the PC actually is.)
    PC_TEMP: u16, // 12 bit Unnamed and like PC not a real register. But Memory address 1 is used to store the current
    // PC value when an interrupt is triggered which then loads address 2(so it can be reloaded), then IR is loaded.
    INTERRUPT: u16, // 12 bit Unnamed, but like PC not a real register. But Memory address 2 is used to store the
                    // start location of the Interrupt subroutine.
}

struct Registers {
    hardware_registers: HWRegisters,
    pseudo_registers: PseudoRegisters,
}

struct Instructions{
    mr: MemoryReferenceInstructions, // Memory reference instructions store or retrieve data from core memory, 
    aug: AugmentedInstructions // While augmented do not. 

}
struct MemoryReferenceInstructions{

}
struct AugmentedInstructions{

}


/// State machine for the computer state
/// StateProgramCounter -> StateFetch


/// PROGRAM COUNTER (P): This state reads the contents of the program 
/// counter from core memory location 0 into the MB, 
/// increments the contents of the MB by 1 (or 2 for a skip instruction), and 
/// rewrites the contents of the
/// MB back in location 0. The incremented contents of the PC remain in the MB
/// as the address of the current instruction. During a jump or jump to subroutine
/// instruction, the effective address specified by the jmp or jms is written into
/// location 0 to transfer program control. Completion of a P cycle initiates a
/// Fetch cycle.
struct StateProgramCounter;
impl StateProgramCounter{
    /// Default initial conditions for PDP5 on power up. 
    fn default() -> StateProgramCounter {
        StateProgramCounter{}
        
    }
    fn execute(state: &mut MachineState) -> StateFetch {
        // Set MB to contents of address 0 
        state.registers.hardware_registers.MB = state.memory[0.into()];
        // Increment MB by 1
        state.registers.hardware_registers.MB += 1.into();
        // Write MB back into PC
        state.memory[0.into()] = state.registers.hardware_registers.MB;
        StateFetch{}
    }
}

/// FETCH (F): During this state an instruction word is read from the core 
/// memory location specified by the contents of the program counter.
struct StateFetch;
impl StateFetch{
    fn execute(state: &mut MachineState) {
        // Gonna assume that we've loaded MB from address 0 in the ProgramCounter state
        // Unsure if this is always true
        let instruction = state.memory[state.registers.hardware_registers.MB];

    }
}
struct Execute_1;
struct Execute_2;
struct Defer;
struct Break;

enum CycleState{
    PC(StateProgramCounter),
    F(StateFetch),
    E1(Execute_1),
    E2(Execute_2),
    D(Defer),
    B(Break),
}
struct MachineState{
    registers: Registers,
    memory: Memory,
    state: CycleState
}

fn main() {
    println!("Hello, world!");
}
