#[cfg(test)]
pub mod tests {
    use machine::memory::memory::Memory;

    #[test]
    fn read_write() {
        let mut mem = Memory::new(1024).unwrap();

        mem.write_u8(0, u8::MAX).unwrap();
        assert_eq!(mem.read_u8(0).unwrap(), u8::MAX);

        mem.write_u16(100, u16::MAX).unwrap();
        assert_eq!(mem.read_u16(100).unwrap(), u16::MAX);

        mem.write_u32(200, u32::MAX).unwrap();
        assert_eq!(mem.read_u32(200).unwrap(), u32::MAX);

        mem.write_u64(300, u64::MAX).unwrap();
        assert_eq!(mem.read_u64(300).unwrap(), u64::MAX);
    }

    #[test]
    #[should_panic]
    fn invalid_read(){
        let mut mem = Memory::new(32).unwrap();
        mem.write_u64(30, 120).unwrap();
    }
}