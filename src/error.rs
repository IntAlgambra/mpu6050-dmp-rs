use core::fmt::Debug;
use core::fmt::Formatter;
use embedded_hal::i2c::I2c;

/// Error during initialization of sensor. Wraps [`Error`].
pub struct InitError<I>
where
    I: I2c,
    <I>::Error: Debug,
{
    pub i2c: I,
    pub error: Error<I>,
}

impl<I> Debug for InitError<I>
where
    I: I2c,
    <I>::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.error.fmt(f)
    }
}

/// Error for sensor operations.
pub enum Error<I>
where
    I: I2c,
    <I>::Error: Debug,
{
    WriteError(<I>::Error),
    WriteReadError(<I>::Error),
    WrongDevice,
}

impl<I> Debug for Error<I>
where
    I: I2c,
    <I>::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Self::WriteReadError(e) => f.debug_tuple("WriteReadError").field(e).finish(),
            Self::WriteError(e) => f.debug_tuple("WriteError").field(e).finish(),
            Self::WrongDevice => f.write_str("WrongDevice"),
        }
    }
}
