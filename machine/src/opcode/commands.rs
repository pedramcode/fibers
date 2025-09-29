use crate::{execptions::MachineError, fiber::fiber::{Fiber, Reg}, memory::memory::Memory};

pub fn push(mem: &mut Memory, fib: &mut Fiber, value: u64) -> Result<(), MachineError> {
    fib.push(mem, value)
}

pub fn pop(mem: &mut Memory, fib: &mut Fiber, reg: Reg) -> Result<(), MachineError> {
    let val = fib.pop(mem)?;
    fib.set_register(mem, reg, val)
}

pub fn mov(mem: &mut Memory, fib: &Fiber, reg: Reg, num: u64) -> Result<(), MachineError> {
    fib.set_register(mem, reg, num)
}

pub fn add(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = fib.pop(mem)?;
    let c = a + b;
    fib.push(mem, c)
}

pub fn sub(mem: &mut Memory, fib: &mut Fiber) -> Result<(), MachineError> {
    let a = fib.pop(mem)?;
    let b = fib.pop(mem)?;
    let c = a - b;
    fib.push(mem, c)
}