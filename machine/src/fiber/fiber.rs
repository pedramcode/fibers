use crate::{execptions::MachineError, fiber::section::Section, memory::{allocation::Pointer, memory::Memory}, utils};

#[derive(Debug, Clone)]
pub enum Reg {
    PC, SP,
    R0, R1, R2, R3, R4, R5, R6, R7,
}

#[derive(Debug)]
pub enum Flag {
    Zero, Overflow, Negative, Carry,
}

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
    pub(crate) text_section: Section,
    pub(crate) data_section: Section,
}

impl Fiber {
    pub fn new(mem: &mut Memory, rng: &mut Box<rand::prelude::ThreadRng>) -> Result<Self, MachineError> {
        let res = Self {
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
            stack: mem.allocate(4 * 1024)?,
            id: mem.allocate(8)?,
            text_section: Section::new(mem)?,
            data_section: Section::new(mem)?,
        };
        res.set_register(mem, Reg::PC, 0)?;
        res.set_register(mem, Reg::SP, 0)?;
        mem.write_u64(res.id.address, utils::random::random_fiber_id(rng))?;
        mem.write_u8(res.flag.address, 0)?;
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
        // deallocate sections
        self.text_section.free(mem)?;
        self.data_section.free(mem)?;
        // dellocate other
        mem.deallocate(&self.stack)?;
        mem.deallocate(&self.flag)?;
        mem.deallocate(&self.id)?;

        Ok(())
    }

    pub fn set_register(&self, mem: &mut Memory, reg: Reg, val: u64) -> Result<(), MachineError> {
        match reg {
            Reg::R0 => mem.write_u64(self.registers.r0.address, val),
            Reg::R1 => mem.write_u64(self.registers.r1.address, val),
            Reg::R2 => mem.write_u64(self.registers.r2.address, val),
            Reg::R3 => mem.write_u64(self.registers.r3.address, val),
            Reg::R4 => mem.write_u64(self.registers.r4.address, val),
            Reg::R5 => mem.write_u64(self.registers.r5.address, val),
            Reg::R6 => mem.write_u64(self.registers.r6.address, val),
            Reg::R7 => mem.write_u64(self.registers.r7.address, val),
            Reg::PC => mem.write_u64(self.registers.pc.address, val),
            Reg::SP => mem.write_u64(self.registers.sp.address, val),
        }
    }

    pub fn get_register(&self, mem: &Memory, reg: Reg) -> Result<u64, MachineError> {
        match reg {
            Reg::R0 => mem.read_u64(self.registers.r0.address),
            Reg::R1 => mem.read_u64(self.registers.r1.address),
            Reg::R2 => mem.read_u64(self.registers.r2.address),
            Reg::R3 => mem.read_u64(self.registers.r3.address),
            Reg::R4 => mem.read_u64(self.registers.r4.address),
            Reg::R5 => mem.read_u64(self.registers.r5.address),
            Reg::R6 => mem.read_u64(self.registers.r6.address),
            Reg::R7 => mem.read_u64(self.registers.r7.address),
            Reg::PC => mem.read_u64(self.registers.pc.address),
            Reg::SP => mem.read_u64(self.registers.sp.address),
        }
    }

    pub fn get_flag(&self, mem: &Memory, flag: Flag) -> Result<bool, MachineError> {
        let flags = mem.read_u8(self.flag.address)?;

        let bit = match flag {
            Flag::Zero => 0,
            Flag::Overflow => 1,
            Flag::Negative => 2,
            Flag::Carry => 3,
        };

        Ok((flags >> bit) & 1 == 1)
    }

    pub fn set_flag(&self, mem: &mut Memory, flag: Flag, value: bool) -> Result<(), MachineError> {
        let mut flags = mem.read_u8(self.flag.address)?;

        let bit = match flag {
            Flag::Zero => 0,
            Flag::Overflow => 1,
            Flag::Negative => 2,
            Flag::Carry => 3,
        };

        if value {
            flags |= 1 << bit;
        } else {
            flags &= !(1 << bit);
        }

        mem.write_u8(self.flag.address, flags)?;

        Ok(())
    }
}
