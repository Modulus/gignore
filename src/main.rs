

use gignore::{clone_gitignore_repo, extract_ignore_file_names};

// If you do not import module, tests in it will not be run...
mod utilities;

fn main() -> std::io::Result<()> {

    clone_gitignore_repo("/tmp/github/gitignore", "https://github.com/github/gitignore.git");


    let files = extract_ignore_file_names("/tmp/github/gitignore")?;

    for file in files {
        println!("{:?}", file);
    }

    let first : String ="Go".into();
    let second : String = "Golang".into();

    println!("{:?}", first.to_lowercase().contains(second.to_lowercase().as_str()));
    println!("{:?}", second.to_lowercase().contains(first.to_lowercase().as_str()));
    Ok(())
}


