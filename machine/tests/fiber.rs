#[cfg(test)]
pub mod tests {
    use machine::{fiber::fiber::{Fiber, Flag}, memory::memory::Memory};

    #[test]
    fn initialize() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.kill(&mut mem).unwrap();
        let f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.kill(&mut mem).unwrap();
        let f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.kill(&mut mem).unwrap();
        let f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.kill(&mut mem).unwrap();
        let f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.kill(&mut mem).unwrap();
        let f1 = Fiber::new(&mut mem, &mut rng).unwrap();
        let f2 = Fiber::new(&mut mem, &mut rng).unwrap();
        let f3 = Fiber::new(&mut mem, &mut rng).unwrap();
        let f4 = Fiber::new(&mut mem, &mut rng).unwrap();
        f1.kill(&mut mem).unwrap();
        f2.kill(&mut mem).unwrap();
        f3.kill(&mut mem).unwrap();
        f4.kill(&mut mem).unwrap();
    }

    #[test]
    #[ignore]
    fn brute_force() {
        let mut mem = Memory::new(128 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        for _ in 0..10000 {
            let f = Fiber::new(&mut mem, &mut rng).unwrap();
            f.kill(&mut mem).unwrap();
        }
    }

    #[test]
    fn flags() {
        let mut mem = Memory::new(128 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let f = Fiber::new(&mut mem, &mut rng).unwrap();
        assert_eq!(f.get_flag(&mem, Flag::Zero).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Carry).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Negative).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Overflow).unwrap(), false);

        f.set_flag(&mut mem, Flag::Zero, true).unwrap();
        assert_eq!(f.get_flag(&mem, Flag::Zero).unwrap(), true);
        assert_eq!(f.get_flag(&mem, Flag::Carry).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Negative).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Overflow).unwrap(), false);

        f.set_flag(&mut mem, Flag::Overflow, true).unwrap();
        assert_eq!(f.get_flag(&mem, Flag::Zero).unwrap(), true);
        assert_eq!(f.get_flag(&mem, Flag::Carry).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Negative).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Overflow).unwrap(), true);

        f.set_flag(&mut mem, Flag::Zero, false).unwrap();
        assert_eq!(f.get_flag(&mem, Flag::Zero).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Carry).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Negative).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Overflow).unwrap(), true);
        
        f.set_flag(&mut mem, Flag::Carry, true).unwrap();
        f.set_flag(&mut mem, Flag::Negative, true).unwrap();
        assert_eq!(f.get_flag(&mem, Flag::Zero).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Carry).unwrap(), true);
        assert_eq!(f.get_flag(&mem, Flag::Negative).unwrap(), true);
        assert_eq!(f.get_flag(&mem, Flag::Overflow).unwrap(), true);

        f.set_flag(&mut mem, Flag::Negative, false).unwrap();
        assert_eq!(f.get_flag(&mem, Flag::Zero).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Carry).unwrap(), true);
        assert_eq!(f.get_flag(&mem, Flag::Negative).unwrap(), false);
        assert_eq!(f.get_flag(&mem, Flag::Overflow).unwrap(), true);
    }
}