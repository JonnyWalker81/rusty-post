extern crate tokio_curl;
extern crate curl;

// extern crate env_logger;
extern crate futures;
extern crate tokio_core;
// extern crate tokio_curl;

use curl::easy::Easy;
use futures::Future;
use tokio_core::reactor::Core;
use tokio_curl::Session;

use std::io::{stdout, Write};

fn example() {
    // env_logger::init().unwrap();

    let mut lp = Core::new().unwrap();
    let session = Session::new(lp.handle());

    // Once we've got our session available to us, execute our two requests.
    // Each request will be a GET request and for now we just ignore the actual
    // downloaded data.
    let mut a = Easy::new();
    a.get(true).unwrap();
    a.url("https://www.rust-lang.org").unwrap();
    a.write_function(|data| Ok(data.len())).unwrap();

    let mut b = Easy::new();
    b.get(true).unwrap();
    b.url("https://github.com").unwrap();
    b.write_function(|data| {
        println!("{:?}", data);
        // Ok(data.len());
        Ok(stdout().write(data).unwrap())
    }).unwrap();

    let requests = session.perform(a).join(session.perform(b));

    // Run both requests, waiting for them to finish. Once done we print out
    // their response codes and errors.
    let (mut a, mut b) = lp.run(requests).unwrap();
    println!("{:?}", a.response_code());
    println!("{:?}", b.response_code());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        example()
    }
}
