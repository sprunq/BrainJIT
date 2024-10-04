pub mod codegen;
pub mod executor;
pub mod state;
pub mod x86_64;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeResultCode {
    Ok = 0,
    IoError = 1,
}

impl RuntimeResultCode {
    pub fn is_error(self) -> bool {
        self != RuntimeResultCode::Ok
    }
}

impl TryFrom<u8> for RuntimeResultCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RuntimeResultCode::Ok),
            1 => Ok(RuntimeResultCode::IoError),
            _ => Err("Invalid result code"),
        }
    }
}
