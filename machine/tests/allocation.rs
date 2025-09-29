#[cfg(test)]
pub mod tests {
    use machine::memory::memory::Memory;

    #[test]
    fn allocate() {
        let mut mem = Memory::new(128).unwrap();
        let ptr = mem.allocate(8).unwrap();
        mem.write_u64(ptr.address, 123123123123).unwrap();
        assert_eq!(mem.read_u64(ptr.address).unwrap(), 123123123123);
    }

    #[test]
    fn deallocate() {
        let mut mem = Memory::new(128).unwrap();
        let ptr = mem.allocate(8).unwrap();
        mem.deallocate(&ptr).unwrap();
    }

    #[test]
    #[should_panic]
    fn allocate_large() {
        let mut mem = Memory::new(128).unwrap();
        mem.allocate(256).unwrap();
    }

    #[test]
    #[should_panic]
    fn deallocate_invalid() {
        let mut mem = Memory::new(128).unwrap();
        let ptr = mem.allocate(8).unwrap();
        mem.deallocate(&ptr).unwrap();
        mem.deallocate(&ptr).unwrap();
    }

    #[test]
    fn allocate_series() {
        let mut mem = Memory::new(128).unwrap();
        let ptr1 = mem.allocate(8).unwrap();
        let ptr2 = mem.allocate(6).unwrap();
        let ptr3 = mem.allocate(10).unwrap();
        assert_eq!(ptr1.address, 0);
        assert_eq!(ptr2.address, 8);
        assert_eq!(ptr3.address, 14);
    }

    #[test]
    fn allocate_series_gap() {
        let mut mem = Memory::new(128).unwrap();
        let ptr1 = mem.allocate(8).unwrap();
        let ptr2 = mem.allocate(6).unwrap();
        let ptr3 = mem.allocate(10).unwrap();
        mem.deallocate(&ptr2).unwrap();
        let ptr4 = mem.allocate(3).unwrap();
        assert_eq!(ptr1.address, 0);
        assert_eq!(ptr3.address, 14);
        assert_eq!(ptr4.address, 8);
    }

    #[test]
    fn allocate_series_start_gap() {
        let mut mem = Memory::new(128).unwrap();
        let ptr1 = mem.allocate(8).unwrap();
        let ptr2 = mem.allocate(6).unwrap();
        let ptr3 = mem.allocate(10).unwrap();
        mem.deallocate(&ptr1).unwrap();
        let ptr4 = mem.allocate(2).unwrap();
        assert_eq!(ptr1.address, 0);
        assert_eq!(ptr2.address, 8);
        assert_eq!(ptr3.address, 14);
        assert_eq!(ptr4.address, 0);
    }

    #[test]
    fn reallocate() {
        let mut mem = Memory::new(128).unwrap();
        let ptr1 = mem.allocate(4).unwrap();
        let _ptr2 = mem.allocate(4).unwrap();
        mem.write_u16(ptr1.address, u16::MAX).unwrap();
        let ptr1 = mem.reallocate(&ptr1, 8).unwrap();
        assert_eq!(mem.read_u16(ptr1.address).unwrap(), u16::MAX);
        assert_eq!(ptr1.address, 8);
    }

    #[test]
    fn reallocate_same_size() {
        let mut mem = Memory::new(128).unwrap();
        let ptr1 = mem.allocate(4).unwrap();
        let _ptr2 = mem.allocate(4).unwrap();
        mem.write_u16(ptr1.address, u16::MAX).unwrap();
        let ptr1 = mem.reallocate(&ptr1, 4).unwrap();
        assert_eq!(mem.read_u16(ptr1.address).unwrap(), u16::MAX);
        assert_eq!(ptr1.address, 0);
    }

    #[test]
    #[should_panic]
    fn reallocate_large() {
        let mut mem = Memory::new(128).unwrap();
        let ptr1 = mem.allocate(4).unwrap();
        let _ptr2 = mem.allocate(4).unwrap();
        mem.reallocate(&ptr1, 256).unwrap();
    }
}