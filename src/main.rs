mod config;
mod send;

use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("mailpeep")
        .version("0.1.0")
        .author("72nd <msg@72frg.com>")
        .about("send batch mails")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("init").about("initializes a new config file").arg(
                Arg::new("PATH")
                    .about("path to new config file")
                    .value_name("PATH")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            App::new("send")
                .about("sends the mail")
                .arg(
                    Arg::new("config")
                        .about("path to config file")
                        .long("config")
                        .short('c')
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("message")
                        .about("path to text file with the message in it")
                        .long("message")
                        .short('m')
                        .required(true)
                        .takes_value(true),
                ),
        ).get_matches();

    match matches.subcommand() {
        Some(("init", x)) => init_cmd(x),
        Some(("send", x)) => send_cmd(x),
        _ => {}
    };
}

fn init_cmd(matches: &clap::ArgMatches) {
    let path = matches.value_of("PATH").unwrap();
    let cfg = config::Config::new();
    cfg.save(path);
}

fn send_cmd(matches: &clap::ArgMatches) {
    let cfg_path = matches.value_of("config").unwrap();
    let msg_path = matches.value_of("message").unwrap();

    let cfg = config::Config::open(cfg_path);
    send::send(&cfg, msg_path);
}
