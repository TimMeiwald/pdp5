use crate::consts::*;
use crate::memory::Memory;
use crate::unsigned_integer_12::u12;
pub struct MemoryReferenceInstruction {
    current_page?
    instruction: u12,
    address: u12, // Bitmasked value without the indirect addressing etc.
    opcode: OpCode,
    indirect_addressing: bool,
    page_0: bool,
}
impl MemoryReferenceInstruction {
    /// If bit 3 is 1 then it's indirect addressing mode where the 7 bits
    /// point to the address in page 0 or current page(depending on bit 4)
    /// where the absolute address is actually stored which adds a cycle of time
    /// as we then load that absolute address.
    /// If bit 3 is 0 then it's direct addressing mode where the 7 bits
    /// point to the address in page 0 or current pag(depending on bit 4) that
    /// we want to load.
    /// Returns the content of whatever address whether direct or indirect.
    ///
    pub fn new(instruction: u12) -> MemoryReferenceInstruction {
        let opcode = OpCode::from(instruction);
        let indirect_addressing = MemoryReferenceInstruction::_get_indirect_addressing(instruction);
        let page_0 = MemoryReferenceInstruction::_get_addressing_page_0(instruction);
        let address = MemoryReferenceInstruction::_get_address(instruction);
        MemoryReferenceInstruction {
            instruction,
            address,
            opcode,
            indirect_addressing,
            page_0,
        }
    }
    pub fn get_opcode(&self) -> OpCode {
        self.opcode
    }
    pub fn get_address(&self) -> u12 {
        self.address
    }
    pub fn get_addressing_page_0(&self) -> bool {
        self.page_0
    }
    pub fn get_indirect_addressing(&self) -> bool {
        self.page_0
    }

    fn _get_indirect_addressing(instruction: u12) -> bool {
        let indirect_mask: u12 = 0b0001_0000_0000.into();
        if (indirect_mask & instruction) > 0.into() {
            true
        } else {
            false
        }
    }
    fn _get_addressing_page_0(instruction: u12) -> bool {
        let page_0_or_current_page_mask: u12 = 0b0000_1000_0000.into();
        if (page_0_or_current_page_mask & instruction) > 0.into() {
            true
        } else {
            false
        }
    }
    fn _get_address(instruction: u12) -> u12 {
        let mask: u12 = 0b0000_0111_1111.into();
        return instruction & mask;
    }
}

/// Logical AND between Y and Acc
/// Where Y is some core memory location.
struct InstrAnd {
    instruction_time: u32,
}
impl InstrAnd {
    pub fn new() -> InstrAnd {
        InstrAnd {
            instruction_time: CYCLE_TIME_NS * 3,
        }
    }
    pub fn execute() {}
}

// Index and skip if zero. The C(Y) are incre-
// mented by one in twos complement arith-
// metic. If the resultant C(Y) = 0, the next
// instruction is skipped. If the resultant
// C(Y) != 0, the program proceeds to the next
// instruction. The C(AC) are unaffected.
// C(Y) + 1 = > C(Y) .
// if result = 0, C(PC) + 1 = > C(PC).
struct InstrISZ {
    instruction_time: u32,
}
impl InstrISZ {
    pub fn new() -> InstrISZ {
        InstrISZ {
            instruction_time: CYCLE_TIME_NS * 3,
        }
    }
    pub fn execute() {}
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum OpCode {
    AND = 0,
    TAD = 1,
    ISZ = 2,
    DCA = 3,
    JMS = 4,
    JMP = 5,
    IOT = 6,
    OPERATE = 7,
}
impl From<u12> for OpCode {
    fn from(item: u12) -> OpCode {
        let mask: u16 = 0b0000_0000_0000_0111;
        let value: u16 = u16::from(item) & mask;
        match value {
            0 => OpCode::AND,
            1 => OpCode::TAD,
            2 => OpCode::ISZ,
            3 => OpCode::DCA,
            4 => OpCode::JMS,
            5 => OpCode::JMP,
            6 => OpCode::IOT,
            7 => OpCode::OPERATE,
            _ => panic!("Not a Opcode."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opcode_7() {
        let value: u12 = 7.into();
        assert_eq!(OpCode::OPERATE, value.into())
    }
    #[test]
    fn test_get_opcode_0() {
        let value: u12 = 0.into();
        assert_eq!(OpCode::AND, value.into())
    }
    #[test]
    fn test_get_opcode_8() {
        // Opcode 8 does not exist, only first 3 bits are used
        //  So 8 unsuprisingly yields 0.
        let value: u12 = 8.into();
        assert_eq!(OpCode::AND, value.into())
    }
}
