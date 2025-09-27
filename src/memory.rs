use crate::unsigned_integer_12::u12;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};
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

    fn get_current_page(&self) -> u12 {
        let mask: u12 = 0b1111_1000_0000.into();
        let program_counter = self.memory[0];
        let result = program_counter & mask;
        let result = result >> 7.into();
        debug_assert!(result < 32.into()); // Only 32 pages, i.e 0-31
        result
    }

    fn get_page_address(&self) -> u12 {
        let cur_page = self.get_current_page();
        cur_page * 128.into()
    }


    /// Ensures that address content is autoindexed if page 0 from location 10 to 17.
    fn autoindex(&mut self, address: u12) -> u12 {
        let result = self.memory[address];
        if address >= 10.into() && address <= 17.into() {
            // Increments after load for autoindex locations.
            self.memory[address] += 1.into();
        }
        result
    }

    /// Return the absolute address(not the contents) of a given address/zero_page/increment_before_load combo
    pub fn get_absolute_address(&mut self, address: u12, indirect: bool, zero_page: bool) -> u12 {
        let mask = 0b0000_0111_1111.into();
        let address = address & mask; // Only lower 7 bits are used for address
        match zero_page {
            true => match indirect {
                false => address,
                true => self.autoindex(address),
            },
            false => {
                let page_address = self.get_page_address();
                match indirect {
                    false => page_address + address,
                    true => self.autoindex(page_address + address),
                }
            }
        }
    }
    fn get_indirect_addressing(instruction: u12) -> bool {
        let indirect_mask: u12 = 0b0001_0000_0000.into();
        if (indirect_mask & instruction) > 0.into() {
            true
        } else {
            false
        }
    }
    fn get_addressing_page_0(instruction: u12) -> bool {
        let page_0_or_current_page_mask: u12 = 0b0000_1000_0000.into();
        if (page_0_or_current_page_mask & instruction) > 0.into() {
            true
        } else {
            false
        }
    }

    // Increment is for e.g Isz not related to auto indexing
    // Not sure what expected behaviour for Isz that touches autoincrement even is.
    pub fn load_and_increment(&mut self, instruction: u12) -> u12 {
        let indirect = Memory::get_indirect_addressing(instruction);
        let zero_page = Memory::get_addressing_page_0(instruction);
        let address = self.get_absolute_address(instruction, indirect, zero_page);
        // Unsure if autoincrement and ISZ increment can both happen to make a 2 jump or if they are exclusive.
        // Increments contents before load for ISZ instruction
        self.memory[address] += 1.into();
        self.autoindex(address)
    }

    pub fn load(&mut self, instruction: u12) -> u12 {
        let indirect = Memory::get_indirect_addressing(instruction);
        let zero_page = Memory::get_addressing_page_0(instruction);
        let address = self.get_absolute_address(instruction, indirect, zero_page);
        self.autoindex(address)
    }

    /// Used in e.g dca Y, we set the contents of a given indirect/direct/zero_page address to the accumulator.
    pub fn set_value(&mut self, instruction: u12, value: u12) {
        let abs_address = self.load(instruction);
        self.memory[abs_address] = value;
    }

    pub fn increment_pc_counter(&mut self) {
        self.memory[0] += 1.into();
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

    #[test]
    fn test_get_current_page_is_zero() {
        let buf: [u12; 4096] = [0.into(); 4096];
        let mem = Memory::default(buf);
        assert_eq!(mem.get_current_page(), 0.into())
    }

    #[test]
    fn test_get_current_page_is_highest_page() {
        let buf: [u12; 4096] = [0.into(); 4096];
        let mut mem = Memory::default(buf);
        // PC counter is 0th memory address
        mem.memory[0] = 4095.into();
        assert_eq!(mem.get_current_page(), 31.into())
    }

    #[test]
    fn test_load_indirect_direct_zero_page_and_not_zero_page() {
        let buf: [u12; 4096] = [0.into(); 4096];
        let mut mem = Memory::default(buf);
        // PC counter is 0th memory address
        mem.memory[0] = 4095.into();
        mem.memory[10] = 100.into();
        mem.memory[100] = 102.into();
        mem.memory[31 * 128] = 200.into();
        mem.memory[31 * 128 + 10] = 10.into();

        let address = mem.get_absolute_address(0.into(), false, true);
        println!("Address: {address:?}");
        assert_eq!(address, 0.into());
        assert_eq!(mem.autoindex(address), 4095.into());

        // Address 0 is Program Counter set to 4095 so current page is 31.
        // And in current page 31 at address 0 it's 200.
        let address = mem.get_absolute_address(0.into(), false, false);
        println!("Address: {address:?}");
        assert_eq!(address, (128 * 31).into());
        assert_eq!(mem.autoindex(address), 200.into());

        // In 0 page at Address 10 it's 100, and at absolute address 100 it's 102.
        let address = mem.get_absolute_address(10.into(), true, true);
        println!("Address: {address:?}");
        assert_eq!(address, 100.into());
        assert_eq!(mem.autoindex(address), 102.into());

        // Current page is 31. So Adddress 10 is 31*128 + 10, which is 10 so absolute adddress 10.
        // At absolute address 10 we set the value to 100.
        // BUT, 10 is an autoindex and we already loaded 10 in the previous line so it was autoincremented by 1.
        let address = mem.get_absolute_address(10.into(), true, false);
        println!("Address: {address:?}");
        assert_eq!(address, 10.into());
        assert_eq!(mem.autoindex(address), 101.into());
    }
}
