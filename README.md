
Proof of concept implementation of Nostr [NIP-03](https://github.com/nostr-protocol/nips/blob/master/03.md) OpenTimestamps Attestations for Events

# Usage

With the following nostr event

```json
{
  "id": ...,
  "kind": ...,
  ...,
  ...,
  "ots": <base64-encoded OTS file data>
}
```


```rust
let event_id = event_json["id"];
let ots = nostr_ots::timestamp_event(event_id).unwrap();
event_json["ots"] = ots;
```

# Test

Test implementation against OpenTimestamps python [client](https://github.com/opentimestamps/opentimestamps-client)

```bash

$ echo "Proof of concept implementation of NIP-03" > example

$ shasum -a 256 example
d6f3c7616621ea55fa99444dc82ce7eafed2e71352a0890882b2e42285b90724  example

$ cargo run --example stamp -- d6f3c7616621ea55fa99444dc82ce7eafed2e71352a0890882b2e42285b90724  example | base64 --decode >example.ots

$ ots info example.ots
File sha256 hash: d6f3c7616621ea55fa99444dc82ce7eafed2e71352a0890882b2e42285b90724
Timestamp:
 -> append 35abd6957bb035a904df2a74d1287c56
    sha256
    prepend 63ebc438
    append ba3242b113ee37d7
    verify PendingAttestation('https://bob.btc.calendar.opentimestamps.org')
 -> append 77cf7970f11e52ec3531b27b44d856e7
    sha256
    prepend 63ebc439
    append bd08b13e2e6f49f5
    verify PendingAttestation('https://btc.calendar.catallaxy.com')
 -> append a2231bf8052e0f79947121c687b3f97e
    sha256
    append b80ffca6f3ea6ad3e5a4bb0b0be819ba4753bc5a3a8987c46fa0fde76847a520
    sha256
    prepend 63ebc438
    append cf1c91194578d4e3
    verify PendingAttestation('https://finney.calendar.eternitywall.com')
 -> append ab64d01a5ebe068d205024e098a69feb
    sha256
    prepend 1153c8b5ab758ff941eaf6507c3e774abd58a81089e86c8bbec137a0e36c6680
    sha256
    prepend 63ebc438
    append 59e9634366c30207
    verify PendingAttestation('https://alice.btc.calendar.opentimestamps.org')
```

# TODO

- Support async other than blocking
- Better support of upstream library to generate timestamp
- ots client prepend some data before submitting the proof to avoid leaking the real hash
- General code improvement
- Create a struct holding the http client to avoid reinitializing every time
- Better integration with nostrs structs, if a crate providing general/standard struct/traits exists
- handle upgrade of the ots in an event json
- handle verify of the ots in the event json, outputs ots proof date
- use httptest for local testing
