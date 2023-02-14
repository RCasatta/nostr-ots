#[non_exhaustive]
pub struct Options {
    /// Calendar server digest url
    pub calendars: Vec<String>,

    /// Correct reply from `at_least` calendars is considered ok.
    /// Default: 2
    pub at_least: usize,

    /// Overall timeout for each request to a calendar in milliseconds.
    /// Default: 5000 millisecs
    pub timeout: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            calendars: CALENDARS.map(|s| s.to_string()).to_vec(),
            at_least: 2,
            timeout: 5000,
        }
    }
}

pub const CALENDARS: [&str; 4] = [
    "https://a.pool.opentimestamps.org/digest",
    "https://b.pool.opentimestamps.org/digest",
    "https://a.pool.eternitywall.com/digest",
    "https://ots.btc.catallaxy.com/digest",
];
