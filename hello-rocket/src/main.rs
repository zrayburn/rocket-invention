#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate crypto;
use rocket::http::RawStr;
use crypto::hmac;



#[get("/hello/<key>/<text>")]
fn hello(key: &RawStr, text: &RawStr) -> String {
    let object = crypto::hmac::Hmac(key)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}