use std::fs;
use std::path::PathBuf;
use gignore::clone_gitignore_repo;

fn main() -> std::io::Result<()> {

    clone_gitignore_repo("/tmp/github/gitignore", "https://github.com/github/gitignore.git");



    let files = extract_ignore_file_names("/tmp/github/gitignore")?;

    for file in files {
        println!("{:?}", file);
    }

    Ok(())
}



fn extract_ignore_file_names(folder: &str) -> Result<Vec<String>, std::io::Error> {
    let files = fs::read_dir(folder)?.into_iter().filter_map(| entry | {
        let entry = entry.unwrap();
        let current_file_name = entry.path();
        let result = extract_ignore_file_name(current_file_name);
        result
    }).collect::<Vec<_>>();
    Ok(files)
}

fn extract_ignore_file_name(path_buf: PathBuf) -> Option<String> {
    let file_name = path_buf.to_str()?;
    return Some(file_name.split("/").last()?.to_string());
}



#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extract_ignore_file_name_has_only_file_name_after_extraction(){
        let file_name = PathBuf::from("/tmp/github/gitignore/SymphonyCMS.gitignore");
        let result = extract_ignore_file_name(file_name);
        assert!(result.is_some());
        assert!(result.unwrap() == "SymphonyCMS.gitignore");
    }

}