use crate::{execptions::MachineError, memory::{allocation::Pointer, memory::Memory}, utils};

#[derive(Debug)]
pub struct Registers {
    pc: Pointer,
    sp: Pointer,
    r0: Pointer,
    r1: Pointer,
    r2: Pointer,
    r3: Pointer,
    r4: Pointer,
    r5: Pointer,
    r6: Pointer,
    r7: Pointer,
}

#[derive(Debug)]
pub struct Fiber {
    pub(crate) id: Pointer,
    pub(crate) registers: Registers,
    pub(crate) flag: Pointer,
    pub(crate) stack: Pointer,
}

impl Fiber {
    pub fn new(mem: &mut Memory, rng: &mut Box<rand::prelude::ThreadRng>) -> Result<Self, MachineError> {
        let mut res = Self {
            flag: mem.allocate(1)?,
            registers: Registers { 
                pc: mem.allocate(8)?,
                sp: mem.allocate(8)?,
                r0: mem.allocate(8)?,
                r1: mem.allocate(8)?,
                r2: mem.allocate(8)?,
                r3: mem.allocate(8)?,
                r4: mem.allocate(8)?,
                r5: mem.allocate(8)?,
                r6: mem.allocate(8)?,
                r7: mem.allocate(8)?,
            },
            stack: mem.allocate(8 * 1024)?, // TODO are you sure 8 KB for each fiber?
            id: mem.allocate(8)?,
        };
        res.set_register(mem, 101, 0)?;
        mem.write_u64(res.id.address, utils::random::random_fiber_id(rng))?;
        Ok(res)
    }

    pub fn get_id(&self, mem: &Memory) -> Result<u64, MachineError> {
        mem.read_u64(self.id.address)
    }

    pub fn kill(&self, mem: &mut Memory) -> Result<(), MachineError> {
        // deallocate registers
        mem.deallocate(&self.registers.pc)?;
        mem.deallocate(&self.registers.sp)?;
        mem.deallocate(&self.registers.r0)?;
        mem.deallocate(&self.registers.r1)?;
        mem.deallocate(&self.registers.r2)?;
        mem.deallocate(&self.registers.r3)?;
        mem.deallocate(&self.registers.r4)?;
        mem.deallocate(&self.registers.r5)?;
        mem.deallocate(&self.registers.r6)?;
        mem.deallocate(&self.registers.r7)?;
        // dellocate other
        mem.deallocate(&self.stack)?;
        mem.deallocate(&self.flag)?;
        mem.deallocate(&self.id)?;

        Ok(())
    }

    pub fn set_register(&mut self, mem: &mut Memory, reg: u8, val: u64) -> Result<(), MachineError> {
        match reg {
            0 => mem.write_u64(self.registers.r0.address, val),
            1 => mem.write_u64(self.registers.r1.address, val),
            2 => mem.write_u64(self.registers.r2.address, val),
            3 => mem.write_u64(self.registers.r3.address, val),
            4 => mem.write_u64(self.registers.r4.address, val),
            5 => mem.write_u64(self.registers.r5.address, val),
            6 => mem.write_u64(self.registers.r6.address, val),
            7 => mem.write_u64(self.registers.r7.address, val),
            100 => mem.write_u64(self.registers.pc.address, val),
            101 => mem.write_u64(self.registers.sp.address, val),
            _ => Err(MachineError::InvalidRegister)
        }
    }

    pub fn get_register(&mut self, mem: &Memory, reg: u8) -> Result<u64, MachineError> {
        match reg {
            0 => mem.read_u64(self.registers.r0.address),
            1 => mem.read_u64(self.registers.r1.address),
            2 => mem.read_u64(self.registers.r2.address),
            3 => mem.read_u64(self.registers.r3.address),
            4 => mem.read_u64(self.registers.r4.address),
            5 => mem.read_u64(self.registers.r5.address),
            6 => mem.read_u64(self.registers.r6.address),
            7 => mem.read_u64(self.registers.r7.address),
            100 => mem.read_u64(self.registers.pc.address),
            101 => mem.read_u64(self.registers.sp.address),
            _ => Err(MachineError::InvalidRegister)
        }
    }
}
