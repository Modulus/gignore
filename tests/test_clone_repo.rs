#[cfg(test)]
mod tests {

    use gignore::clone_gitignore_repo;
    use std::fs::{self, remove_dir_all};
    use git2::Repository;

    #[test]
    fn test_clone_gitignore_repo_exists_at_local_path(){
        let folder = "/tmp/testing/test/gitignore";
        let repo_url = "https://github.com/github/gitignore.git";
        clone_gitignore_repo(&folder, &repo_url);


        assert!(fs::exists(folder).expect("Failed to check if folder exists"));
        assert!(false == Repository::open(folder).unwrap().is_empty().unwrap());

        remove_dir_all(folder).expect("Failed to remove folder");

    }
}