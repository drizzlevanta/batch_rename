use std::process;

use batch_rename::Params;
use clap::Parser;
// use std::env;

fn main() {
    // let mut args = env::args();

    // let params = Params::build(&mut args).unwrap_or_else(|e| {
    //     eprintln!("Error parsing arguments: {e}");
    //     process::exit(1);
    // });

    match Params::try_parse() {
        Err(e) => {
            eprintln!("Invalid params: {e}");
            process::exit(1);
        }
        Ok(params) => match batch_rename::batch_rename(params) {
            Err(e) => {
                eprintln!("Error processing: {e}");
            }
            Ok(len) => {
                println!("{len} files renamed.");
            }
        },
    };
}
