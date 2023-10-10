use crate::dmp_firmware::FIRMWARE;
use crate::error::Error;
use crate::registers::Register;
use crate::sensor::Mpu6050;
use core::fmt::Debug;
use embedded_hal::i2c::I2c;

const BANK_SIZE: usize = 256;
const CHUNK_SIZE: usize = 16;

impl<I> Mpu6050<I>
where
    I: I2c,
    <I>::Error: Debug,
{
    pub fn load_firmware(&mut self) -> Result<(), Error<I>> {
        //log::info!("loading firmware");
        self.write_memory(&FIRMWARE)
        //self.boot_firmware()
    }

    pub fn boot_firmware(&mut self) -> Result<(), Error<I>> {
        self.write(&[Register::PrgmStart as u8, 0x04, 0x00])
    }

    fn write_memory(&mut self, data: &[u8]) -> Result<(), Error<I>> {
        for (bank, chunk) in data.chunks(BANK_SIZE).enumerate() {
            self.write_bank(bank as u8, chunk)?;
        }
        Ok(())
    }

    fn write_bank(&mut self, bank: u8, data: &[u8]) -> Result<(), Error<I>> {
        self.set_bank(bank)?;

        for (i, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
            let mut prolog_and_chunk: [u8; CHUNK_SIZE + 1] = [0; CHUNK_SIZE + 1];
            prolog_and_chunk[0] = Register::MemRw as u8;
            for (i, b) in chunk.iter().enumerate() {
                prolog_and_chunk[i + 1] = *b;
            }
            self.set_memory_start_address((i * CHUNK_SIZE) as u8)?;
            self.write(&prolog_and_chunk)?;
        }

        //log::info!("write {}", data.len());
        Ok(())
    }

    fn set_bank(&mut self, bank: u8) -> Result<(), Error<I>> {
        //log::info!("set bank={}", bank);
        self.write_register(Register::BankSel, bank)
    }

    fn set_memory_start_address(&mut self, addr: u8) -> Result<(), Error<I>> {
        //log::info!("set mem={}", addr);
        self.write_register(Register::MemStartAddr, addr)
    }
}
