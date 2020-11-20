use core::fmt::{self, Debug, Display, Formatter};

pub type OneWireResult<T, E> = Result<T, OneWireError<E>>;

#[derive(Debug, Copy, Clone)]
pub enum OneWireError<E> {
    /// The Bus was expected to be pulled high by a ~5K ohm pull-up resistor, but it wasn't
    BusNotHigh,

    PinError(E),

    /// An unexpected response was received from a command. This generally happens when a new sensor is added
    /// or removed from the bus during a command, such as a device search.
    UnexpectedResponse,

    FamilyCodeMismatch,
    CrcMismatch,
    Timeout,
}

impl<E: Display> Display for OneWireError<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::BusNotHigh => write!(f, "The Bus was expected to be pulled high by a ~5K ohm pull-up resistor, but it wasn't."),
            Self::PinError(error) => write!(f, "Pin error: {}.", error),
            Self::UnexpectedResponse => write!(f, "An unexpected response was received from a command."),
            Self::FamilyCodeMismatch => write!(f, "Family code mismatch."),
            Self::CrcMismatch => write!(f, "CRC mismatch."),
            Self::Timeout => write!(f, "Timeout."),
        }
    }
}
