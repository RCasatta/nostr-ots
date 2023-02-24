use std::{fmt::Display, time::Duration};

pub trait Stamper: Send + Sync + Sized {
    fn new(timeout: Duration) -> Result<Self, String>;
    fn stamp(&self, digest_endpoint: &str, digest: &[u8]) -> Result<Vec<u8>, StamperError>;
}

#[derive(Debug)]
pub enum StamperError {
    Status(u16),
    IoError(std::io::Error),
    Transport(String),
}

impl Display for StamperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for StamperError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StamperError::Status(_) | StamperError::Transport(_) => None,
            StamperError::IoError(e) => Some(e),
        }
    }
}

#[cfg(feature = "ureq")]
pub(crate) mod ureq {
    use std::time::Duration;

    use super::{Stamper, StamperError};

    pub(crate) struct UreqStamper(ureq::Agent);

    impl Stamper for UreqStamper {
        fn new(timeout: Duration) -> Result<Self, String> {
            Ok(Self(ureq::builder().timeout(timeout).build()))
        }

        fn stamp(&self, digest_endpoint: &str, digest: &[u8]) -> Result<Vec<u8>, StamperError> {
            let resp = self
                .0
                .post(digest_endpoint)
                .send(digest)
                .map_err(|e| StamperError::Transport(e.to_string()))?;
            if resp.status() == 200 {
                let mut result = vec![];
                resp.into_reader()
                    .read_to_end(&mut result)
                    .map_err(|e| StamperError::IoError(e))?;

                Ok(result)
            } else {
                Err(StamperError::Status(resp.status()))
            }
        }
    }
}
