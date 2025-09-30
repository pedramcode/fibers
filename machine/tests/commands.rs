#[cfg(test)]
pub mod tests {
    use machine::{fiber::{fiber::{Fiber, Flag, Reg}}, memory::memory::Memory, opcode::commands};

    #[test]
    fn pushpop() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 10).unwrap();
        commands::push(&mut mem, &mut f, 11).unwrap();
        commands::push(&mut mem, &mut f, 12).unwrap();
        commands::pop(&mut mem, &mut f, Reg::R0).unwrap();
        commands::pop(&mut mem, &mut f, Reg::R1).unwrap();
        commands::pop(&mut mem, &mut f,Reg::R2).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 12);
        assert_eq!(f.get_register(&mem, Reg::R1).unwrap(), 11);
        assert_eq!(f.get_register(&mem, Reg::R2).unwrap(), 10);
    }

    #[test]
    fn add() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 10).unwrap();
        commands::push(&mut mem, &mut f, 5).unwrap();
        commands::add(&mut mem, &mut f).unwrap();
        commands::pop(&mut mem, &mut f, Reg::R0).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 15);
    }

    #[test]
    fn sub() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 3).unwrap();
        commands::push(&mut mem, &mut f, 6).unwrap();
        commands::sub(&mut mem, &mut f).unwrap();
        commands::pop(&mut mem, &mut f, Reg::R0).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 3);
        assert_eq!(f.get_flag(&mem, Flag::Negative).unwrap(), false);
    }

    #[test]
    fn mov() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::mov(&mut mem, &mut f, Reg::R0, 1998).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 1998);
    }

    #[test]
    fn sub_neg() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 6).unwrap();
        commands::push(&mut mem, &mut f, 3).unwrap();
        commands::sub(&mut mem, &mut f).unwrap();
        assert_eq!(f.get_flag(&mem, Flag::Negative).unwrap(), true);
    }

    #[test]
    fn dup() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 6).unwrap();
        commands::dup(&mut mem, &mut f).unwrap();
        commands::pop(&mut mem, &mut f, Reg::R0).unwrap();
        commands::pop(&mut mem, &mut f, Reg::R1).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 6);
        assert_eq!(f.get_register(&mem, Reg::R1).unwrap(), 6);
    }

    #[test]
    #[should_panic]
    fn dup_empty() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::dup(&mut mem, &mut f).unwrap();
    }

    #[test]
    fn drop() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 1).unwrap();
        commands::push(&mut mem, &mut f, 2).unwrap();
        commands::drop(&mut mem, &mut f).unwrap();
        commands::pop(&mut mem, &mut f, Reg::R0).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 1);
    }

    #[test]
    #[should_panic]
    fn drop_empty() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::drop(&mut mem, &mut f).unwrap();
    }

    #[test]
    fn swap() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 1).unwrap();
        commands::push(&mut mem, &mut f, 2).unwrap();
        commands::swap(&mut mem, &mut f).unwrap();
        commands::pop(&mut mem, &mut f, Reg::R0).unwrap();
        commands::pop(&mut mem, &mut f, Reg::R1).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 1);
        assert_eq!(f.get_register(&mem, Reg::R1).unwrap(), 2);
    }

    #[test]
    fn incdec() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::inc(&mut mem, &f, Reg::R0).unwrap();
        commands::inc(&mut mem, &f, Reg::R0).unwrap();
        commands::inc(&mut mem, &f, Reg::R0).unwrap();
        commands::inc(&mut mem, &f, Reg::R0).unwrap();
        commands::inc(&mut mem, &f, Reg::R0).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 5);
        commands::dec(&mut mem, &f, Reg::R0).unwrap();
        commands::dec(&mut mem, &f, Reg::R0).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 3);
    }

    #[test]
    fn decovf() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::dec(&mut mem, &f, Reg::R0).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), u64::MAX);
    }
}