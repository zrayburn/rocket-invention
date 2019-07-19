#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate crypto;
extern crate base64;
extern crate rustc_serialize as serialize;
use serialize::base64::{STANDARD, ToBase64};
use rocket::http::RawStr;
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::sha2::Sha512;
use crypto::mac::Mac;

#[get("/hmac/<key>/<text>")]
fn hello(key: &RawStr, text: &RawStr) -> String {
    let mut object = Hmac::new(Sha512::new(), key.as_bytes());
    object.input(text.as_bytes());
    format!("HMAC digest: {}", object.result().code().to_base64(STANDARD))
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}