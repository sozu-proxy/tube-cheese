#[macro_use] extern crate hyper;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
extern crate mio;
extern crate mio_uds;
extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
extern crate pretty_env_logger;
extern crate url;
extern crate serde;
extern crate serde_json;
extern crate sozu_command_lib as sozu_command;

use clap::{App,Arg};

use sozu_command::config::Config;



mod api;
mod config;
mod providers;


fn main() {
  pretty_env_logger::init().unwrap();
  let matches = App::new("traefik-manager-bin")
                        .version(crate_version!())
                        .about("configuration tool")
                        .arg(Arg::with_name("api")
                            .long("api")
                            .value_name("traefik API")
                            .help("traefik API URL")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Sets a custom config file")
                            .takes_value(true)
                            .required(true))
                        .get_matches();


    let config_file = matches.value_of("config").expect("required config file");
    let config      = Config::load_from_path(config_file).expect("could not parse configuration file");
    let config_tx   = config::driver(config.command_socket);

    let url = matches.value_of("api").expect("required config file");
    let url = hyper::Url::parse(&url).expect("invalid url");
    if url.scheme() != "http" {
        println!("This example only works with 'http' URLs.");
        return;
    }

    api::driver(url, config_tx);
}
