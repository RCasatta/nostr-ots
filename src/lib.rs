use base64::{engine::general_purpose, Engine};
use bitcoin_hashes::sha256;
use opentimestamps::{
    ser::{Deserializer, DigestType},
    DetachedTimestampFile, Timestamp,
};
use std::{str::FromStr, thread};

pub use error::Error;
pub use options::Options;

mod error;
mod options;

/// Timestamp an `event_id` according to [NIP-03](https://github.com/nostr-protocol/nips/blob/master/03.md), returning a base64 ots proof.
///
/// `event_id` must be an event id as defined in [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md)
pub fn timestamp_event(event_id: &str) -> Result<String, Error> {
    timestamp_event_with_options(event_id, &Options::default())
}

/// Like [`timestamp_event`] but with `options`.
///
/// Options is a non-exhaustive struct to allow backward-compatible changes, but you cannot
/// instantiate, use `let mut opt = Options::default()` and change needed options
pub fn timestamp_event_with_options(event_id: &str, options: &Options) -> Result<String, Error> {
    let client = ureq::builder().timeout(options.timeout).build();

    // The `event_id` is a SHA256 hash of the hash-serialized event, so we can treat it as a hash
    // directly and use it as the based for a detached timestamp file later.
    let hash = sha256::Hash::from_str(event_id)?;

    let results: Vec<_> = thread::scope(|s| {
        let mut handles = vec![];

        for el in options.aggregators.iter() {
            let h = s.spawn(|| {
                let body = client.post(el).send(&hash[..])?;
                if body.status() == 200 {
                    let mut result = vec![];
                    body.into_reader().read_to_end(&mut result)?;
                    Ok(result)
                } else {
                    Err(Error::Not200(el.to_string(), body.status()))
                }
            });
            handles.push(h);
        }

        handles
            .into_iter()
            .map(|h| {
                h.join()
                    .expect("thread cannot panic, no unwrap or index access")
            })
            .collect()
    });
    let mut oks = vec![];
    let mut errs = vec![];
    for r in results {
        match r {
            Ok(ok) => oks.push(ok),
            Err(err) => errs.push(err),
        }
    }

    if oks.len() < options.at_least {
        return Err(Error::TooFewResults {
            errors: errs.iter().map(|e| e.to_string()).collect(),
            at_least: options.at_least,
            aggregators: options.aggregators.len(),
        });
    }
    let mut all = vec![];
    for (i, r) in oks.iter().enumerate() {
        // Insert fork opcodes before each proof, except the last one.
        if i < oks.len() - 1 {
            all.push(0xFF);
        }
        all.extend(r);
    }

    let mut deserializer = Deserializer::new(&all[..]);
    let timestamp = Timestamp::deserialize(&mut deserializer, hash.to_vec())?;

    let detached = DetachedTimestampFile {
        digest_type: DigestType::Sha256,
        timestamp,
    };
    let mut result = vec![];
    detached.to_writer(&mut result).unwrap();

    let b64 = general_purpose::STANDARD.encode(&result);

    Ok(b64)
}

#[cfg(test)]
mod test {
    use base64::{engine::general_purpose, Engine};
    use opentimestamps::{ser::DigestType, DetachedTimestampFile};

    use crate::{timestamp_event, timestamp_event_with_options, Error, Options};

    #[test]
    fn test_timestamp_event() {
        let result =
            timestamp_event("f5e5842b677ec450c5668daf8f99827cba91a9d80705ab3e0422f0ac4519cf84")
                .unwrap();

        assert!(result.len() > 20);

        let bytes = general_purpose::STANDARD.decode(result).unwrap();

        let t = DetachedTimestampFile::from_reader(&bytes[..]).unwrap();
        assert_eq!(t.digest_type, DigestType::Sha256);
    }

    #[test]
    fn test_timestamp_event_with_options() {
        let mut options = Options::default();

        assert!(timestamp_event_with_options(
            "f5e5842b677ec450c5668daf8f99827cba91a9d80705ab3e0422f0ac4519cf84",
            &options,
        )
        .is_ok());

        options.aggregators[0] = "http://notexist".to_string();
        options.at_least = 4;
        let err = timestamp_event_with_options(
            "f5e5842b677ec450c5668daf8f99827cba91a9d80705ab3e0422f0ac4519cf84",
            &options,
        );
        match err {
            Err(Error::TooFewResults {
                errors,
                calendars,
                at_least,
            }) => {
                assert_eq!(errors.len(), 1);
                assert_eq!(at_least, 4);
                assert_eq!(calendars, 4);
            }
            _ => assert!(false),
        }

        options.aggregators = vec!["http://notexist".to_string()];
        options.at_least = 1;

        assert!(timestamp_event_with_options(
            "f5e5842b677ec450c5668daf8f99827cba91a9d80705ab3e0422f0ac4519cf84",
            &options,
        )
        .is_err());
    }
}
