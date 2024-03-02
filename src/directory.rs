pub mod FileOperation {

    use clap::{App, Arg};
    use crate::file::FileManager::file;
    use std::collections::HashMap;

    trait DisplayFiles<'a> {
        fn list_files(&self);
    }

    trait FileOperation<'a, T> {
        fn create_file(&mut self, file_name: String, new_file: file<T>);
        fn delete_file(&mut self, file_name: String) -> Result<(), String>;
    }

    #[derive(Debug)]
    pub struct Directory<T> {
        pub files: HashMap<String, file<T>>,
    }

    impl<'a, T: std::fmt::Debug> Directory<T> {
        pub fn display_directory(&self) {
            println!("Directory : ");
            println!("Files : {:?}", self.files);
        }

        pub fn create_file(&mut self, file_name: String, new_file: file<T>) {
            self.files.insert(file_name, new_file);
        }

        pub fn delete_file(&mut self, file_name: String) -> Result<(), String> {
            if self.files.remove(&file_name).is_some() {
                Ok(())
            } else {
                Err(format!(
                    "Cannot delete file '{}'. File not found.",
                    file_name
                ))
            }
        }
    }

    impl<'a, T: std::fmt::Debug> DisplayFiles<'a> for Directory<T> {
        fn list_files(&self) {
            println!("Directory Files : ");
            println!("{:?}", self.files);
        }
    }

    impl<'a, T> Directory<T> {
        pub fn new_directory() -> Directory<T> {
            Directory {
                files: HashMap::new(),
            }
        }
    }

    impl<'a, T: std::fmt::Debug> FileOperation<'a, T> for Directory<T> {
        fn create_file(&mut self, file_name: String, new_file: file<T>) {
            self.files.insert(file_name, new_file);
        }

        fn delete_file(&mut self, file_name: String) -> Result<(), String> {
            if self.files.remove(&file_name).is_some() {
                Ok(())
            } else {
                Err(format!(
                    "Cannot delete file '{}'. File not found.",
                    file_name
                ))
            }
        }
    }
}