
use gignore::{clone_gitignore_repo, extract_ignore_file_names};

fn main() -> std::io::Result<()> {

    clone_gitignore_repo("/tmp/github/gitignore", "https://github.com/github/gitignore.git");


    let files = extract_ignore_file_names("/tmp/github/gitignore")?;

    for file in files {
        println!("{:?}", file);
    }

    Ok(())
}
