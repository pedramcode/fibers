use crate::{execptions::MachineError, fiber::fiber::Fiber, memory::memory::Memory};

impl Fiber {
    pub fn push(&mut self, mem: &mut Memory, data: u64) -> Result<(), MachineError> {
        if self.get_register(mem, 101)? as usize > self.stack.size {
            if self.stack.size > 256 * 1024 {
                return Err(MachineError::StackOverflow);
            }
            let prev_stack = self.stack.clone();
            self.stack = mem.reallocate(&prev_stack, prev_stack.size + 64)?;
            mem.deallocate(&prev_stack)?;
        }
        mem.write_u64(self.stack.address + self.get_register(mem, 101)? as usize, data)?;
        let r = self.get_register(mem, 101)?;
        self.set_register(mem, 101, r + 8)?;
        Ok(())
    }

    // TODO shrink stack if needed
    pub fn pop(&mut self, mem: &mut Memory) -> Result<u64, MachineError> {
        if self.get_register(mem, 101)? as usize == 0 {
            return Err(MachineError::StackUnderflow);
        }
        let val = mem.read_u64(self.stack.address + (self.get_register(mem, 101)? as usize - 8))?;
        let r = self.get_register(mem, 101)?;
        self.set_register(mem, 101, r - 8)?;
        Ok(val)
    }

    pub fn swap(&mut self, mem: &mut Memory) -> Result<(), MachineError> {
        let v1 = self.pop(mem)?;
        let v2 = self.pop(mem)?;
        self.push(mem, v1)?;
        self.push(mem, v2)?;
        Ok(())
    }
}