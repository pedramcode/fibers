use std::ops::Range;

use crate::execptions::MachineError;

#[derive(Debug)]
pub struct Memory {
    pub(crate) data: Vec<u8>,
    pub(crate) blocks: Vec<Range<usize>>,
}

impl Memory {
    pub fn new(size: usize) -> Result<Self, MachineError> {
        Ok(Self {
            data: vec![0u8; size],
            blocks: Vec::new(),
        })
    }
}