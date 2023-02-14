use std::env;

use nostr_ots::timestamp_event;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Need the digest as argument")
    }
    println!("{}", timestamp_event(&args[1]).unwrap());
}
