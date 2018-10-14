// Copyright © 2018 David Caldwell <david@porkrind.org>

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::{Request, Response};
use rocket::http::Status;
use std::io::Cursor;

//#[get("/")]
#[catch(404)]
fn index<'a>(req: &Request) -> Response<'a> {
    Response::build()
        .raw_header("X-Source-URL", "https://github.com/caldwell/whatsmyip")
        .sized_body(Cursor::new(format!("{}", req.remote().unwrap_or("0.0.0.0:0".parse().unwrap()).ip())))
        .status(Status::Ok)
        .finalize()
}

fn main() {
    println!("Error: {}", rocket::ignite()
//             .mount("/", routes![index])
             .catch(catchers![index])
             .launch());
}
