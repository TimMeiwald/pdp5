use std::{
    fmt::{Binary, Debug, LowerHex, Octal},
    ops::{AddAssign, Index, IndexMut},
};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct u12 {
    value: u16,
}
impl AddAssign for u12{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        debug_assert!(self.value <= 4095, "Overflow occurred")
    }
}
impl Index<u12> for [u12]{
    type Output = u12;
    fn index(&self, index: u12) -> &Self::Output {
        &self[*&usize::from(index)]
    }
}
impl IndexMut<u12> for [u12]{
    fn index_mut(&mut self, index: u12) -> &mut Self::Output {
        &mut self[*&usize::from(index)]
    }
}
impl Binary for u12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.value, f)
    }
}
impl Octal for u12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.value, f)
    }
}
impl LowerHex for u12{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.value, f)
    }
}
impl Debug for u12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("u12")
            .field("value in decimal", &self.value)
            .field("value in binary", &format!("{:#014b}", &self.value))
            .field("value in hex", &format!("{:#05x}", &self.value))
            .finish()
    }
}

impl From<u16> for u12 {
    fn from(item: u16) -> Self {
        let mask: u16 = 0b0000_1111_1111_1111;
        let value = item & mask;
        debug_assert!(value <= 4095);
        u12 { value }
    }
}
impl From<u12> for usize {
    fn from(item: u12) -> usize {
        item.value as usize
    }
}
impl From<&u8> for u12 {
    fn from(item: &u8) -> u12 {
        u12::from(*item as u16)
    }
}
impl From<usize> for u12 {
    fn from(item: usize) -> u12 {
        let item = item as u16;
        Self::from(item)
    }
}
impl From<i32> for u12 {
    fn from(item: i32) -> u12 {
        let item = item as u16;
        Self::from(item)
    }
}
impl From<u12> for u16 {
    fn from(item: u12) -> u16 {
        item.value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let result = u12::from(0);
        println!("{result:?}");
        assert_eq!(0, result.value);
    }
    #[test]
    fn test_max() {
        let result = u12::from(u16::MAX);
        println!("{result:?}");
        assert_eq!(4095, result.value);
    }

    #[test]
    fn test_4095() {
        let result = u12::from(4095);
        println!("{result:?}");
        assert_eq!(4095, result.value);
    }

    #[test]
    fn test_255() {
        let result = u12::from(255);
        println!("{result:?}");
        assert_eq!(255, result.value);
    }

    #[test]
    fn test_39() {
        let result = u12::from(39);
        println!("{result:?}");
        assert_eq!(39, result.value);
    }
}
