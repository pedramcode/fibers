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
}