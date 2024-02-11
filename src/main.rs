use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use tiny_http::{Header, Response, Server};

use clap::{Parser};
use clap_derive::{Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    port: u16,
    executable: PathBuf,
    args: Option<Vec<String>>
}

const PANGO_HTML: &str = r#"<html>
<head>
    <meta http-equiv="refresh" content="10; url=http://localhost:3000">
</head>

<center>
    <h3 style='font-family: sans-serif'>Pangolin is starting your service!</h1>
        <h1>🦔</h1>
</center>
</html>"#;

fn main() {
    let cli = Cli::parse();
    let waiting_room = thread::spawn(move || {
        let waiting_room = Server::http("0.0.0.0:26015").unwrap();
        for request in waiting_room.incoming_requests() {
            let mut res = Response::from_string(PANGO_HTML);
            let str = "Content-Type: text/html; charset=utf-8";
            res.add_header(Header::from_str(str).unwrap());
            request.respond(res).unwrap();
        }
    });
    {
        let bind_address = format!("0.0.0.0:{}", cli.port);
        let server = Server::http(bind_address).unwrap();

        for request in server.incoming_requests() {
            let mut res = Response::from_string(PANGO_HTML);
            let str = "Content-Type: text/html; charset=utf-8";
            res.add_header(Header::from_str(str).unwrap());
            request.respond(res).unwrap();
            break;
        }
    }
    sleep(Duration::from_millis(2000));
    println!("Starting thing...");
    let mut commo = Command::new(cli.executable);
    match cli.args {
        None => {}
        Some(args) => {
            commo.args(args);
        }
    }
    let _output = commo.spawn().unwrap();
    //signal::kill(Pid::from_raw(output.id() as pid_t), Signal::SIGTERM).unwrap();
    println!("started!");
    waiting_room.join().unwrap();
}
