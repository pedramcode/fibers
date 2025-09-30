#[cfg(test)]
pub mod tests {
    use machine::machine::machine::Machine;


    #[test]
    fn initialize() {
        let _machine = Machine::new(16 * 1024 * 1024).unwrap();
    }

    #[test]
    fn spawn() {
        let mut machine = Machine::new(16 * 1024 * 1024).unwrap();
        let f1 = machine.spawn().unwrap();
        let f2 = machine.spawn().unwrap();
        let f3 = machine.spawn().unwrap();
        let f4 = machine.spawn().unwrap();
        machine.kill(f1).unwrap();
        machine.kill(f2).unwrap();
        machine.kill(f3).unwrap();
        machine.kill(f4).unwrap();
    }

    #[test]
    #[ignore]
    fn spawn_many() {
        let mut machine = Machine::new(128 * 1024 * 1024).unwrap();
        let mut fibers: Vec<u64> = Vec::new();
        let range = 0..1000;
        for _ in range.clone() {
            let id = machine.spawn().unwrap();
            fibers.push(id);
        }
        for i in range.clone() {
            machine.kill(fibers[i]).unwrap();
        }
    }
}