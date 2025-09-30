use crate::{execptions::MachineError, fiber::fiber::Fiber, memory::memory::Memory};

pub struct Machine {
    mem: Memory,
    rng: Box<rand::prelude::ThreadRng>,
    fibers: Vec<Fiber>,
}

impl Machine {
    pub fn new(size: usize) -> Result<Self, MachineError> {
        Ok(Self{
            fibers: Vec::new(),
            mem: Memory::new(size)?,
            rng: Box::new(rand::rng()),
        })
    }

    pub fn spawn(&mut self) -> Result<u64, MachineError> {
        let fib = Fiber::new(&mut self.mem, &mut self.rng)?;
        let id = fib.get_id(&self.mem)?;
        self.fibers.push(fib);
        Ok(id)
    }

    pub fn kill(&mut self, fiber_id: u64) -> Result<(), MachineError> {
        if let Some(idx) = self.fibers.iter().position(|x| {
            if let Ok(id) = x.get_id(&self.mem) {
                return id == fiber_id;
            } else {
                false
            }
        }) {
            self.fibers[idx].kill(&mut self.mem)?;
            self.fibers.swap_remove(idx);
        }
        Ok(())
    }
}
