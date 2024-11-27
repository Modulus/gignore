use std::fs;
use git2::Repository;


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