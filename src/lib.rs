#[cfg(not(target_os = "macos"))]
compile_error!("works only on macOS");

pub use smc::{
    Celsius, CpuTemperatures, Fahrenheit, FanSpeed, MilliAmpere, MilliAmpereHours, Rpm, Smc, Volt,
    Watt,
};
use std::{
    error::Error as StdError,
    fmt::{self, Display},
};

pub type Result<T> = std::result::Result<T, Error>;

mod smc;

#[derive(Debug)]
pub enum Error {
    SmcNotAvailable,
    InsufficientPrivileges,
    SmcError(i32),
    DataError { key: u32, tpe: u32 },
}

fn tpe_name(tpe: &u32) -> String {
    let bytes = tpe.to_be_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SmcNotAvailable => write!(f, "SMC is not available, are you running on a Mac?"),
            Error::InsufficientPrivileges => {
                write!(f, "Could not perform SMC operation, try running with sudo")
            }
            Error::SmcError(code) => write!(f, "Could not perform SMC operation: {:08x}", code),
            Error::DataError { key, tpe } => write!(
                f,
                "Could not read data for key {} of type {}",
                tpe_name(key),
                tpe_name(tpe)
            ),
        }
    }
}
