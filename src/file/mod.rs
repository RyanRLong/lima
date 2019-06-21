pub mod comparator {

    extern crate walkdir;

    use std::cmp::Ordering;
    use std::fs::File;
    use std::io::Read;

    use std::path::Path;


    #[derive(Debug)]
    pub struct FileModifiedComparator<'a> {
        path: &'a Path,
    }

    pub const DAY: u64 = HOUR * 24;
    pub const HOUR: u64 = MINUTE * 60;
    pub const MINUTE: u64 = 60;

    impl<'a> FileModifiedComparator<'a> {
        pub fn new(path: &'a Path) -> FileModifiedComparator {
            let path: &'a Path = path;
            FileModifiedComparator { path }
        }

        pub fn last_modified_in_secs(&self) -> u64 {
            self.path
                .metadata()
                .unwrap()
                .modified()
                .unwrap()
                .elapsed()
                .unwrap()
                .as_secs()
        }

        pub fn is_older_than_secs(&self, secs: u64) -> bool {
            self.last_modified_in_secs() > secs
        }

        pub fn is_older_than_days(&self, days: u64) -> bool {
            self.last_modified_in_secs() / DAY > days
        }

        pub fn get_path_as_string(&self) -> Option<&str> {
            self.path.to_str()
        }

        pub fn get_path(&self) -> &std::path::Path {
            self.path
        }

        pub fn get_contents(&self) -> std::vec::Vec<u8> {
            let mut file_content = Vec::new();
            let mut file = File::open(self.get_path()).expect("Unable to open file");
            file.read_to_end(&mut file_content).expect("Unable to read");
            file_content
        }
    }

    impl<'a> PartialOrd for FileModifiedComparator<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(
                self.last_modified_in_secs()
                    .cmp(&other.last_modified_in_secs()),
            )
        }
    }

    impl<'a> Ord for FileModifiedComparator<'a> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.last_modified_in_secs()
                .cmp(&other.last_modified_in_secs())
        }
    }

    impl<'a> PartialEq for FileModifiedComparator<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.last_modified_in_secs() == other.last_modified_in_secs()
        }
    }

    impl<'a> Eq for FileModifiedComparator<'a> {}
}
