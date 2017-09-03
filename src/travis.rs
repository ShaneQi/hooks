extern crate url;

use iron;
use iron::{IronResult, Response};
use hyper_tls::HttpsConnector;
use serde_json;
use tokio_core::reactor::Core;
use hyper;
use hyper::client::Client;
use dotenv;
use std::io::Read;
use self::url::form_urlencoded::parse;
use std::borrow::Cow;

#[derive(Deserialize, Debug)]
pub struct TravisNotification {
    pub number: String,
    pub build_url: String,
    pub result: i16,
    pub result_message: String,
    pub branch: String,
    pub message: String,
    pub author_name: String,
    pub repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub owner_name: String,
}

#[derive(Serialize, Debug)]
pub struct TelegramTextMessage {
    pub chat_id: i32,
    pub text: String,
    pub parse_mode: String,
}

pub fn travis(req: &mut iron::Request) -> IronResult<Response> {
    let mut body_string = String::new();
    let _ = req.body.read_to_string(&mut body_string).expect(
        "Failed to read string from request body.",
    );
    let payload_pair: (Cow<str>, Cow<str>) = parse(body_string.as_bytes()).next().expect(
        "Failued to get payload from request body.",
    );
    let payload = payload_pair.1;
    let notif_result: serde_json::Result<TravisNotification> = serde_json::from_str(&payload);
    match notif_result {
        Result::Ok(notif) => {
            let mut core = Core::new().unwrap();
            let client = Client::configure()
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
        Result::Err(err) => {
            println!("{}", payload);
            println!("{}", err);
        }
    }
    Ok(Response::with((iron::status::Ok, "")))
}

pub fn travis_notif_msg(notif: TravisNotification) -> TelegramTextMessage {
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

fn telegram_bot_token() -> String {
    return dotenv::var("TELEGRAM_BOT_TOKEN").expect(
        "Failed to find telegram bot token in .env file.",
    );
}