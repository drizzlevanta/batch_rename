use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn batch_rename<P>(path: P, new_batch_name: &str) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut files = access_dir(path)?;
    sort_dir(&mut files);
    rename_dir(&files, new_batch_name)
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

fn sort_dir(files: &mut Vec<PathBuf>) {
    files.sort_by(|a, b| alphanumeric_sort::compare_path(a, b));
}

fn rename_dir(files: &Vec<PathBuf>, new_batch_name: &str) -> io::Result<()> {
    for (index, file) in files.iter().enumerate() {
        let new_file_name = file.parent().unwrap().display();
        fs::rename(
            file,
            format!(r"{}\{}_{}.jpg", new_file_name, new_batch_name, index + 1),
        )?;
    }
    Ok(())
}

// let file_name = file.file_name().unwrap().to_str().unwrap();
// let (_, suffix) = file_name.rsplit_once('-').unwrap();

#[derive(Debug)]
pub struct Params {
    pub dir_path: String,
    pub batch_name: String,
}

impl Params {
    pub fn build<A>(args: &mut A) -> Result<Params, &'static str>
    where
        A: Iterator<Item = String>,
    {
        args.next();
        let dir_path: String;
        let batch_name: String;

        match args.next() {
            Some(arg) => dir_path = arg,
            None => return Err("No directory path provided"),
        }

        match args.next() {
            Some(arg) => batch_name = arg,
            None => return Err("No batch name provided"),
        }

        Ok(Params {
            dir_path,
            batch_name,
        })
    }
}
