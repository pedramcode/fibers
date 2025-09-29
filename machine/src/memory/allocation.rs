use std::ops::Range;

use crate::{execptions::MachineError, memory::memory::Memory, utils::normalize::normalize_size};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pointer {
    pub address: usize,
    pub size: usize,
}

impl Memory {
    // first-fit
    pub fn allocate(&mut self, size: usize) -> Result<Pointer, MachineError> {
        let size = normalize_size(size);
        self.blocks.sort_by_key(|x| x.start);

        if let Some(block) = self.blocks.first() {
            if block.start > 0 && block.start > size {
                self.blocks.push(0..size);
                self.set_zero(0..size)?;
                return Ok(Pointer{address: 0, size: size});
            }
        }

        let windows: Vec<_> = self.blocks.windows(2).map(|w| (w[0].end, w[1].start)).collect();
        for (end, start) in windows {
            if start - end > size {
                self.blocks.push(end..end + size);
                self.set_zero(end..end + size)?;
                return Ok(Pointer { address: end, size });
            }
        }

        if let Some(block) = self.blocks.last() {
            let block_end = block.end;
            if block_end + size < self.data.len() {
                self.blocks.push(block_end..block_end + size);
                self.set_zero(block_end..block_end + size)?;
                return Ok(Pointer{address: block_end, size: size});
            }
        } else {
            if self.blocks.len() == 0 && size < self.data.len() {
                self.blocks.push(0..size);
                self.set_zero(0..size)?;
                return Ok(Pointer { address: 0, size });
            }
        }

        Err(MachineError::InsufficientMemory(None))
    }

    fn set_zero(&mut self, range: Range<usize>) -> Result<(), MachineError> {
        if range.end > self.data.len() {
            return Err(MachineError::InsufficientMemory(None));
        }
        for idx in range {
            self.data[idx] = 0;
        }
        Ok(())
    }

    pub fn deallocate(&mut self, ptr: &Pointer) -> Result<(), MachineError> {
        if let Some(idx) = self.blocks.iter().position(|x| x.start == ptr.address) {
            self.blocks.swap_remove(idx);
            Ok(())
        } else {
            Err(MachineError::InvalidPointer(None))
        }
    }

    // TODO enable grow/shrink for better performance
    pub fn reallocate(&mut self, ptr: &Pointer, size: usize) -> Result<Pointer, MachineError> {
        if ptr.size == size {
            return Ok(ptr.clone());
        }
        let new_ptr = self.allocate(size)?;
        for idx in 0..size {
            self.data[new_ptr.address + idx] = self.data[ptr.address + idx];
        }
        self.deallocate(ptr)?;
        Ok(new_ptr)
    }
}