extern crate url;

use iron;
use iron::{IronResult, Response};
use std::io::Read;
use telegram;
use std::borrow::Cow;
use self::url::form_urlencoded::parse;
use serde_json;

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
        Result::Ok(notif) => telegram::send_message(travis_notif_msg(notif)),
        Result::Err(err) => {
            println!("{}", payload);
            println!("{}", err);
        }
    }
    Ok(Response::with((iron::status::Ok, "")))
}

pub fn travis_notif_msg(notif: TravisNotification) -> String {
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
    vec![line1, line2, line3, line4].join("\n")
}
