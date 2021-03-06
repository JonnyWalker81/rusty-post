/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use curl::easy::{ Easy, List };
// use futures::Future;
use tokio_core::reactor::Core;
use tokio_curl::Session;
use request::utilities::{EmptyWriter};

use hyper::{Client};
use futures::Future;

use futures::stream::Stream;

use std::io::{stdout, Write};
use std::str;
use std::sync::Arc;

#[derive(Clone)]
pub enum HttpMethod {
    Get,
    Post
}

#[derive(Clone)]
pub struct RequestConfig<'a>{
    pub method: HttpMethod,
    pub headers: Vec<&'a str>,
    pub body: &'a str,
}

pub struct Request<'a> {
    pub url: String,
    pub config: RequestConfig<'a>,
    pub response: Response
}

#[derive(Clone)]
pub struct Response {
    pub body: String
}

impl<'a> RequestConfig<'a> {
    pub fn new(method: HttpMethod, headers: &Vec<&'a str>, body: &'a str) -> RequestConfig<'a>{
        RequestConfig {
            method: method,
            headers: headers.clone(),
            body: body,
        }
    }
}

impl<'a> Request<'a> {
    pub fn new(url: String) -> Request<'a> {
        Request {
            url: url.clone(),
            config: RequestConfig {
                method: HttpMethod::Get,
                headers: Vec::new(),
                body: "",
            },
            response: Response { body: "".into() }
        }
    }

    pub fn new_with_config(url: String, config: &RequestConfig<'a>) -> Request<'a> {
        Request {
            url: url.clone(),
            config: config.clone(),
            response: Response {body: "".into() }
        }
    }

    pub fn request(&mut self) -> Response {
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
        // header_list.append("Authorization: Basic anJvdGhiZXJnQGJsdWViZWFtLmNvbTpiYjEyMw==").unwrap();
        // header_list.append("Content-Type: application/json").unwrap();

        for h in self.config.headers.clone() {
            header_list.append(h).unwrap();
        }

        let mut b = Easy::new();

        match self.config.method {
            HttpMethod::Get => {
                b.get(true).unwrap();
            },
            HttpMethod::Post => {
                b.post(true).unwrap();
            }
        }
        // http://markups.elasticbeanstalk.com/documents/2299d82e-c268-4df8-b22e-bc6fa88793b3/markups
        b.url(self.url.as_str()).unwrap();
        b.http_headers(header_list).unwrap();
        // let mut response_body = String::new();
        b.write_function(|data| {
            // println!("{:?}", data);
            // Ok(data.len());
            stdout().write(data).unwrap();
            self.response.body.push_str(str::from_utf8(data).unwrap());
            Ok(data.len())
        }).unwrap();

        b.header_function(|header| {
            println!("header: {}", str::from_utf8(header).unwrap());
            true
        }).unwrap();

        b.post_field_size(self.config.body.as_bytes().len() as u64).unwrap();
        b.post_fields_copy(self.config.body.as_bytes()).unwrap();

        // let mut transfer = b.transfer();
        // transfer.write_function(|data| {
        //     response_body.push_str(str::from_utf8(data).unwrap());
        //     Ok(data.len())
        // });

        let url = self.url.parse::<hyper::Uri>().unwraap();
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let client = Client::new(&handle);

        let work = client.get(url).and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            res.body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
        }).map(|_| {
            println!("\n\nDone.");
        });

        core.run(work).unwrap();

        let requests = session.perform(b);

        // Run both requests, waiting for them to finish. Once done we print out
        // their response codes and errors.
        let mut b = lp.run(requests).unwrap();
        // println!("{:?}", a.response_code());
        println!("{:?}", b.response_code());

        return self.response.clone();
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
