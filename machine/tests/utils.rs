#[cfg(test)]
pub mod tests {
    use machine::utils::binary::{Combine, Split};

    #[test]
    pub fn split_combine() {
        // u16

        let val = u16::split(1234);
        assert_eq!(val.0, 4);
        assert_eq!(val.1, 210);
        
        let val = u16::combine((4, 210));
        assert_eq!(val, 1234);

        // u32

        let val: (u16, u16) = u32::split(123123);
        assert_eq!(val.0, 1);
        assert_eq!(val.1, 57587);

        let val = u32::combine((1, 57587));
        assert_eq!(val, 123123);

        let val: (u8, u8, u8, u8) = u32::split(123123);
        assert_eq!(val.0, 0);
        assert_eq!(val.1, 1);
        assert_eq!(val.2, 224);
        assert_eq!(val.3, 243);

        let val = u32::combine((0, 1, 224, 243));
        assert_eq!(val, 123123);

        // u64

        let val: (u32, u32) = u64::split(123123123123);
        assert_eq!(val.0, 28);
        assert_eq!(val.1, 2864038835);

        let val = u64::combine((28, 2864038835));
        assert_eq!(val, 123123123123);

        let val: (u16, u16, u16, u16) = u64::split(123123123123);
        assert_eq!(val.0, 0);
        assert_eq!(val.1, 0b00011100);
        assert_eq!(val.2, 0b1010101010110101);
        assert_eq!(val.3, 0b1100001110110011);

        let val = u64::combine((0, 0b00011100, 0b1010101010110101, 0b1100001110110011));
        assert_eq!(val, 123123123123);
        
        let val: (u8, u8, u8, u8, u8, u8, u8, u8) = u64::split(123123123123);
        assert_eq!(val.0, 0);
        assert_eq!(val.1, 0);
        assert_eq!(val.2, 0);
        assert_eq!(val.3, 0b00011100);
        assert_eq!(val.4, 0b10101010);
        assert_eq!(val.5, 0b10110101);
        assert_eq!(val.6, 0b11000011);
        assert_eq!(val.7, 0b10110011);

        let val = u64::combine((0,0,0,0b00011100,0b10101010,0b10110101,0b11000011,0b10110011));
        assert_eq!(val, 123123123123);
    }
}