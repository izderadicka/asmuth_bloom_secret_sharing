use clap::{ArgMatches,App, SubCommand, Arg};
use std::str::FromStr;

#[allow(dead_code)]
pub mod built_info {
   // The file has been placed there by the build script.
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}



pub fn  parse_args<'a>() -> ArgMatches<'a> {
let version = match built_info::GIT_VERSION {
                Some(ref v) => format!("{} git:{}",built_info::PKG_VERSION,v),
                None => built_info::PKG_VERSION.into()};
App::new(built_info::PKG_NAME)
            .version(&version[..])
            .about(built_info::PKG_DESCRIPTION)
            .author(built_info::PKG_AUTHORS)
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
                .about("recovers original secret from shared secrets")
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
        ThresholdTooSmall {}
    }
}

pub fn parse_threshold(args: &ArgMatches) -> Result<u16, Error> {
    match args.value_of("threshold") {
                Some(n) => {
                    let t = u16::from_str(n)?;
                    if t<2 {
                        return Err(Error::ThresholdTooSmall);
                    }
                    Ok(t)},
                None => unreachable!()
    }
}