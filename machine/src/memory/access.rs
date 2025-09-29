use crate::{execptions::MachineError, memory::memory::Memory, utils::binary::{Combine, Split}};

impl Memory {
    pub fn read_u8(&self, address: usize) -> Result<u8, MachineError> {
        if address >= self.data.len() {
            return Err(MachineError::InvalidAddress(None));
        }
        Ok(self.data[address])
    }

    pub fn read_u16(&self, address: usize) -> Result<u16, MachineError> {
        if address + 1 >= self.data.len() {
            return Err(MachineError::InvalidAddress(None));
        }
        Ok(u16::combine((self.data[address],self.data[address + 1])))
    }

    pub fn read_u32(&self, address: usize) -> Result<u32, MachineError> {
        if address + 3 >= self.data.len() {
            return Err(MachineError::InvalidAddress(None));
        }
        Ok(u32::combine((self.data[address],self.data[address + 1],self.data[address + 2],self.data[address + 3])))
    }

    pub fn read_u64(&self, address: usize) -> Result<u64, MachineError> {
        if address + 7 >= self.data.len() {
            return Err(MachineError::InvalidAddress(None));
        }
        Ok(u64::combine((self.data[address],self.data[address + 1],self.data[address + 2],self.data[address + 3],self.data[address + 4],self.data[address + 5],self.data[address + 6],self.data[address + 7])))
    }

    pub fn write_u8(&mut self, address: usize, val: u8) -> Result<(), MachineError> {
        if address >= self.data.len() {
            return Err(MachineError::InvalidAddress(None));
        }
        self.data[address] = val;
        Ok(())
    }

    pub fn write_u16(&mut self, address: usize, val: u16) -> Result<(), MachineError> {
        if address + 1 >= self.data.len() {
            return Err(MachineError::InvalidAddress(None));
        }
        let res: (u8, u8) = u16::split(val);
        self.data[address] = res.0;
        self.data[address + 1] = res.1;
        Ok(())
    }

    pub fn write_u32(&mut self, address: usize, val: u32) -> Result<(), MachineError> {
        if address + 3 >= self.data.len() {
            return Err(MachineError::InvalidAddress(None));
        }
        let res: (u8, u8, u8, u8) = u32::split(val);
        self.data[address] = res.0;
        self.data[address + 1] = res.1;
        self.data[address + 2] = res.2;
        self.data[address + 3] = res.3;
        Ok(())
    }

    pub fn write_u64(&mut self, address: usize, val: u64) -> Result<(), MachineError> {
        if address + 7 >= self.data.len() {
            return Err(MachineError::InvalidAddress(None));
        }
        let res: (u8, u8, u8, u8, u8, u8, u8, u8) = u64::split(val);
        self.data[address] = res.0;
        self.data[address + 1] = res.1;
        self.data[address + 2] = res.2;
        self.data[address + 3] = res.3;
        self.data[address + 4] = res.4;
        self.data[address + 5] = res.5;
        self.data[address + 6] = res.6;
        self.data[address + 7] = res.7;
        Ok(())
    }
}