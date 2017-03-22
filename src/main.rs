extern crate clap;
extern crate rusty_post_lib;

use clap::App;
use clap::Arg;
use rusty_post_lib::request::request::{Request, RequestConfig, HttpMethod};

fn main() {
    let matches = App::new("rusty-post")
        .version("0.1.0")
        .about("Make HTTP requests")
        .author("Jonathan Rothberg")
            .arg(Arg::with_name("request")
                 .short("X")
                 .long("request")
                .default_value("GET")
                 .takes_value(true))
                 .arg(Arg::with_name("url")
                      .long("url")
                      .required(true)
                      .takes_value(true))
        .arg(Arg::with_name("H")
             .short("H")
             .long("header")
             .multiple(true)
             .takes_value(true)
        .default_value(""))
        .arg(Arg::with_name("data")
             .short("d")
             .long("data")
             .takes_value(true)
             .default_value("")
        ).get_matches();
    
    match matches.value_of("url") {
        Some(u) => {
            let headers = match matches.values_of("H") {
                Some(h) => {
                    h.collect()
                },
                None => Vec::new()
            };

            let data = match matches.value_of("data") {
                Some(d) => {
                    d
                },
                None => ""
            };

            let method = match matches.value_of("request") {
                Some(m) if m == "" || m == "GET" || m == "get" || m == "Get" => HttpMethod::Get,
                Some(m) if m == "POST" || m == "post" || m == "Post" => HttpMethod::Post,
                _ => HttpMethod::Get
            };

            println!("Headers: {:?}", headers);
            let config = RequestConfig::new(method, &headers, data);
            let request = Request::new_with_config(u.into(), &config);
            let resp = request.request();
            println!("{}", resp);
        },
        None => {}
    }
    println!("Hello, world!");
}
