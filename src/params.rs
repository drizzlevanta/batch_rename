use std::path::PathBuf;

use clap::Parser;
#[derive(Debug, Default, Parser)]
#[clap(author = "LZ", version = "1.0.0", about)]
///Batch rename files in a directory, with auto-numbering as suffix.
pub struct Params {
    #[clap(short = 'd', long = "dir-path")]
    #[clap(default_value = ".")]
    ///Path to the directory. Defaults to current directory
    pub dir_path: PathBuf,

    ///Prefix for the renaming
    // #[clap(short = 'n', long = "batch-name")]
    pub batch_name: String,

    ///Using alphanumeric sort prior to renaming files. Defaults to true. If set to false, regular sort will be used.
    #[clap(short = 'a', long = "alphanumeric-sort")]
    #[clap(default_value_t = true)]
    pub alphanumeric_sort: bool,

    ///New file extension
    #[clap(short = 'e', long = "extension")]
    pub extension: Option<String>,
}
