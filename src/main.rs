extern crate actix_web;
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate env_logger;
extern crate serde;
extern crate serde_json;

use actix_web::middleware::Logger;
use actix_web::{http, server, App, Json, Responder};

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    greeting: String,
}

fn greet(input: Json<Input>) -> impl Responder {
    format!("Hello {}!", input.name)
}

fn main() {
    env_logger::init();
    let arguments = clap::App::new("rust-starter")
        .version("0.1")
        .about("Rust starter for TBB")
        .author("Max Trense")
        .subcommand(
            clap::SubCommand::with_name("server")
                .about("Start the webserver")
                .arg(
                    clap::Arg::with_name("listen")
                        .long("listen")
                        .help("Listen to this address")
                        .takes_value(true)
                        .value_name("ADDRESS"),
                ),
        )
        .get_matches();

    if let Some(server) = arguments.subcommand_matches("server") {
        if let Some(listen) = server.value_of("listen") {
            server::new(|| {
                App::new()
                    .middleware(Logger::default())
                    .resource("/greet", |r| r.method(http::Method::POST).with(greet))
            })
            .bind(listen)
            .expect("Cannot bind to port")
            .run();
        } else {
            println!("Please provide a listen address to bind to (--listen).");
        }
    }
}
