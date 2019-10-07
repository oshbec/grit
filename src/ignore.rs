use std::path::PathBuf;

/// Holds ignore patterns and answers questions about whether stuff should be ignored
pub struct Ignore {
    patterns: Vec<String>,
}

impl Ignore {
    #[cfg(test)]
    pub fn new() -> Ignore {
        Ignore {
            patterns: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    /// Check to see if a given path should be ignored
    pub fn ignore_item(&self, path: &PathBuf) -> bool {
        let file_name: String = path
            .file_name()
            .expect("Couldn't get file name from path")
            .to_string_lossy()
            .to_owned()
            .to_string();
        self.patterns.contains(&file_name)
    }

    /// Filter a list of paths to those that shouldn't be ignored
    pub fn ignore_items(&self, paths: Vec<PathBuf>) -> Vec<PathBuf> {
        paths
            .into_iter()
            .filter(|p| !self.ignore_item(&p))
            .collect()
    }
}

impl Default for Ignore {
    fn default() -> Ignore {
        Ignore {
            patterns: vec![String::from(".git")],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ignore;
    use std::path::PathBuf;

    #[test]
    fn ignores_a_file() {
        let mut ignore = Ignore::new();
        ignore.add_pattern(String::from("README"));
        let should_ignore = ignore.ignore_item(&PathBuf::from("./haha/README"));
        assert_eq!(should_ignore, true);
    }

    #[test]
    fn doesnt_ignore_a_file() {
        let mut ignore = Ignore::new();
        ignore.add_pattern(String::from("README"));
        let should_ignore = ignore.ignore_item(&PathBuf::from("./haha/LICENSE"));
        assert_eq!(should_ignore, false);
    }

    #[test]
    fn ignores_file_from_a_list() {
        let mut ignore = Ignore::new();
        ignore.add_pattern(String::from("README"));
        let files = vec![
            PathBuf::from("./haha/LICENSE"),
            PathBuf::from("./haha/README"),
        ];
        assert_eq!(ignore.patterns.len(), 1);
        assert_eq!(
            ignore.ignore_items(files).get(0).unwrap().to_owned(),
            PathBuf::from("./haha/LICENSE")
        );
    }

    #[test]
    fn ignores_multiple_files_from_a_list() {
        let mut ignore = Ignore::new();
        ignore.add_pattern(String::from("README"));
        ignore.add_pattern(String::from("LICENSE"));

        let files = vec![
            PathBuf::from("./haha/LICENSE"),
            PathBuf::from("./haha/WAT"),
            PathBuf::from("./haha/README"),
            PathBuf::from("./haha/HUH"),
        ];

        let remaining_files = ignore.ignore_items(files);

        assert_eq!(ignore.patterns.len(), 2);
        assert_eq!(
            remaining_files.get(0).unwrap().to_owned(),
            PathBuf::from("./haha/WAT")
        );

        assert_eq!(
            remaining_files.get(1).unwrap().to_owned(),
            PathBuf::from("./haha/HUH")
        );
    }
}
