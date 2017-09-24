
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper;
use serde_json;
use dotenv;
use hyper::client::Client;

pub fn send_message(message: String) {
    let mut core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());
    let uri = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        &telegram_bot_token()
    ).parse()
        .unwrap();
    let mut req: hyper::Request<hyper::Body> = hyper::Request::new(hyper::Method::Post, uri);
    let msg = TelegramTextMessage {
        chat_id: 80548625,
        text: message,
        parse_mode: "Markdown".to_string(),
    };
    let body = serde_json::to_string(&msg).unwrap();
    req.headers_mut().set(hyper::header::ContentType::json());
    req.set_body(body);
    let post = client.request(req);
    let _ = core.run(post);
}

#[derive(Serialize, Debug)]
pub struct TelegramTextMessage {
    pub chat_id: i32,
    pub text: String,
    pub parse_mode: String,
}

fn telegram_bot_token() -> String {
    return dotenv::var("TELEGRAM_BOT_TOKEN").expect(
        "Failed to find telegram bot token in .env file.",
    );
}