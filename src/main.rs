#[macro_use]
extern crate router;
extern crate iron;
extern crate hooks;

use iron::prelude::*;

fn router() -> router::Router {
    router!{
        travis_ci_notifications: post "/travis" => hooks::travis::travis,
    }
}


fn main() {
    let chain = Chain::new(router());
    Iron::new(chain).http("0.0.0.0:9877").unwrap();
}
