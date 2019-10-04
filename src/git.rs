extern crate directories;
extern crate git2;

use directories::ProjectDirs;
use git2::{Repository, ResetType, Error};
use std::path::PathBuf;
use std::fs::create_dir_all;

const LIBRARY_REPO: &str = "https://github.com/streamlib/library";

struct Git {
    pub dirs: ProjectDirs,
    pub repo: Option<Repository>
}

impl Git {

    pub fn new() -> Self {
        Git {
            dirs: ProjectDirs::from("", "", "streamlib").unwrap(),
            repo: None
        }
    }

    pub fn init(&mut self) {
        let mut repo_path = PathBuf::from(self.dirs.config_dir());
        repo_path.push("library");

        if !repo_path.exists() {
            create_dir_all(&repo_path).unwrap();
        }

        self.repo = match Repository::open(&repo_path) {
            Ok(repo) => Some(repo),
            Err(_e) => {
                match Repository::clone(LIBRARY_REPO, repo_path) {
                    Ok(repo) => Some(repo),
                    Err(e) => panic!("failed to open: {}", e),
                }
            }
        };
    }

    pub fn update(self) -> Result<(), Error> {
        // pull implementation (fetch heads, reset hard) borrowed from
        // https://github.com/rust-lang/crates.io/blob/6a44062edc2ec99e30a7770bbcc97d9cec110dd1/src/git.rs#L123-L128
        let repo = self.repo.unwrap();
        let mut origin = repo.find_remote("origin")?;
        origin.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
        let head = repo.head().unwrap().target().unwrap();
        let obj = repo.find_object(head, None)?;
        repo.reset(&obj, ResetType::Hard, None)?;
        Ok(())
    }

}

#[cfg(test)]
mod tests {

    use super::Git;

    #[test]
    fn test_git_build_path() {
        let mut g = Git::new();
        g.init();
        g.update().unwrap();
    }
}
