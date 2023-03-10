use batch_rename::Params;
use std::{env, process};

fn main() {
    let mut args = env::args();

    let params = Params::build(&mut args).unwrap_or_else(|e| {
        eprintln!("Error parsing arguments: {e}");
        process::exit(1);
    });

    println!("params: {:?}", params);

    if let Err(e) = batch_rename::batch_rename(params.dir_path, &params.batch_name) {
        eprintln!("Error processing: {}", e);
    }
}
