extern crate clap;
extern crate rusty_post_lib;

use clap::App;
use clap::Arg;
use rusty_post_lib::request::request::{Request};

fn main() {
    let matches = App::new("rusty-post")
        .version("0.1.0")
        .about("Make HTTP requests")
        .author("Jonathan Rothberg")
            .arg(Arg::with_name("request")
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
        .takes_value(true)).get_matches();
    
    match matches.value_of("url") {
        Some(u) => {
            let headers = match matches.values_of("H") {
                Some(h) => {
                    h.collect()
                },
                None => Vec::new()
            };

            println!("Headers: {:?}", headers);
            let request = Request::new(u.into());
            request.request();
        },
        None => {}
    }
    println!("Hello, world!");
}
