use crate::{execptions::MachineError, fiber::fiber::{Fiber, Flag, Reg}, memory::memory::Memory};

pub fn push(mem: &mut Memory, fib: &mut Fiber, value: u64) -> Result<(), MachineError> {
    fib.push(mem, value)
}

pub fn pop(mem: &mut Memory, fib: &mut Fiber, reg: Reg) -> Result<(), MachineError> {
    let val = fib.pop(mem)?;
    fib.set_flag(mem, Flag::Zero, val == 0)?;
    fib.set_register(mem, reg, val)
}

pub fn mov(mem: &mut Memory, fib: &Fiber, reg: Reg, num: u64) -> Result<(), MachineError> {
    fib.set_register(mem, reg, num)
}

pub fn add(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)? as i64;
    let b = fib.pop(mem)? as i64;
    let c = a.wrapping_add(b);
    fib.set_flag(mem, Flag::Zero, c == 0)?;
    fib.set_flag(mem, Flag::Negative, c < 0)?;
    fib.set_flag(mem, Flag::Overflow, (a > 0 && b > 0 && c < 0) || (a < 0 && b < 0 && c > 0))?;
    let carry = (a as u64).overflowing_add(b as u64).1;
    fib.set_flag(mem, Flag::Carry, carry)?;
    fib.push(mem, c as u64)
}

pub fn sub(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)? as i64;
    let b = fib.pop(mem)? as i64;
    let c = a.wrapping_sub(b);
    fib.set_flag(mem, Flag::Zero, c == 0)?;
    fib.set_flag(mem, Flag::Negative, c < 0)?;
    fib.set_flag(mem, Flag::Overflow, (a > 0 && b < 0 && c < 0) || (a < 0 && b > 0 && c > 0))?;
    let borrow = (a as u64).overflowing_sub(b as u64).1;
    fib.set_flag(mem, Flag::Carry, !borrow)?;
    fib.push(mem, c as u64)
}

pub fn drop(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let val = fib.pop(mem)?;
    fib.set_flag(mem, Flag::Zero, val == 0)?;
    Ok(())
}

pub fn dup(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let val = fib.peek(mem)?;
    fib.push(mem, val)
}

pub fn swap(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    fib.swap(mem)
}

pub fn inc(mem: &mut Memory, fib: &Fiber, reg: Reg) -> Result<(), MachineError> {
    let val = fib.get_register(mem, reg.clone())?;
    let res = val.wrapping_add(1);
    fib.set_register(mem, reg, res)
}

pub fn dec(mem: &mut Memory, fib: &Fiber, reg: Reg) -> Result<(), MachineError> {
    let val = fib.get_register(mem, reg.clone())?;
    let res = val.wrapping_sub(1);
    fib.set_register(mem, reg, res)
}

pub fn jmp(mem: &mut Memory, fib: &Fiber, address: usize) -> Result<(), MachineError> {
    if address < fib.text_section.data.address || address >= fib.text_section.data.address + fib.text_section.data.size {
        return Err(MachineError::InvalidAddress(None));
    }
    fib.set_register(mem, Reg::PC, address as u64)
}

pub fn jnz(mem: &mut Memory, fib: &Fiber, address: usize) -> Result<(), MachineError> {
    if address < fib.text_section.data.address || address >= fib.text_section.data.address + fib.text_section.data.size {
        return Err(MachineError::InvalidAddress(None));
    }
    if !fib.get_flag(mem, Flag::Zero)? {
        return fib.set_register(mem, Reg::PC, address as u64);
    }
    Ok(())
}

pub fn jz(mem: &mut Memory, fib: &Fiber, address: usize) -> Result<(), MachineError> {
    if address < fib.text_section.data.address || address >= fib.text_section.data.address + fib.text_section.data.size {
        return Err(MachineError::InvalidAddress(None));
    }
    if fib.get_flag(mem, Flag::Zero)? {
        return fib.set_register(mem, Reg::PC, address as u64);
    }
    Ok(())
}

pub fn jg(mem: &mut Memory, fib: &Fiber, address: usize) -> Result<(), MachineError> {
    if address < fib.text_section.data.address || address >= fib.text_section.data.address + fib.text_section.data.size {
        return Err(MachineError::InvalidAddress(None));
    }
    if !fib.get_flag(mem, Flag::Zero)? && (fib.get_flag(mem, Flag::Negative)? == fib.get_flag(mem, Flag::Overflow)?) {
        return fib.set_register(mem, Reg::PC, address as u64);
    }
    Ok(())
}

pub fn jge(mem: &mut Memory, fib: &Fiber, address: usize) -> Result<(), MachineError> {
    if address < fib.text_section.data.address || address >= fib.text_section.data.address + fib.text_section.data.size {
        return Err(MachineError::InvalidAddress(None));
    }
    if fib.get_flag(mem, Flag::Negative)? == fib.get_flag(mem, Flag::Overflow)? {
        return fib.set_register(mem, Reg::PC, address as u64);
    }
    Ok(())
}

pub fn jl(mem: &mut Memory, fib: &Fiber, address: usize) -> Result<(), MachineError> {
    if address < fib.text_section.data.address || address >= fib.text_section.data.address + fib.text_section.data.size {
        return Err(MachineError::InvalidAddress(None));
    }
    if fib.get_flag(mem, Flag::Negative)? != fib.get_flag(mem, Flag::Overflow)? {
        return fib.set_register(mem, Reg::PC, address as u64);
    }
    Ok(())
}

pub fn jle(mem: &mut Memory, fib: &Fiber, address: usize) -> Result<(), MachineError> {
    if address < fib.text_section.data.address || address >= fib.text_section.data.address + fib.text_section.data.size {
        return Err(MachineError::InvalidAddress(None));
    }
    if fib.get_flag(mem, Flag::Zero)? || (fib.get_flag(mem, Flag::Negative)? != fib.get_flag(mem, Flag::Overflow)?) {
        return fib.set_register(mem, Reg::PC, address as u64);
    }
    Ok(())
}

pub fn and(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = fib.pop(mem)?;
    let c = a & b;
    fib.push(mem, c)
}

pub fn or(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = fib.pop(mem)?;
    let c = a | b;
    fib.push(mem, c)
}

pub fn not(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = !a;
    fib.push(mem, b)
}

pub fn xor(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = fib.pop(mem)?;
    let c = a ^ b;
    fib.push(mem, c)
}

pub fn shl(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = fib.pop(mem)?;
    let c = a << b;
    fib.push(mem, c)
}

pub fn shr(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = fib.pop(mem)?;
    let c = a >> b;
    fib.push(mem, c)
}

pub fn rol(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = fib.pop(mem)?;
    let c = a.rotate_left(b as u32);
    fib.push(mem, c)
}

pub fn ror(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = fib.pop(mem)?;
    let c = a.rotate_right(b as u32);
    fib.push(mem, c)
}