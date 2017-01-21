extern crate clap;
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;
#[macro_use]
extern crate simple_error;

use clap::App;
use hyper::Url;
use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;
use simple_error::SimpleError;
use std::io::Read;
use std::str::FromStr;

fn pushover(client: &Client, message: &str, title: &str) -> Result<(), SimpleError> {
    let pushover_tok = try_with!(std::env::var("NOTI_PUSHOVER_TOK"), "cannot get pushover token");
    let pushover_dest = try_with!(std::env::var("NOTI_PUSHOVER_DEST"), "cannotpushover destination");

    let mut pushover_url = try_with!(Url::parse("https://api.pushover.net"), "cannot parse pushover_url");
    pushover_url.set_path("/1/messages.json");
    pushover_url.query_pairs_mut()
        .append_pair("token", pushover_tok.as_str())
        .append_pair("user", pushover_dest.as_str())
        .append_pair("message", message)
        .append_pair("title", title);

    let mut response = try_with!(client.post(pushover_url).send(), "cannot send https post or get response");
    if !response.status.is_success() {
        return Err(SimpleError::new(format!("unsuccessful response, {}", response.status)));
    }
    let mut json_response = String::new();
    try_with!(response.read_to_string(&mut json_response), "cannot read response");
    let value = try_with!(Value::from_str(json_response.as_str()), "cannot parse json response");
    let json_object = require_with!(value.as_object(), "json response is not an object");
    if json_object.get("status") != Some(&Value::U64(1)) {
        return Err(SimpleError::new(format!("request failed, {:?}", json_object.get("error"))));
    }
    Ok(())
}

fn run() -> Result<(), SimpleError> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                          .version(env!("CARGO_PKG_VERSION"))
                          .author(env!("CARGO_PKG_AUTHORS"))
                          .about("Trigger notification")
                          .args_from_usage(
                              "-t, --title=[TITLE] 'Notification title.'
                               -m, --message=[MESSAGE] 'Notification message.`
                               -w, --pwatch=[PID] 'Trigger notification after PID disappears.'
                               -o, --pushover 'Trigger a Pushover notification. Requires NOTI_PUSHOVER_TOK and NOTI_PUSHOVER_DEST to be set.'")
                          .get_matches();
    let title = matches.value_of("TITLE").unwrap_or(env!("CARGO_PKG_NAME"));
    let message = matches.value_of("MESSAGE").unwrap_or("Done!");
    let maybe_pid = matches.value_of("PID");
    let ssl = try_with!(NativeTlsClient::new(), "cannot initialize tls");
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    if matches.is_present("pushover") {
        try_with!(pushover(&client, message, title), "cannot notify with pushover");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{:?}", e);
    }
}
