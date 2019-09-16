use std::path::PathBuf;

pub struct Ignore {
    patterns: Vec<String>,
}

impl Ignore {
    pub fn new() -> Ignore {
        Ignore {
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn ignore_item(&self, path: &PathBuf) -> bool {
        let file_name: String = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_owned()
            .to_string();
        self.patterns.contains(&file_name)
    }

    pub fn ignore_items(&self, paths: Vec<PathBuf>) -> Vec<PathBuf> {
        paths
            .into_iter()
            .filter_map(|p| {
                if self.ignore_item(&p) {
                    return None;
                }
                Some(p)
            })
            .collect()
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
