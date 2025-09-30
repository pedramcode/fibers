#[cfg(test)]
pub mod tests {
    use std::u64;

    use machine::{fiber::fiber::Fiber, memory::memory::Memory};

    #[test]
    fn pushpop() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.push(&mut mem, u64::MAX).unwrap();
        assert_eq!(f.pop(&mut mem).unwrap(), u64::MAX);
    }

    #[test]
    fn pushpop_series() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.push(&mut mem, 1).unwrap();
        f.push(&mut mem, 2).unwrap();
        f.push(&mut mem, 3).unwrap();
        f.push(&mut mem, 4).unwrap();
        f.push(&mut mem, 5).unwrap();
        assert_eq!(f.pop(&mut mem).unwrap(), 5);
        assert_eq!(f.pop(&mut mem).unwrap(), 4);
        assert_eq!(f.pop(&mut mem).unwrap(), 3);
        assert_eq!(f.pop(&mut mem).unwrap(), 2);
        assert_eq!(f.pop(&mut mem).unwrap(), 1);
    }

    #[test]
    fn peek() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.push(&mut mem, 1).unwrap();
        f.push(&mut mem, 2).unwrap();
        assert_eq!(f.peek(&mut mem).unwrap(), 2);
        assert_eq!(f.peek(&mut mem).unwrap(), 2);
        assert_eq!(f.peek(&mut mem).unwrap(), 2);
        assert_eq!(f.pop(&mut mem).unwrap(), 2);
        assert_eq!(f.peek(&mut mem).unwrap(), 1);
    }

    #[test]
    #[should_panic]
    fn peek_empty() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.peek(&mut mem).unwrap();
    }

    #[test]
    #[should_panic]
    fn stackoverflow() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        for _ in 0..10000000 {
            f.push(&mut mem, 1).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn stackunderflow() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.pop(&mut mem).unwrap();
    }

    #[test]
    fn swap() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.push(&mut mem, 1).unwrap();
        f.push(&mut mem, 2).unwrap();
        assert_eq!(f.pop(&mut mem).unwrap(), 2);
        assert_eq!(f.pop(&mut mem).unwrap(), 1);

        f.push(&mut mem, 3).unwrap();
        f.push(&mut mem, u64::MAX).unwrap();
        f.swap(&mut mem).unwrap();
        assert_eq!(f.pop(&mut mem).unwrap(), 3);
        assert_eq!(f.pop(&mut mem).unwrap(), u64::MAX);
    }

    #[test]
    #[should_panic]
    fn swap_empty() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.swap(&mut mem).unwrap();
    }

    #[test]
    #[should_panic]
    fn swap_one() {
        let mut mem = Memory::new(8 * 1024 * 1024).unwrap();
        let mut rng = Box::new(rand::rng());
        let mut f = Fiber::new(&mut mem, &mut rng).unwrap();
        f.push(&mut mem, 1).unwrap();
        f.swap(&mut mem).unwrap();
    }
}