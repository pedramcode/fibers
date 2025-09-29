#[cfg(test)]
pub mod tests {
    use machine::{fiber::fiber::{Fiber, Reg}, memory::memory::Memory, opcode::commands};

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
    }

    #[test]
    fn mov() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::mov(&mut mem, &mut f, Reg::R0, 1998).unwrap();
        assert_eq!(f.get_register(&mem, Reg::R0).unwrap(), 1998);
    }
}