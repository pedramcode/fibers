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