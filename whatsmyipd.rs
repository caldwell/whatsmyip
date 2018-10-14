// Copyright © 2018 David Caldwell <david@porkrind.org>
extern crate iron;
extern crate chrono ;

use chrono::prelude::*;
use iron::prelude::*;
use iron::status;
use std::env;

fn main() {
    fn handler(req: &mut Request) -> IronResult<Response> {
        let ip = req.remote_addr.ip().to_string();
        println!("{:35} IP: {}", Local::now(), ip);
        let mut resp = Response::with((status::Ok, ip));
        resp.headers.append_raw("X-Source-URL", b"https://github.com/caldwell/whatsmyip".to_vec());
        Ok(resp)
    }
    let _server = Iron::new(handler).http(format!("0.0.0.0:{}", env::var("WHATSMYIP_PORT").unwrap_or("3141".to_string()))).unwrap();
}
