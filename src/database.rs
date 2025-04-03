use std::{
    fs::File,
    io::{Read, Write},
    sync::atomic::{AtomicU64, Ordering},
};

use eyre::Result;

pub struct Database(AtomicU64);

impl Database {
    pub fn new(file: &mut File) -> Result<Self> {
        let mut buf: [u8; 8] = [0; 8];
        file.read(&mut buf)?;

        Ok(Self(AtomicU64::new(u64::from_le_bytes(buf))))
    }

    pub fn increment(&self) -> u64 {
        self.0.fetch_add(1, Ordering::Relaxed)
    }

    pub fn save(&self, file: &mut File) -> Result<()> {
        let views = self.0.load(Ordering::Relaxed);
        file.write_all(&views.to_le_bytes())?;

        Ok(())
    }
}
