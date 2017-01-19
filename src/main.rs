extern crate clap;
extern crate hyper;
extern crate hyper_native_tls;

use clap::App;
use hyper::Url;
use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn pushover(client: &Client, message: &str, title: &str) {
    let pushover_tok = std::env::var("NOTI_PUSHOVER_TOK").expect("pushover token error");
    let pushover_dest = std::env::var("NOTI_PUSHOVER_DEST").expect("pushover destination error");

    let mut pushover_url = Url::parse("https://api.pushover.net").expect("pushover_url error");
    pushover_url.set_path("/1/messages.json");
    pushover_url.query_pairs_mut()
        .append_pair("token", pushover_tok.as_str())
        .append_pair("user", pushover_dest.as_str())
        .append_pair("message", message)
        .append_pair("title", title);
    println!("{:?}", pushover_url);

    let res = client.post(pushover_url).send().expect("post sent error");
    println!("{:?}", res);
}

fn main() {
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
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let mut client = Client::with_connector(connector);

    pushover(&client, message, title);
}
