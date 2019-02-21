extern crate url;

use iron;
use iron::{IronResult, Response};
use std::io::Read;
use telegram;
use serde_json;

#[derive(Deserialize, Debug)]
pub struct AppCenterNotification {
    pub app_name: String,
    pub branch: String,
    pub build_id: String,
    pub build_link: String,
    pub build_status: String,
    pub source_version: String,
}

pub fn handle(req: &mut iron::Request) -> IronResult<Response> {
    let mut body_string = String::new();
    let _ = req.body.read_to_string(&mut body_string).expect(
        "Failed to read string from request body.",
    );
    let notif_result: serde_json::Result<AppCenterNotification> = serde_json::from_str(&body_string);
    match notif_result {
        Result::Ok(notif) => telegram::send_message(app_center_notif_msg(notif)),
        Result::Err(err) => {
            println!("{}", body_string);
            println!("{}", err);
        }
    }
    Ok(Response::with((iron::status::Ok, "")))
}

pub fn app_center_notif_msg(notif: AppCenterNotification) -> String {
    let mark = match notif.build_status.as_str() {
        "Succeeded" => "✅",
        _ => "❌",
    };
    let line1 = format!(
        "{} *{}/{}*",
        mark,
        notif.app_name,
        notif.branch
    );
    let line2 = format!(
        "[Build #{}]({}) {}",
        notif.build_id,
        notif.build_link,
        notif.build_status
    );
    let line3 = notif.source_version;
    vec![line1, line2, line3].join("\n")
}
