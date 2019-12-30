#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    InvalidSignature,
    InvalidPublicKey,
    InvalidSecretKey,
    InvalidRecoveryId,
    InvalidMessage,
    InvalidInputLength,
    TweakOutOfRange,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidSignature => write!(f, "Invalid signature"),
            Error::InvalidPublicKey => write!(f, "Invalid public key"),
            Error::InvalidSecretKey => write!(f, "Invalid secret key"),
            Error::InvalidRecoveryId => write!(f, "Invalid recovery ID"),
            Error::InvalidMessage => write!(f, "Invalid message"),
            Error::InvalidInputLength => write!(f, "Invalid input length"),
            Error::TweakOutOfRange => write!(f, "Tweak out of range"),
        }
    }
}
