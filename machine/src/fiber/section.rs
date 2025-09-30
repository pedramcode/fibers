use crate::{execptions::MachineError, memory::{allocation::Pointer, memory::Memory}};

#[derive(Debug)]
pub struct Section {
    pub(crate) dp: Pointer, // data pointer
    pub(crate) data: Pointer,
}

pub trait MemoryMan {
    fn append_data(val: Self, mem: &mut Memory, address: usize) -> Result<(), MachineError>;
    fn read_data(mem: &Memory, address: usize) -> Result<Self, MachineError> where Self: Sized;
    fn size_in_bytes() -> usize;
}

impl MemoryMan for u8 {
    fn append_data(val: Self, mem: &mut Memory, address: usize) -> Result<(), MachineError> {
        mem.write_u8(address, val)
    }

    fn size_in_bytes() -> usize {
        1
    }
    
    fn read_data(mem: &Memory, address: usize) -> Result<Self, MachineError> {
        mem.read_u8(address)
    }
}

impl MemoryMan for u16 {
    fn append_data(val: Self, mem: &mut Memory, address: usize) -> Result<(), MachineError> {
        mem.write_u16(address, val)
    }

    fn size_in_bytes() -> usize {
        2
    }

    fn read_data(mem: &Memory, address: usize) -> Result<Self, MachineError> {
        mem.read_u16(address)
    }
}

impl MemoryMan for u32 {
    fn append_data(val: Self, mem: &mut Memory, address: usize) -> Result<(), MachineError> {
        mem.write_u32(address, val)
    }

    fn size_in_bytes() -> usize {
        4
    }

    fn read_data(mem: &Memory, address: usize) -> Result<Self, MachineError> {
        mem.read_u32(address)
    }
}

impl MemoryMan for u64 {
    fn append_data(val: Self, mem: &mut Memory, address: usize) -> Result<(), MachineError> {
        mem.write_u64(address, val)
    }

    fn size_in_bytes() -> usize {
        8
    }

    fn read_data(mem: &Memory, address: usize) -> Result<Self, MachineError> {
        mem.read_u64(address)
    }
}

impl Section {
    pub fn new(mem: &mut Memory) -> Result<Self, MachineError> {
        Ok(Self {
            dp: mem.allocate(8)?,
            data: mem.allocate(8 * 1024)?,
        })
    }

    pub fn free(&self, mem: &mut Memory) -> Result<(), MachineError> {
        mem.deallocate(&self.data)?;
        mem.deallocate(&self.dp)?;
        Ok(())
    }

    /// appends data to the section and increase DP
    pub fn append_data<T: MemoryMan>(&self, mem: &mut Memory, data: T) -> Result<(), MachineError> {
        let dp = mem.read_u64(self.dp.address)?;
        T::append_data(data, mem, self.data.address + dp as usize)?;
        mem.write_u64(self.dp.address, dp + T::size_in_bytes() as u64)?;
        Ok(())
    }

    pub fn read_data<T: MemoryMan>(&self, mem: &Memory, offset: usize) -> Result<T, MachineError> {
        T::read_data(mem, self.data.address + offset * T::size_in_bytes())
    }
}
