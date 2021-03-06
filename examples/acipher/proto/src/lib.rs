pub enum Command {
    GenKey,
    GetSize,
    Encrypt,
    Decrypt,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::GenKey,
            1 => Command::GetSize,
            2 => Command::Encrypt, 
            3 => Command::Decrypt,
            _ => Command::Unknown,
        }
    }
}

pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
