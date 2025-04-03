use std::{
    collections::HashMap,
    fmt,
    io::{self, BufRead, Error, ErrorKind, Read},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    ops::Deref,
    sync::RwLock,
};

use log::info;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub name: String,
    pub content: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.content)
    }
}

#[derive(Default)]
pub struct Database(RwLock<HashMap<IpAddr, Option<Message>>>);

impl Deref for Database {
    type Target = RwLock<HashMap<IpAddr, Option<Message>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq, Eq)]
pub enum StoreError {
    NameTooLong,
    ContentTooLong,
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::NameTooLong => write!(f, "name too long"),
            StoreError::ContentTooLong => write!(f, "content too long"),
        }
    }
}

impl fmt::Debug for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Database {
    pub fn store(&self, ip: IpAddr, msg: Option<Message>) -> Result<(), StoreError> {
        if let Some(Message {
            ref name,
            ref content,
        }) = msg
        {
            if name.len() > 127 {
                return Err(StoreError::NameTooLong);
            } else if content.len() > u8::MAX as usize {
                return Err(StoreError::ContentTooLong);
            }

            info!("storing \"{}\" for {}", msg.as_ref().unwrap(), ip);
        }

        let mut map = self.0.write().unwrap();
        let entry = map.entry(ip).or_default();
        if let Some(_) = msg {
            *entry = msg
        }

        Ok(())
    }

    pub fn load(&self, reader: impl io::Read) -> io::Result<()> {
        let mut map = self.0.write().unwrap();

        let mut reader = io::BufReader::new(reader);
        while !reader.fill_buf()?.is_empty() {
            let mut header = [0u8; 1];
            reader.read_exact(&mut header)?;
            let name_len = header[0] & 0x7f;
            let ip_type = header[0] >> 7;

            let ip = match ip_type {
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

            let conv =
                |v| String::from_utf8(v).map_err(|err| Error::new(ErrorKind::InvalidData, err));

            let msg = match name_len {
                1.. => {
                    let mut content_len = [0u8; 1];
                    reader.read_exact(&mut content_len)?;
                    let content_len = content_len[0];

                    let mut name = vec![0; name_len as usize];
                    reader.read_exact(&mut name)?;

                    let mut content = vec![0; content_len as usize];
                    reader.read_exact(&mut content)?;

                    Some(Message {
                        name: conv(name)?,
                        content: conv(content)?,
                    })
                }
                0 => None,
            };

            map.insert(ip, msg);
        }

        Ok(())
    }

    pub fn save(&self, mut writer: impl io::Write) -> io::Result<()> {
        for (ip, msg) in self.0.read().unwrap().iter() {
            let name_err = || Error::new(ErrorKind::InvalidData, "name is too long");

            let (name_len, content_len) = match msg {
                Some(Message { name, content }) => (
                    u8::try_from(name.len()).map_err(|_| name_err())?,
                    u8::try_from(content.len())
                        .map_err(|_| Error::new(ErrorKind::InvalidData, "message is too long"))?,
                ),
                None => (0, 0),
            };

            if name_len >> 7 == 1 {
                return Err(name_err());
            }

            let ip_type = match ip {
                IpAddr::V4(_) => 0,
                IpAddr::V6(_) => 1,
            };

            writer.write_all(&[name_len | ip_type << 7])?;

            match ip {
                IpAddr::V4(v4) => writer.write_all(&v4.octets())?,
                IpAddr::V6(v6) => writer.write_all(&v6.octets())?,
            }

            if let Some(Message { name, content }) = msg {
                writer.write_all(&[content_len])?;
                writer.write_all(name.as_bytes())?;
                writer.write_all(content.as_bytes())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_limit() {
        let result = Database::default().store(
            IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)),
            Some(Message {
                name: "a".repeat(128),
                content: "BABABABA!!!".to_string(),
            }),
        );

        assert_eq!(result, Err(StoreError::NameTooLong));
    }

    #[test]
    fn content_limit() {
        let result = Database::default().store(
            IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)),
            Some(Message {
                name: "bwabwabwabwa...".to_string(),
                content: "a".repeat(u8::MAX as usize + 1),
            }),
        );

        assert_eq!(result, Err(StoreError::ContentTooLong));
    }

    #[test]
    fn multiple() {
        let mut io = Vec::new();

        let first = (
            IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)),
            Some(Message {
                name: "celestial".to_string(),
                content: "hiya! <3".to_string(),
            }),
        );

        let second = (
            IpAddr::V6(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8)),
            Some(Message {
                name: "evil celestial".to_string(),
                content: "BYE.".to_string(),
            }),
        );

        {
            let db = Database::default();
            db.store(first.0, first.1.clone()).unwrap();
            db.store(second.0, second.1.clone()).unwrap();
            db.save(&mut io).expect("db should save");
        }

        {
            let db = Database::default();
            db.load(io.as_slice()).expect("db should load");

            let map = db.read().unwrap();
            assert_eq!(
                map.get(&first.0).expect("first entry should exist").clone(),
                first.1,
                "first entry was not correct "
            );
            assert_eq!(
                map.get(&second.0)
                    .expect("second entry should exist")
                    .clone(),
                second.1,
                "second entry was not correct"
            );
        }
    }
}
