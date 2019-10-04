extern crate directories;

use directories::ProjectDirs;

struct Git {
    pub dirs: ProjectDirs
}

impl Git {

    pub fn new() -> Self {
        Git {
            dirs: ProjectDirs::from("", "", "streamlib").unwrap()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Git;
    use std::path::PathBuf;


    #[test]
    fn test_git_build_path() {
        let g = Git::new();
        let mut p = PathBuf::from(g.dirs.config_dir());
        p.push("library");
    }
}
