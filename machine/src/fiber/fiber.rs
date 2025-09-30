use crate::{execptions::MachineError, fiber::section::Section, memory::{self, allocation::Pointer, memory::Memory}, opcode::{commands, opcodes::Opcodes}, utils};

#[derive(Debug, Clone)]
pub enum Reg {
    PC, SP,
    R0, R1, R2, R3, R4, R5, R6, R7,
}

impl Reg {
    pub fn from_u8(val: u8) -> Result<Self, MachineError> {
        match val {
            0 => Ok(Self::R0),
            1 => Ok(Self::R1),
            2 => Ok(Self::R2),
            3 => Ok(Self::R3),
            4 => Ok(Self::R4),
            5 => Ok(Self::R5),
            6 => Ok(Self::R6),
            7 => Ok(Self::R7),
            100 => Ok(Self::PC),
            101 => Ok(Self::SP),
            _ => Err(MachineError::InvalidRegister),
        }
    }
}

#[derive(Debug)]
pub enum Flag {
    Zero, Overflow, Negative, Carry,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FiberState {
    RUNNING = 0x00,
    HALTED = 0x01,
    BLOCKED = 0x02,
}

impl FiberState {
    pub fn from_u8(val: u8) -> Result<Self, MachineError> {
        match val {
            0x00 => Ok(Self::RUNNING),
            0x01 => Ok(Self::HALTED),
            0x02 => Ok(Self::BLOCKED),
            _ => Err(MachineError::InvalidFiberState)
        }
    }
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
    pub(crate) state: Pointer,
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
            state: mem.allocate(1)?,
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

    pub fn set_state(&self, mem: &mut Memory, state: FiberState) -> Result<(), MachineError> {
        mem.write_u8(self.state.address, state as u8)
    }

    pub fn get_state(&self, mem: &Memory) -> Result<FiberState, MachineError> {
        let val = mem.read_u8(self.state.address)?;
        FiberState::from_u8(val)
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

    fn advance_pc(&self, mem: &mut Memory, step: u64) -> Result<(), MachineError> {
        let cur = self.get_register(&mem, Reg::PC)?;
        self.set_register(mem, Reg::PC, cur + step)
    }

    fn get_pc(&self, mem: &Memory) -> Result<u64, MachineError> {
        self.get_register(&mem, Reg::PC)
    }

    pub fn execute(&mut self, mem: &mut Memory) -> Result<(), MachineError> {
        self.set_state(mem, FiberState::RUNNING)?;
        loop {
            let opcode_read = self.text_section.read_u16(mem, self.get_pc(mem)? as usize)?;
            self.advance_pc(mem, 2)?;
            let instr = Opcodes::try_from(opcode_read);
            if let Ok(opcode) = instr {
                match opcode {
                    Opcodes::PUSH => {
                        let val = self.text_section.read_u64(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 8)?;
                        println!("------------------------------------ {:b}", val);
                        commands::push(mem, self, val)?;
                    },
                    Opcodes::POP => {
                        let reg = self.text_section.read_u8(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 1)?;
                        commands::pop(mem, self, Reg::from_u8(reg)?)?;
                    },
                    Opcodes::MOV => {
                        let reg = self.text_section.read_u8(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 1)?;
                        let val = self.text_section.read_u64(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 8)?;
                        commands::mov(mem, self, Reg::from_u8(reg)?, val)?;
                    },
                    Opcodes::ADD => {
                        commands::add(mem, self)?;
                    },
                    Opcodes::SUB => {
                        commands::sub(mem, self)?;
                    },
                    Opcodes::DROP => {
                        commands::drop(mem, self)?;
                    },
                    Opcodes::DUP => {
                        commands::dup(mem, self)?;
                    },
                    Opcodes::SWP => {
                        commands::swap(mem, self)?;
                    },
                    Opcodes::INC => {
                        let reg = self.text_section.read_u8(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 1)?;
                        commands::inc(mem, self, Reg::from_u8(reg)?)?;
                    },
                    Opcodes::DEC => {
                        let reg = self.text_section.read_u8(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 1)?;
                        commands::dec(mem, self, Reg::from_u8(reg)?)?;
                    },
                    Opcodes::JMP => {
                        let val = self.text_section.read_u64(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 8)?;
                        commands::jmp(mem, self, val as usize)?;
                    },
                    Opcodes::JZ => {
                        let val = self.text_section.read_u64(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 8)?;
                        commands::jz(mem, self, val as usize)?;
                    },
                    Opcodes::JNZ => {
                        let val = self.text_section.read_u64(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 8)?;
                        commands::jnz(mem, self, val as usize)?;
                    },
                    Opcodes::JG => {
                        let val = self.text_section.read_u64(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 8)?;
                        commands::jg(mem, self, val as usize)?;
                    },
                    Opcodes::JGE => {
                        let val = self.text_section.read_u64(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 8)?;
                        commands::jge(mem, self, val as usize)?;
                    },
                    Opcodes::JL => {
                        let val = self.text_section.read_u64(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 8)?;
                        commands::jl(mem, self, val as usize)?;
                    },
                    Opcodes::JLE => {
                        let val = self.text_section.read_u64(mem, self.get_pc(mem)? as usize)?;
                        self.advance_pc(mem, 8)?;
                        commands::jle(mem, self, val as usize)?;
                    },
                    Opcodes::AND => {
                        commands::and(mem, self)?;
                    },
                    Opcodes::OR => {
                        commands::or(mem, self)?;
                    },
                    Opcodes::NOT => {
                        commands::not(mem, self)?;
                    },
                    Opcodes::XOR => {
                        commands::xor(mem, self)?;
                    },
                    Opcodes::SHR => {
                        commands::shr(mem, self)?;
                    },
                    Opcodes::SHL => {
                        commands::shl(mem, self)?;
                    },
                    Opcodes::ROL => {
                        commands::rol(mem, self)?;
                    },
                    Opcodes::ROR => {
                        commands::ror(mem, self)?;
                    },
                    Opcodes::HLT => {
                        self.set_state(mem, FiberState::HALTED)?;
                        return Ok(());
                    },
                    Opcodes::YLD => {
                        self.set_state(mem, FiberState::BLOCKED)?;
                        return Ok(());
                    },
                }
            } else {
                self.set_state(mem, FiberState::HALTED)?;
                return Err(MachineError::InvalidOpcode(Some(format!("opcode: {} at #{:x}", opcode_read, self.get_pc(mem)?).to_string())));
            }
        }
    }
}
