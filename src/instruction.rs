use crate::{consts::*, Registers};
use crate::memory::Memory;
use crate::unsigned_integer_12::u12;

enum InstructionEvent{
    Nothing,
    SkipNextInstruction,
}

trait MemoryReferenceInstruction{
    fn execute(&self, registers: &mut Registers, memory: &mut Memory);
}


///Logical AND. The AND operation is performed
/// between the C(Y) and the C(AC). The result
/// is left in the AC, and the original C(AC) are
/// lost. The C(Y) are unchanged. Corresponding
/// bits are compared independently. This in-
/// struction, often called extract or mask, can
/// be considered as a bit-by-bit multiplication.
pub struct InstrAnd(u12);
impl MemoryReferenceInstruction for InstrAnd{
    fn execute(&self, registers: &mut Registers, memory: &mut Memory){
        let accumulator = registers.hardware_registers.AC;
        let y = memory.load(self.0);
        let result = y & accumulator;
        registers.hardware_registers.AC = result;
    }
}

/// Twos complement add. The C(Y) are added
/// to the C(AC) in twos complement arithmetic.
/// The result is left in the AC and the original
/// C(AC) are lost. The C(Y) are unchanged. If
/// there is a carry from AC,,, the link is comple-
/// mented. This feature is useful in multiple pre-
/// cision arithmetic.
/// C(Y) + C(AC) = > C(A
pub struct InstrTad(u12);
impl MemoryReferenceInstruction for InstrTad{
    fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        let accumulator = registers.hardware_registers.AC;
        let y = memory.load(self.0);
        println!("WARNING: Tad may not be correct, I just used + because lazy, TODO!");
        // In particular still need to handle updating the link register. 
        let result = y + accumulator;
        registers.hardware_registers.AC = result;


    }
}
// Index and skip if zero. The C(Y) are incre-
// mented by one in twos complement arith-
// metic. If the resultant C(Y) = 0, the next
// instruction is skipped. If the resultant
// C(Y) != 0, the program proceeds to the next
// instruction. The C(AC) are unaffected.

pub struct InstrIsz(u12);
impl MemoryReferenceInstruction for InstrIsz{
    fn execute(&self, _registers: &mut Registers, memory: &mut Memory) {
        let y = memory.load_and_increment(self.0);
        if y == 0.into(){
            memory.increment_pc_counter();
        }
    }
}

/// Deposit and clear AC. The C(AC) are deposited
/// in core memory location Y and the AC is then
/// cleared. The previous C(Y) are lost.
pub struct InstrDca(u12);
impl MemoryReferenceInstruction for InstrDca{
    fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        memory.set_value(self.0, registers.hardware_registers.AC);
        registers.hardware_registers.AC = 0.into();
    }
}