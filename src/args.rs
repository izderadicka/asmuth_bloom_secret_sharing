use clap::{ArgMatches,App, SubCommand, Arg};
use std::str::FromStr;

pub fn  parse_args<'a>() -> ArgMatches<'a> {
App::new("asmuth_bloom_secret_sharing")
            .version("0.2.1")
            .about("secret sharing with Asmut- Bloom scheme")
            .author("Ivan Zderadicka <ivan@zderadicka.eu>")
            .subcommand(SubCommand::with_name("generate")
                .about("generates shared secrets")
                .arg(Arg::with_name("secret")
                    .value_name("SECRET")
                    .help("secret to share, if not provided, reads it from stdin")
                )
                .arg(Arg::with_name("threshold")
                    .short("t")
                    .long("threshold")
                    .required(true)
                    .takes_value(true)
                    .help("threshold - number of shared secrets needed to reconstruct original value")
                )
                .arg(Arg::with_name("number")
                    .short("n")
                    .long("number")
                    .takes_value(true)
                    .help("number of shared secrets - defaults to threshold (if provided must be equal or \
                    bigger then threshold)")
                )
                .arg(Arg::with_name("bits")
                    .short("b")
                    .long("bits")
                    .takes_value(true)
                    .help("maximum number of bits of the secret, also hides actual size of the 
                    secret, if not provided uses current secret size")
                )
            )
            .subcommand(SubCommand::with_name("recover")
                .about("recovers originalsecret from shared secrets")
                .arg(Arg::with_name("share")
                    .value_name("SHARE")
                    .multiple(true)
                    .help("Shared secrets, must provide at least threshold shares, \
                    if not provided on command line read from stdin")
                )
                .arg(Arg::with_name("threshold")
                    .short("t")
                    .long("threshold")
                    .required(true)
                    .takes_value(true)
                    .help("threshold - number of shared secrets needed to reconstruct original value")
                )

            )
            .get_matches()
}

quick_error! {
    #[derive(Debug)]
    pub enum Error{
        ThresholdParsingError(err: ::std::num::ParseIntError){
            from()
            cause(err)
        }
    }
}

pub fn parse_threshold(args: &ArgMatches) -> Result<u16, Error> {
    match args.value_of("threshold") {
                Some(n) => Ok(u16::from_str(n)?),
                None => unreachable!()
    }
}