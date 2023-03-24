use chrono::Utc;

use crate::params::Params;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn batch_rename(params: Params) -> io::Result<usize> {
    println!("params:{:?}", params);
    let mut files = access_dir(&params.dir_path)?;

    backup_dir(&files, &params.dir_path)?;

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
        .filter(|p| p.is_file())
        .collect::<Vec<_>>();

    println!("{} files found, excluding any directory.", files.len());
    Ok(files)
}

fn sort_dir(files: &mut Vec<PathBuf>, is_alphanumeric_sort: bool) {
    println!("Sorting files...");
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
    println!("Renaming files...");

    let path = path.as_ref().display();
    let mut counter: usize = 0;

    //allocate a vector to store temp files
    let mut pending_files: Vec<(PathBuf, PathBuf)> = Vec::new();

    for (index, file) in files.iter().enumerate() {
        let new_file_name = PathBuf::from(format!(
            r"{}\{}_{}.{}",
            path,
            new_batch_name,
            index + 1,
            ext.unwrap_or_else(|| { file.extension().unwrap_or_default().to_str().unwrap() })
        ));

        //handle file name collision
        if new_file_name.is_file() {
            //TODO try regular expression
            let file_stem = new_file_name.file_stem().unwrap();
            file_stem.to_str().replace("_temp");
            let file_ext = new_file_name.extension();

            let mut temp_file_name = new_file_name.as_path().to_owned();
            let temp = format!("{}_temp", file_stem.to_str().unwrap());
            temp_file_name.set_file_name(temp);
            if let Some(ext) = file_ext {
                //preserve file extension
                temp_file_name.set_extension(ext);
            }

            //rename files to temp files
            fs::rename(file, &temp_file_name)?;
            pending_files.push((temp_file_name, new_file_name));
            continue;
        }

        //rename files which do not have any name collision
        fs::rename(file, &new_file_name)?;
        counter += 1;
    }

    println!("Number of temp files: {}", pending_files.len());

    //rename temp files
    for (from, to) in pending_files.into_iter() {
        fs::rename(from, to)?;
        counter += 1;
    }

    //return the number of files renamed
    Ok(counter)
}

fn backup_dir<P>(files: &Vec<PathBuf>, dir: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    println!("Backing up files...");

    //get current time
    let current_time = Utc::now().format("%Y-%m-%d-%H-%M-%S");

    //create backup directory
    let backup_dir = dir
        .as_ref()
        .join(format!("batch_rename_backup_{}", current_time));

    println!("Creating backup directory...{}", &backup_dir.display());
    fs::create_dir(&backup_dir)?;

    for file in files.iter() {
        let file_name = file.file_name().unwrap();
        let dst = backup_dir.join(file_name);
        fs::copy(file, dst)?;
    }

    println!("Finished backing up files");

    Ok(())
}
