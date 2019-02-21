#[macro_use]
extern crate router;
extern crate iron;
extern crate hooks;

use iron::prelude::*;

fn router() -> router::Router {
    router!{
        travis_ci_notifications: post "/travis" => hooks::travis::travis,
        rusty_blog_updater: post "/rusty_blog" => hooks::rusty_blog::handle,
        app_center_notifications: post "/app_center" => hooks::app_center::handle,
    }
}


fn main() {
    let chain = Chain::new(router());
    Iron::new(chain).http("0.0.0.0:9877").unwrap();
}
