use crate::unsigned_integer_12::u12;

#[derive(PartialEq, Debug)]
pub enum OpCode{
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
            _ => panic!("Not a Opcode.")
        }

    }
}

pub struct Instruction {
    value: u12,
}
impl From<u12> for Instruction {
    fn from(item: u12) -> Instruction {
        Instruction { value: item }
    }
}
impl Instruction {
    pub fn get_opcode(&self) -> OpCode {
        self.value.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opcode_7() {
        let value: u12 = 7.into();
        let x = Instruction::from(value);
        assert_eq!(OpCode::OPERATE, x.get_opcode())
    }
    #[test]
    fn test_get_opcode_0() {
        let value: u12 = 0.into();
        let x = Instruction::from(value);
        assert_eq!(OpCode::AND, x.get_opcode())
    }
    #[test]
    fn test_get_opcode_8() {
        // Opcode 8 does not exist, only first 3 bits are used
        //  So 8 unsuprisingly yields 0.
        let value: u12 = 8.into();
        let x = Instruction::from(value);
        assert_eq!(OpCode::AND, x.get_opcode())
    }

}
