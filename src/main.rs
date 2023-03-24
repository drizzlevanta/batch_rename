use std::process;

use batch_rename::params::Params;
use clap::Parser;

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

// fn main() -> std::io::Result<()> {
//     let path = Path::new("path/to/directory");
//     let pattern = "old_pattern";
//     let replace_with = "new_pattern";

//     for entry in fs::read_dir(path)? {
//         let entry = entry?;
//         let file_name = entry.file_name().into_string().unwrap();

//         if file_name.contains(pattern) {
//             let new_file_name = file_name.replace(pattern, replace_with);
//             let old_path = entry.path();
//             let new_path = old_path.with_file_name(new_file_name);
//             fs::rename(old_path, new_path)?;
//         }
//     }

//     Ok(())
// }
// In this example, we use the read_dir function to iterate over all entries in the directory, and then we check if the file name contains the pattern we're looking for using the contains method on the file name. If the file has the pattern we're looking for, we use the replace method to create a new file name with the pattern replaced by the new pattern. We then use the with_file_name method to create a new path with the new file name, and then use the rename function to rename the file.

//   // Check if the new file name already exists, add counter if necessary
//   while Path::new(&new_file_name).exists() {
//     new_file_name = format!("{}-{}", replace_with, counter);
//     counter += 1;
// }

// let new_path = old_path.with_file_name(new_file_name);
// fs::rename(old_path, new_path)?;

// use regex::Regex;

// fn main() {
//     let input_str = "The quick brown fox";
//     let regex_str = r"(?P<first_word>\w+)";
//     let replace_str = "lazy $first_word";
//     let re = Regex::new(regex_str).unwrap();
//     let result = re.replace(input_str, replace_str);
//     println!("{}", result);
// }
