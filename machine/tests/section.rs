#[cfg(test)]
pub mod tests {
    use machine::{fiber::{section::Section}, memory::memory::Memory};

    #[test]
    fn append_data() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let s = Section::new(&mut mem).unwrap();
        s.append_data(&mut mem, 10u8).unwrap();
        s.append_data(&mut mem, 10u16).unwrap();
        s.append_data(&mut mem, 10u32).unwrap();
        s.append_data(&mut mem, 10u64).unwrap();
    }

    #[test]
    fn read_data_u8() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let s = Section::new(&mut mem).unwrap();
        s.append_data(&mut mem, 10u8).unwrap();
        s.append_data(&mut mem, 20u8).unwrap();
        s.append_data(&mut mem, 30u8).unwrap();
        s.append_data(&mut mem, 40u8).unwrap();
        assert_eq!(s.read_data::<u8>(&mem, 0).unwrap(), 10);
        assert_eq!(s.read_data::<u8>(&mem, 1).unwrap(), 20);
        assert_eq!(s.read_data::<u8>(&mem, 2).unwrap(), 30);
        assert_eq!(s.read_data::<u8>(&mem, 3).unwrap(), 40);
    }

    #[test]
    fn read_data_u16() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let s = Section::new(&mut mem).unwrap();
        s.append_data(&mut mem, 10u16).unwrap();
        s.append_data(&mut mem, 20u16).unwrap();
        s.append_data(&mut mem, 30u16).unwrap();
        s.append_data(&mut mem, 40u16).unwrap();
        assert_eq!(s.read_data::<u16>(&mem, 0).unwrap(), 10);
        assert_eq!(s.read_data::<u16>(&mem, 1).unwrap(), 20);
        assert_eq!(s.read_data::<u16>(&mem, 2).unwrap(), 30);
        assert_eq!(s.read_data::<u16>(&mem, 3).unwrap(), 40);
    }

    #[test]
    fn read_data_u32() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let s = Section::new(&mut mem).unwrap();
        s.append_data(&mut mem, 10u32).unwrap();
        s.append_data(&mut mem, 20u32).unwrap();
        s.append_data(&mut mem, 30u32).unwrap();
        s.append_data(&mut mem, 40u32).unwrap();
        assert_eq!(s.read_data::<u32>(&mem, 0).unwrap(), 10);
        assert_eq!(s.read_data::<u32>(&mem, 1).unwrap(), 20);
        assert_eq!(s.read_data::<u32>(&mem, 2).unwrap(), 30);
        assert_eq!(s.read_data::<u32>(&mem, 3).unwrap(), 40);
    }

    #[test]
    fn read_data_u64() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let s = Section::new(&mut mem).unwrap();
        s.append_data(&mut mem, 10u64).unwrap();
        s.append_data(&mut mem, 20u64).unwrap();
        s.append_data(&mut mem, 30u64).unwrap();
        s.append_data(&mut mem, 40u64).unwrap();
        assert_eq!(s.read_data::<u64>(&mem, 0).unwrap(), 10);
        assert_eq!(s.read_data::<u64>(&mem, 1).unwrap(), 20);
        assert_eq!(s.read_data::<u64>(&mem, 2).unwrap(), 30);
        assert_eq!(s.read_data::<u64>(&mem, 3).unwrap(), 40);
    }
}