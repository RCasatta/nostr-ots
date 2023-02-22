#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ureq error: {0}")]
    Ureq(Box<ureq::Error>), // boxed because ureq::Error is 240 bytes

    #[error(transparent)]
    Hashes(#[from] bitcoin_hashes::Error),

    #[error(transparent)]
    Hex(#[from] bitcoin_hashes::hex::Error),

    #[error(transparent)]
    Ots(#[from] opentimestamps::error::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Calendar server {0} returned HTTPS status code {1} instead of 200 OK")]
    Not200(String, u16),

    #[error("Out of {calendars} calendars, we expected at least {at_least} good responses, but there were these errors: {errors:?}")]
    TooFewResults {
        errors: Vec<String>,
        calendars: usize,
        at_least: usize,
    },
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Error::Ureq(Box::new(value))
    }
}
