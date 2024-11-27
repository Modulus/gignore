use std::fs;
use git2::Repository;
use std::path::PathBuf;

pub fn clone_gitignore_repo(local_repo_path: &str, repo_url: &str) {
    if !fs::exists(&local_repo_path).expect("Folder does not exist, creating") {
        let _ = match Repository::clone(repo_url, &local_repo_path) {
            Ok(repo) => {
                println!("cloned gitignore repo to {:?}", repo.path());
                repo
            }
            Err(e) => panic!("failed to clone: {}", e),
        };
        
    }
}


pub fn extract_ignore_file_names(folder: &str) -> Result<Vec<String>, std::io::Error> {
    let files = fs::read_dir(folder)?.into_iter().filter_map(| entry | {
        let entry = entry.unwrap();
        let current_file_name = entry.path();
        let result = extract_ignore_file_name(current_file_name);
        result
    }).collect::<Vec<_>>();
    Ok(files)
}

pub fn extract_ignore_file_name(path_buf: PathBuf) -> Option<String> {
    let file_name = path_buf.to_str()?;
    return Some(file_name.split("/").last()?.to_string());
}

pub fn get_prefix(file_name: &str) -> Option<String> {
    let prefix = file_name.split(".").next()?;
    return Some(prefix.to_string());
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

    #[test]
    fn test_get_prefix_from_file_name_returns_valid_prefix(){
        let file_name = "SymphonyCMS.gitignore";
        let result = get_prefix(file_name);
        assert!(result.is_some());
        assert!(result.unwrap() == "SymphonyCMS");
    }

}