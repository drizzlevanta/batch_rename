use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::Parser;

pub fn batch_rename(params: Params) -> io::Result<usize> {
    println!("params:{:?}", params);
    let mut files = access_dir(&params.dir_path)?;
    sort_dir(&mut files, params.alphanumeric_sort);
    rename_dir(
        &files,
        &params.batch_name,
        params.extension.as_deref(),
        &params.dir_path,
    )
}

fn access_dir<P>(path: P) -> io::Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let files = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .filter_map(|x| x.ok())
        .collect::<Vec<_>>();

    Ok(files)
}

fn sort_dir(files: &mut Vec<PathBuf>, is_alphanumeric_sort: bool) {
    if is_alphanumeric_sort {
        files.sort_by(|a, b| alphanumeric_sort::compare_path(a, b));
    } else {
        files.sort();
    }
}

fn rename_dir<P>(
    files: &Vec<PathBuf>,
    new_batch_name: &str,
    ext: Option<&str>,
    path: P,
) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    fs::rename("a.txt", "a.txt")?;
    // let path_name = file.parent().unwrap().display();
    let path = path.as_ref().display();
    for (index, file) in files.iter().enumerate() {
        fs::rename(
            file,
            format!(
                r"{}\{}_{}.{}",
                path,
                new_batch_name,
                index + 1,
                ext.unwrap_or_else(|| { file.extension().unwrap_or_default().to_str().unwrap() })
            ),
        )?;
    }
    Ok(files.len())
}

// let file_name = file.file_name().unwrap().to_str().unwrap();
// let (_, suffix) = file_name.rsplit_once('-').unwrap();

#[derive(Debug, Default, Parser)]
#[clap(author = "LZ", version = "1.0.0", about)]
///Batch rename files in a directory, with auto-numbering as suffix.
pub struct Params {
    #[clap(short = 'd', long = "dir-path")]
    #[clap(default_value = ".")]
    ///Path to the directory. Defaults to current directory
    pub dir_path: String,

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

// impl Params {
//     pub fn build<A>(args: &mut A) -> Result<Params, &'static str>
//     where
//         A: Iterator<Item = String>,
//     {
//         args.next();
//         let dir_path: String;
//         let batch_name: String;

//         match args.next() {
//             Some(arg) => dir_path = arg,
//             None => return Err("No directory path provided"),
//         }

//         match args.next() {
//             Some(arg) => batch_name = arg,
//             None => return Err("No batch name provided"),
//         }

//         Ok(Params {
//             dir_path,
//             batch_name,
//             alphanumeric_sort: true,
//             extension: Some(String::from("jpg")),
//         })
//     }
// }
