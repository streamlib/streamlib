extern crate directories;
extern crate git2;

use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

const LIBRARY_REPO: &str = "https://github.com/streamlib/library";

pub struct Git {
    pub path: PathBuf,
    pub repo: Option<git2::Repository>
}

impl Git {

    pub fn new() -> Self {
        let dirs = directories::ProjectDirs::from("", "", "streamlib").unwrap();
        let mut path = PathBuf::from(dirs.config_dir());
        path.push("library");

        Git {
            path: path,
            repo: None
        }
    }

    pub fn init(mut self) {
        self.open();
        if self.update_required() {
            self.update().unwrap();
        }
    }

    fn open(&mut self) {
        // open the library git repository, or clone it if it does not exist
        if !self.path.exists() {
            fs::create_dir_all(&self.path).unwrap();
        }

        self.repo = match git2::Repository::open(&self.path) {
            Ok(repo) => Some(repo),
            Err(_e) => {
                match git2::Repository::clone(LIBRARY_REPO, &self.path) {
                    Ok(repo) => Some(repo),
                    Err(e) => panic!("failed to open: {}", e),
                }
            }
        };
    }

    fn update_required(&self) -> bool {
        // calculate if an update is required by checking the modified timestamp
        // on the .git/FETCH_HEAD file, which reflects when the last fetch occured
        let mut file = PathBuf::from(&self.path);
        file.push(".git");
        file.push("FETCH_HEAD");
        let modified = fs::metadata(file).unwrap().modified().unwrap();
        let now = SystemTime::now();
        let secs = now.duration_since(modified).unwrap().as_secs();
        secs > (60 * 60 * 24 * 7)
    }

    fn update(self) -> Result<(), git2::Error> {
        // pull implementation (fetch heads, reset hard) borrowed from
        // https://github.com/rust-lang/crates.io/blob/6a44062edc2ec99e30a7770bbcc97d9cec110dd1/src/git.rs#L123-L128
        let repo = self.repo.unwrap();
        let mut origin = repo.find_remote("origin")?;
        origin.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
        let head = repo.head().unwrap().target().unwrap();
        let obj = repo.find_object(head, None)?;
        repo.reset(&obj, git2::ResetType::Hard, None)?;
        Ok(())
    }

}

#[cfg(test)]
mod tests {

    use super::Git;

    #[test]
    fn test_git_build_path() {
        let mut g = Git::new();
        g.open();
        if g.update_required() {
            g.update().unwrap();
        }
    }
}
