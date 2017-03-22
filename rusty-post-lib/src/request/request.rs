/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use curl::easy::{ Easy, List };
// use futures::Future;
use tokio_core::reactor::Core;
use tokio_curl::Session;

use std::io::{stdout, Write};


pub struct Request {
    pub url: String
}

impl Request {
    pub fn new(url: String) -> Request {
        Request {
            url: url.clone()
        }
    }

    pub fn request(&self) {
        // env_logger::init().unwrap();

        let mut lp = Core::new().unwrap();
        let session = Session::new(lp.handle());

        // Once we've got our session available to us, execute our two requests.
        // Each request will be a GET request and for now we just ignore the actual
        // downloaded data.
        // let mut a = Easy::new();
        // a.get(true).unwrap();
        // a.url("http://markups.elasticbeanstalk.com/documents/2299d82e-c268-4df8-b22e-bc6fa88793b3/markups").unwrap();
        // a.write_function(|data| Ok(data.len())).unwrap();

        let mut header_list = List::new();
        header_list.append("Authorization: Basic anJvdGhiZXJnQGJsdWViZWFtLmNvbTpiYjEyMw==").unwrap();
        header_list.append("Content-Type: application/json").unwrap();

        let mut b = Easy::new();
        b.get(true).unwrap();
        // http://markups.elasticbeanstalk.com/documents/2299d82e-c268-4df8-b22e-bc6fa88793b3/markups
        b.url(self.url.as_str()).unwrap();
        b.http_headers(header_list).unwrap();
        b.write_function(|data| {
            // println!("{:?}", data);
            // Ok(data.len());
            stdout().write(data).unwrap();
            Ok(data.len())
        }).unwrap();

        let requests = session.perform(b);

        // Run both requests, waiting for them to finish. Once done we print out
        // their response codes and errors.
        let mut b = lp.run(requests).unwrap();
        // println!("{:?}", a.response_code());
        println!("{:?}", b.response_code());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        Request::new("http://markups.elasticbeanstalk.com/documents/2299d82e-c268-4df8-b22e-bc6fa88793b3/markups".into());
    }
}
