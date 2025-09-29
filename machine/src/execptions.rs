#[derive(Debug)]
pub enum MachineError {
    InvalidAddress(Option<String>),
    InsufficientMemory(Option<String>),
    InvalidPointer(Option<String>),
    StackOverflow,
    StackUnderflow,
    InvalidRegister,
}