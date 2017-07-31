extern crate crypto;
extern crate rand;
#[macro_use]
extern crate clap;
extern crate base64;
extern crate mix;


use std::str;
use std::io;
use std::io::prelude::*;
use std::process;
use clap::{Arg, App, SubCommand, AppSettings};


macro_rules! fatal {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(_) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
                process::exit(1);
            }
        }
    )
}

fn stdin() -> Vec<u8> {
    let mut stdin = io::stdin();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf).expect("No stdin");
    buf
}


fn main() {
    let app = App::new("git-mix")
        .version(crate_version!())
        .about("Mix the git data with AES-256-ECB mode")
        .author("detailyang <detailyang@gmail.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("encrypt")
                .about("Encrypt the git data")
                .arg(
                    Arg::with_name("key")
                        .required(true)
                        .short("k")
                        .long("key")
                        .takes_value(true)
                        .help("Set the key of AES-256-ECB"),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .about("Decrypt the git data")
                .arg(
                    Arg::with_name("key")
                        .required(true)
                        .short("k")
                        .long("key")
                        .takes_value(true)
                        .help("Set the key of AES-256-ECB"),
                ),
        )
        .subcommand(
            SubCommand::with_name("gen")
                .about("Generate .git/config template")
                .arg(
                    Arg::with_name("key")
                    .short("k")
                    .long("key")
                    .takes_value(true)
                    .help("Use the specified key")
                )
        )
        .subcommand(
            SubCommand::with_name("genkey")
                .about("Generate the key of AES-256-ECB")
        );

    match app.get_matches().subcommand() {
        ("encrypt", Some(e)) => {
            let key = e.value_of("key").expect("key requires 32 bytes");
            if key.len() != 32 {
                fatal!("{}", "key requires 32 bytes");
            }

            match mix::encrypt(stdin().as_slice(), key.as_bytes()) {
                Ok(cipher) => print!("{}", cipher),
                Err(e) => fatal!("{}", e),
            }
        }

        ("decrypt", Some(d)) => {
            let key = d.value_of("key").expect("key requires 32 bytes");
            if key.len() != 32 {
                fatal!("key requires 32 bytes");
            }

            match mix::decrypt(stdin().as_slice(), key.as_bytes()) {
                Ok(plain) => print!("{}", plain),
                Err(e) => fatal!("{}", e),
            }
        }

        ("gen", Some(g)) => {
            let key = g.value_of("key");
            let template = mix::gen(key);
            print!("{}", template);
        }

        _ => {
            let key = mix::genkey(32);
            print!("{}", key);
        }
    }
}