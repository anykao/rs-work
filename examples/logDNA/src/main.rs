// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate pretty_env_logger;
#[macro_use]
extern crate dotenv;


// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use dotenv::dotenv;
use std::collections::HashMap;
use errors::*;
use reqwest::header::{Authorization, Basic};
use reqwest::Url;
use std::env;

const IngestBaseURL: &'static str = "https://logs.logdna.com/logs/ingest";

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

// Use this macro to auto-generate the main above. You may want to
// set the `RUST_BACKTRACE` env variable to see a backtrace.
// quick_main!(run);


// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init().unwrap();
    let api_key = env::var("API_KEY").unwrap();
    debug!("{}", api_key);
    debug!("{}", IngestBaseURL);
    let mut url = Url::parse(IngestBaseURL).unwrap();
    url.query_pairs_mut()
        .clear()
        .append_pair("hostname", "EXAMPLE_HOST")
        .append_pair("mac", "1")
        .append_pair("ip", "1")
        .append_pair("now", "1464041337000");
    debug!("url {}", url);
    let client = reqwest::Client::new().unwrap();
    let mut line = HashMap::new();
    line.insert("timestamp", 1464041337000);
    line.insert("line", "this is an awesome log statement");
    line.insert("file", "example.log");
    let mut lines = HashMap::new();
    lines.insert("lines", vec![line]);
    let res = client.post(url)
        .header(Authorization(Basic {
            username: api_key,
            password: None,
        }))
        .json(&lines)
        .send()
        .chain_err(|| "unable to send body")?;
    Ok(())
}
