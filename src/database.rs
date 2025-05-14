use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Error, ErrorKind, Read, Result, Write},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    ops::Deref,
    sync::RwLock,
};

#[derive(Default)]
pub struct Database(RwLock<HashSet<IpAddr>>);

impl Deref for Database {
    type Target = RwLock<HashSet<IpAddr>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Database {
    pub fn store(&self, ip: IpAddr) {
        self.0.write().unwrap().insert(ip);
    }

    pub fn load(&self, reader: impl Read) -> Result<()> {
        let mut map = self.0.write().unwrap();

        let mut reader = BufReader::new(reader);
        while !reader.fill_buf()?.is_empty() {
            let mut ip_type = [0u8; 1];
            reader.read_exact(&mut ip_type)?;

            let ip = match ip_type[0] {
                0 => {
                    let mut buf = [0u8; 4];
                    reader.read_exact(&mut buf)?;
                    IpAddr::V4(Ipv4Addr::from(buf))
                }
                1 => {
                    let mut buf = [0u8; 16];
                    reader.read_exact(&mut buf)?;
                    IpAddr::V6(Ipv6Addr::from(buf))
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "ip type is an invalid value",
                    ))
                }
            };

            map.insert(ip);
        }

        Ok(())
    }

    pub fn save(&self, mut writer: impl Write) -> Result<()> {
        for ip in self.0.read().unwrap().iter() {
            let ip_type = match ip {
                IpAddr::V4(_) => 0,
                IpAddr::V6(_) => 1,
            };

            writer.write_all(&[ip_type])?;

            match ip {
                IpAddr::V4(v4) => writer.write_all(&v4.octets())?,
                IpAddr::V6(v6) => writer.write_all(&v6.octets())?,
            }
        }

        Ok(())
    }
}
