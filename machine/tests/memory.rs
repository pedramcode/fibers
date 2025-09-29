#[cfg(test)]
pub mod tests {
    use machine::memory::memory::Memory;

    #[test]
    fn initialize() {
        Memory::new(32).unwrap();
    }
}