use std::process;

use batch_rename::params::Params;
use clap::Parser;

fn main() {
    match Params::try_parse() {
        Err(e) => {
            eprintln!("Invalid params: {e}");
            process::exit(1);
        }
        Ok(params) => match batch_rename::file_ops::batch_rename(params) {
            Err(e) => {
                eprintln!("Error processing: {e}");
            }
            Ok(len) => {
                println!("{len} files renamed.");
            }
        },
    };
}
