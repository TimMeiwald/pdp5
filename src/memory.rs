use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use crate::{memory, unsigned_integer_12::u12};
pub struct Memory {
    memory: [u12; 4096], // 4096 words of 12 bits
                         // We represent each 12 bit word as 16 bits
                         // Since Rust does not support 12 bit primitives.
}
impl Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Memory:")?;
        let mut count = 0;
        const CHUNK_SIZE: usize = 32;
        for value in self.memory.chunks(CHUNK_SIZE) {
            if count % 1 == 0 {
                write!(f, "\nAddress: {:#03x}, Data: ", count)?;
            }
            let _: Vec<_> = value
                .iter()
                .map(|val| -> Result<(), std::fmt::Error> {
                    write!(f, "{:03x} ", val)?;
                    Ok(())
                })
                .collect();
            count += CHUNK_SIZE;
        }
        writeln!(f, "")?;
        Ok(())
    }
}
impl Index<u12> for Memory {
    type Output = u12;
    fn index(&self, index: u12) -> &Self::Output {
        &self.memory[usize::from(index)]
    }
}
impl IndexMut<u12> for Memory {
    fn index_mut(&mut self, index: u12) -> &mut Self::Output {
        &mut self.memory[usize::from(index)]
    }
}

impl Memory {
    pub fn default(buf: [u12; 4096]) -> Memory {
        Memory { memory: buf }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_addr_0() {
        let buf: [u12; 4096] = [0.into(); 4096];
        let mem = Memory::default(buf);
        let address: u12 = 0.into();
        let result = mem[address];
        println!("{result:?}");
        assert_eq!(result, 0.into());
    }
    #[test]
    fn test_read_addr_max() {
        let buf: [u12; 4096] = [0.into(); 4096];
        let mem = Memory::default(buf);
        let address: u12 = u16::MAX.into();
        let result = mem[address];
        println!("{result:?}");
        assert_eq!(result, 0.into());
    }

    #[test]
    fn test_write_addr_0() {
        let buf: [u12; 4096] = [0.into(); 4096];
        let mut mem = Memory::default(buf);
        let address: u12 = 0.into();
        mem[address] = 100.into();
        let result = mem[address];
        println!("{result:?}");
        assert_eq!(result, 100.into());
    }
    #[test]
    fn test_write_addr_max() {
        let buf: [u12; 4096] = [0.into(); 4096];
        let mut mem = Memory::default(buf);
        let address: u12 = u16::MAX.into();
        mem[address] = 100.into();
        let result = mem[address];
        println!("{result:?}");
        println!("{:?}", mem);
        assert_eq!(result, 100.into());
    }
}
