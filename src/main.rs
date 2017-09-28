
extern crate clap;
#[macro_use]
extern crate quick_error;
extern crate asmuth_bloom_secret_sharing;

mod args;

use std::io::stdin;
use std::io::prelude::*;
use std::str::FromStr;
use asmuth_bloom_secret_sharing::{AsmuthBloomShare, AsmuthBloomRecover, ABSharedSecret};
use std::string::ToString;

type RuntimeError = Box<std::error::Error>;

fn run() -> Result<(), RuntimeError> {
    let args = args::parse_args();
    match args.subcommand() {
        ("generate", Some(sub_args)) => {
            let secret = match sub_args.value_of("secret") {
                Some(secret) => String::from(secret),
                None => {
                    let mut secret = String::new();
                    stdin().read_to_string(&mut secret)?;
                    secret
                }
            };

            if secret.len() < 1 {
                return Err("Empty secret".into());
            }

            let threshold = args::parse_threshold(&sub_args)?;

            let number = match sub_args.value_of("number") {
                Some(n) => u16::from_str(n)?,
                None => threshold,
            };

            let bits = match sub_args.value_of("bits") {
                Some(n) => u16::from_str(n)?,
                None => (secret.len() * 8)  as u16
            };

            if bits > 8000 {
                return Err("This is bit too large secret for this tool".into());
            }

            if number < threshold {
                return Err(
                    "Number of shares must be greater or equal to threshold".into(),
                );
            }
            let mut gen = AsmuthBloomShare::new(bits, number, threshold, 1e-9);
            let share = gen.create_share(secret.as_bytes())?;
            print!("{}", share.to_string());


        }
        ("recover", Some(sub_args)) => {
            let threshold = args::parse_threshold(&sub_args)?;
            let  num_shares = sub_args.occurrences_of("share");
            let mut shares = String::new();
            if num_shares == 0 {
                stdin().read_to_string(&mut shares)?;
                
            } else {
                for share in sub_args.values_of("share").unwrap() {
                    shares.push_str(share);
                    shares.push_str("\n");
                }
            }

            

            let rec = AsmuthBloomRecover::new(threshold);
            let shares = ABSharedSecret::from_str(&shares)?;
            let secret = rec.recover_secret(&shares)?;
            let secret = String::from_utf8(secret)?;
            print!("{}", secret);
        }
        _ => panic!("Invalid subcommand"),

    }


    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Runtime error: {}", e);
        match e.cause() {
            Some(err) => println!("Caused by {}", err ),
            None => ()
        }

        std::process::exit(1);
    }
}
