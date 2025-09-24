mod instruction;
mod memory;
mod unsigned_integer_12;
mod rim_format_reader;

use std::fs;
use std::io::Error;
use std::path::Path;

use crate::instruction::{Instruction, OpCode};
use crate::memory::Memory;
use crate::rim_format_reader::RimFormat;
use crate::unsigned_integer_12::u12;

// Address of PseudoRegister Program Counter.
const PC_ADDRESS: u16 = 0;

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
    SR: u12, // Switch Register.
}
impl HWRegisters {
    pub fn default() -> HWRegisters {
        HWRegisters {
            AC: 0.into(),
            L: 0,
            MB: 0.into(),
            MA: 0.into(),
            IR: 0.into(),
            SR: 0.into(),
        }
    }
}

/// The following are all not real registers but rather are special locations in memory.
/// Corresponding to Memory Address 0, 1 and 2 respectively.
struct PseudoRegisters {
    // 12 bit Program Counter register, This is actually memory location 0 so can be manipulated
    // by the program.(And must subsequently be loaded from MB as this buffers between registers and memory
    // which is what the PC actually is.)
    // 12 bit Unnamed and like PC not a real register. But Memory address 1 is used to store the current
    // PC value when an interrupt is triggered which then loads address 2(so it can be reloaded), then IR is loaded.
    // 12 bit Unnamed, but like PC not a real register. But Memory address 2 is used to store the
    // start location of the Interrupt subroutine.

    // Therefore this is just a named wrapper for PseudoRegister functions
}
impl PseudoRegisters {
    fn load_pc(state: &mut MachineState, instruction: Option<Instruction>) {
        state.registers.hardware_registers.MB = state.memory[PC_ADDRESS.into()];
        match instruction {
            Some(instruction) => {
                if instruction.get_opcode() == OpCode::OPERATE {
                    todo!(
                        "If it's a group two micro instruction with the skip flag set
            We need to increment by 2 else increment by 1."
                    );
                    // Increment MB by 1
                    state.registers.hardware_registers.MB += 1.into();
                } else {
                    // Increment MB by 1
                    state.registers.hardware_registers.MB += 1.into();
                }
            }
            None => {
                    // Increment MB by 1, Start of program so no initial instruction loaded yet. 
                    state.registers.hardware_registers.MB += 1.into();
            }
        }

        // Write MB back into PC
        state.memory[PC_ADDRESS.into()] = state.registers.hardware_registers.MB;
    }

    fn set_pc(state: &mut MachineState, value: u12) {
        state.memory[PC_ADDRESS.into()] = value;
    }
}

struct Registers {
    hardware_registers: HWRegisters,
}
impl Registers {
    pub fn default() -> Registers {
        Registers {
            hardware_registers: HWRegisters::default(),
        }
    }
}

struct Instructions {
    mr: MemoryReferenceInstructions, // Memory reference instructions store or retrieve data from core memory,
    aug: AugmentedInstructions,      // While augmented do not.
}
struct MemoryReferenceInstructions {}
struct AugmentedInstructions {}

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
impl StateProgramCounter {
    /// Default initial conditions for PDP5 on power up.
    fn default() -> StateProgramCounter {
        StateProgramCounter {}
    }
    fn execute(state: &mut MachineState) -> StateFetch {
        // Set MB to contents of address 0
        state.registers.hardware_registers.MB = state.memory[0.into()];
        // Increment MB by 1
        state.registers.hardware_registers.MB += 1.into();
        // Write MB back into PC
        state.memory[0.into()] = state.registers.hardware_registers.MB;
        StateFetch {}
    }
}

/// FETCH (F): During this state an instruction word is read from the core
/// memory location specified by the contents of the program counter.
struct StateFetch;
impl StateFetch {
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

enum CycleState {
    PC(StateProgramCounter),
    F(StateFetch),
    E1(Execute_1),
    E2(Execute_2),
    D(Defer),
    B(Break),
}
impl CycleState {
    pub fn default() -> CycleState {
        CycleState::PC(StateProgramCounter::default())
    }
}

struct MachineState {
    registers: Registers,
    memory: Memory,
    state: CycleState,
}

impl MachineState {
    /// Everything is zeroed. Factory settings effectively
    /// (I think.)
    pub fn default(buf: [u12; 4096]) -> MachineState {
        let memory = Memory::default(buf);
        MachineState {
            registers: Registers::default(),
            memory: memory,
            state: CycleState::default(),
        }
    }

    /// PDP5 used rope core memory which is non-volatile
    /// So writing to memory is equivalent to flashing
    /// This is a utility function for testing
    /// Strictly speaking one should load from
    /// one of the IOT devices to really emulate a PDP5.
    /// This takes a 4096 sized array of u16
    /// which get converted into u12s. Again to make it easier
    /// for testing since u12 is kinda abnormal nowadays.
    pub fn flash(&mut self, bytes: [u16; 4096]) -> () {
        for i in 0..bytes.len() {
            self.memory[i.into()] = bytes[i].into();
        }
    }

    pub fn flash_from_file(&mut self, path: &Path) -> Result<(), Error> {
        let data: Vec<u8> = fs::read(path)?;
        assert!(data.len() <= 4095, "Data is larger than available memory.");
        for (index, byte) in data.iter().enumerate() {
            self.memory[index.into()] = byte.into()
        }
        Ok(())
    }

    pub fn set_initial_start_address(&mut self, address: usize) {
        PseudoRegisters::set_pc(self, address.into());
    }

    /// IOT pulses are at 1 microsecond interval
    /// And presumably approximately 1 microsecond long
    /// Allowing 3 pulses a cycle.
    pub fn step_event(&mut self) {}

    // An instruction can take multiple cycles.
    // This allows us to inspect the state to ensure
    // things happen at the right time.
    pub fn step_cycle(&mut self) {}
    // // Each instruction can be made up with multiple cycles.
    // pub fn step_instruction(&mut self) {
    //     let current_instr = PseudoRegisters::load_pc(state, instruction);
    // }
    pub fn start_program(&mut self){
        PseudoRegisters::load_pc(self, None);
        let mb = self.registers.hardware_registers.MB;
        let current_instr = self.memory[mb];
        println!("Current Instruction: {:?} at Address: {mb:#05x}", current_instr);
        let instr = Instruction::from(current_instr);
        println!("{:?}", instr);

    }
}

fn main() {
    let mut buf: [u12; 4096] = [0.into();  4096];
    let path = Path::new("example_code/binhalt-pm/binhalt-pm");
    let data = RimFormat::load_from_file(path, &mut buf).expect("Dont expect an IO error");

    let mut pdp5 = MachineState::default(*data);
    print!("{:?}", pdp5.memory);

    // Program start address, can be anything really but must be loaded into PC prior to start.
    let start_address = 0o07600;
    pdp5.set_initial_start_address(start_address);

    // pdp5.start_program();

    // for i in 0..500{
    //     let start_address = i;
    //     pdp5.set_initial_start_address(start_address);

    //     pdp5.start_program()
    // }
}
