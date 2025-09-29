#[cfg(test)]
pub mod tests {
    use machine::{fiber::{fiber::Fiber}, memory::memory::Memory, opcode::commands};

    #[test]
    fn pushpop() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 10).unwrap();
        commands::push(&mut mem, &mut f, 11).unwrap();
        commands::push(&mut mem, &mut f, 12).unwrap();
        commands::pop(&mut mem, &mut f, 0).unwrap();
        commands::pop(&mut mem, &mut f, 1).unwrap();
        commands::pop(&mut mem, &mut f,2).unwrap();
        assert_eq!(f.get_register(&mem, 0).unwrap(), 12);
        assert_eq!(f.get_register(&mem, 1).unwrap(), 11);
        assert_eq!(f.get_register(&mem, 2).unwrap(), 10);
    }

    #[test]
    fn add() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 10).unwrap();
        commands::push(&mut mem, &mut f, 5).unwrap();
        commands::add(&mut mem, &mut f).unwrap();
        commands::pop(&mut mem, &mut f, 0).unwrap();
        assert_eq!(f.get_register(&mem, 0).unwrap(), 15);
    }

    #[test]
    fn sub() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        commands::push(&mut mem, &mut f, 3).unwrap();
        commands::push(&mut mem, &mut f, 6).unwrap();
        commands::sub(&mut mem, &mut f).unwrap();
        commands::pop(&mut mem, &mut f, 0).unwrap();
        assert_eq!(f.get_register(&mem, 0).unwrap(), 3);
    }
}