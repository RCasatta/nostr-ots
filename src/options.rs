use crate::Stamper;

#[non_exhaustive]
pub struct Options<S: Stamper> {
    /// POST digest URLs of the aggregator servers.
    pub aggregators: Vec<String>,

    /// The minimum number of aggregators needed for a timestamp to be considered usable.
    ///
    /// Default: 2
    pub at_least: usize,

    pub stamper: S,
}
impl<S: Stamper> Options<S> {
    pub(crate) fn digest_endpoints(&self) -> impl Iterator<Item = String> + '_ {
        self.aggregators.iter().map(|agg| format!("{agg}/digest"))
    }
}

impl<S: Stamper> Options<S> {
    pub fn with_stamper(stamper: S) -> Self {
        Self {
            aggregators: AGGREGATORS.map(|s| s.to_string()).to_vec(),
            at_least: 2,
            stamper,
        }
    }
}

pub const AGGREGATORS: [&str; 4] = [
    "https://a.pool.opentimestamps.org",
    "https://b.pool.opentimestamps.org",
    "https://a.pool.eternitywall.com",
    "https://ots.btc.catallaxy.com",
];
