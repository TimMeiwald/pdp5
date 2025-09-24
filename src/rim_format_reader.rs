
use crate::unsigned_integer_12::u12;
use std::io::Error;
use std::fs;
use std::path::Path;

pub struct RimFormat{}
impl RimFormat{

    fn process_address_content_pair(data: &[u8; 4]) -> [u12; 2]{
        let mut address: u16 = 0;
        let mut content: u16 = 0;
        // println!("Address: {:08b} {:08b}", data[0], data[1]);
        address = data[0] as u16;
        address = address << 6; // Shift left 6 bits
        address += data[1] as u16;
        // println!("Content: {:08b} {:08b}", data[0], data[1]);
        content = data[2] as u16;
        content = content << 6; // Shift left 6 bits
        content += data[3] as u16;
        [address.into(), content.into()]
    }

    pub fn load_from_file<'a>(path: &Path, buf: &'a mut [u12; 4096]) -> Result<&'a mut [u12; 4096], Error> {
        let data: Vec<u8> = fs::read(path)?;
        assert!(data.len() <= 4095, "Data is larger than available memory.");

        // Leader trailer codes mean nothing, Required since you needed to be able to tear tape so you wanted 
        // To be able to write nothing. This mean's the tape is essentially 7 bits not 8 bits.
        const LEADER_TRAILER_CODE: u8 = 0b1000_0000;
        // The RIM format uses the 2nd bit to determine where each absolute address: content pair starts
        // This means the data is essentially 6 bits. 
        // So 2 lines of tape per address for a single 12 bit word
        // Then 2 lines of tape per content of address for another 12 bit word. 
        // Then alternate again to address then content etc until you hit another LEADER_TRAILER_CODE.
        // This is all my speculation so grain of salt. 
        const START_ADDRESS_CONTENT_PAIR: u8 = 0b0100_0000;
        const DATA_MASK: u8 = 0b0011_1111; // We only want to load the data not start_address_content_pair bits.
        let mut buffer: [u8; 4] = [0; 4];
        let mut buffer_counter = 0;
        for byte in data{
            // We use mask since I don't think any data with a leading 1 bit has any value. 
            if byte & LEADER_TRAILER_CODE == LEADER_TRAILER_CODE{
                continue;
            }
            else{
                if byte & START_ADDRESS_CONTENT_PAIR == START_ADDRESS_CONTENT_PAIR{
                    // println!("{buffer:?}");
                    let [address, content] = RimFormat::process_address_content_pair(&buffer);
                    buf[address] = content;
                    // println!("{data:?}");
                    buffer_counter = 0;
                }
                let data= byte & DATA_MASK;
                buffer[buffer_counter] = data;
                buffer_counter += 1;
            }
        }
        Ok(&mut *buf)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_binhalt_pm() {
        let path = Path::new("example_code/binhalt-pm/binhalt-pm");
        let mut buf: [u12; 4096] = [0.into(); 4096];
        let result = RimFormat::load_from_file(path, &mut buf).expect("Don't expect an IO error");
        println!("{result:?}")
    }

}