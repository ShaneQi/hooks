use iron;
use iron::{IronResult, Response};
use dotenv;
use std::process::Command;
use telegram;

pub fn handle(_: &mut iron::Request) -> IronResult<Response> {
    let command_output = Command::new("sshpass")
        .arg("-p")
        .arg(&ssh_password())
        .arg("ssh")
        .arg("-o")
        .arg("StrictHostKeyChecking=no")
        .arg(&ssh_location())
        .arg(&update_command())
        .output()
        .expect("failed to execute process");
    
    let update_result = format!("Shane's blog is rustier!\n\ncontext:\n{:?}", command_output);
    println!("{}", update_result);
    telegram::send_message(update_result);

    Ok(Response::with((iron::status::Ok, "")))
}

fn ssh_password() -> String {
    return dotenv::var("BLOG_SSH_PASSWORD").expect(
        "Failed to find blog ssh password in .env file.",
    );
}

fn ssh_location() -> String {
    return dotenv::var("BLOG_SSH_LOCATION").expect(
        "Failed to find blog ssh location in .env file.",
    );
}

fn update_command() -> String {
    return dotenv::var("BLOG_UPDATE_COMMAND").expect(
        "Failed to find blog update command in .env file.",
    );
}
