#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate router;
extern crate iron;
extern crate serde_json;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;
extern crate dotenv;

use iron::prelude::*;
use std::io::Read;
use hyper_tls::HttpsConnector;

#[derive(Deserialize, Debug)]
struct TravisNotification {
    number: String,
    build_url: String,
    result: i16,
    result_message: String,
    branch: String,
    message: String,
    author_name: String,
    repository: Repository,
}

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    owner_name: String,
}

#[derive(Serialize, Debug)]
struct TelegramTextMessage {
    chat_id: i32,
    text: String,
    parse_mode: String,
}

fn router() -> router::Router {
    router!{
        get_user_repos: post "/travis" => travis,
    }
}

fn telegram_bot_token() -> String {
    return dotenv::var("TELEGRAM_BOT_TOKEN").expect(
        "Failed to find telegram bot token in .env file.",
    );
}

fn main() {
    // Check telegram bot api.
    let _ = telegram_bot_token();
    let chain = Chain::new(router());
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn travis(req: &mut Request) -> IronResult<Response> {
    let mut buffer = String::new();
    let _ = req.body.read_to_string(&mut buffer);
    let notif_result: serde_json::Result<TravisNotification> = serde_json::from_str(&buffer);
    match notif_result {
        Result::Ok(notif) => {
            println!("{}", buffer);
            let mut core = tokio_core::reactor::Core::new().unwrap();
            let client = hyper::client::Client::configure()
                .connector(HttpsConnector::new(4, &core.handle()).unwrap())
                .build(&core.handle());
            let uri = format!(
                "https://api.telegram.org/bot{}/sendMessage",
                &telegram_bot_token()
            ).parse()
                .unwrap();
            let mut req: hyper::Request<hyper::Body> =
                hyper::Request::new(hyper::Method::Post, uri);
            let msg = travis_notif_msg(notif);
            let body = serde_json::to_string(&msg).unwrap();
            req.headers_mut().set(hyper::header::ContentType::json());
            req.set_body(body);
            let post = client.request(req);
            let _ = core.run(post);
        }
        Result::Err(err) => println!("{}", err),
    }
    Ok(Response::with((iron::status::Ok, "")))
}

fn travis_notif_msg(notif: TravisNotification) -> TelegramTextMessage {
    let mark = match notif.result {
        0 => "✅",
        _ => "❌",
    };
    let line1 = format!(
        "{} *{}/{} - {}*",
        mark,
        notif.repository.owner_name,
        notif.repository.name,
        notif.branch
    );
    let line2 = format!(
        "[Build #{}]({}) {}",
        notif.number,
        notif.build_url,
        notif.result_message
    );
    let line3 = notif.author_name + ":";
    let line4 = notif.message;
    TelegramTextMessage {
        chat_id: 80548625,
        text: vec![line1, line2, line3, line4].join("\n"),
        parse_mode: "Markdown".to_string(),
    }
}