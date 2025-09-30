use core::panic;

use crate::{execptions::MachineError, fiber::fiber::{Fiber, FiberState}, memory::memory::Memory};

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

    pub fn write_bytecodes(&mut self, fiber_id: u64, bytecodes: &[u64]) -> Result<(), MachineError> {
        if let Some(idx) = self.fibers.iter().position(|x| {
            if let Ok(fid) = x.get_id(&self.mem) {
                fid == fiber_id
            } else { false }
        }) {
            for pair in bytecodes.chunks(2) {
                match pair[0] {
                    0 => self.fibers[idx].data_section.append_data::<u8>(&mut self.mem, pair[1] as u8)?,
                    1 => self.fibers[idx].data_section.append_data::<u16>(&mut self.mem, pair[1] as u16)?,
                    2 => self.fibers[idx].data_section.append_data::<u32>(&mut self.mem, pair[1] as u32)?,
                    3 => self.fibers[idx].data_section.append_data::<u64>(&mut self.mem, pair[1] as u64)?,
                    _ => return Err(MachineError::InvalidBytecodeDataType),
                };
            }
            Ok(())
        } else {
            return Err(MachineError::InvalidFiber);
        }
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

    pub fn execute(&mut self) -> Result<(), MachineError> {
        Ok(loop {
            // TODO only for testing it'll break, reomve it later
            if self.fibers.len() == 0 {
                break;
            }
            let mut kills: Vec<u64> = Vec::new();
            for fiber in &mut self.fibers {
                let res = fiber.execute(&mut self.mem);
                if let Ok(()) = res {
                    if fiber.get_state(&self.mem)? == FiberState::HALTED {
                        let id = fiber.get_id(&self.mem)?;
                        kills.push(id);
                    }
                } else if let Err(err) = res {
                    // TODO somehow log the error and manage it, for now, PANIC!
                    let id = fiber.get_id(&self.mem)?;
                    kills.push(id);
                    panic!("{:?}", err);
                }
            }
            for id in kills {
                self.kill(id)?;
            }
        })
    }
}
