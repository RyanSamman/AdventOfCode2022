use std::collections::HashMap;

#[derive(Debug)]
pub struct Directory {
    _name: String,
    _index: usize,
    files: HashMap<String, u64>,
    subdirectories: HashMap<String, usize>,
}

impl Directory {
    pub fn new(name: String, index: usize) -> Self {
        Directory {
            _name: name, _index: index,
            files: HashMap::new(),
            subdirectories: HashMap::new(),
        }
    }

    pub fn get_directory_index(&self, dirname: &String) -> Option<&usize> {
        self.subdirectories.get(dirname)
    }

    pub fn add_subdirectory(&mut self, dirname: String, dir_index: usize) {
        self.subdirectories.insert(dirname, dir_index);
    }

    pub fn add_file(&mut self, filename: String, file_size: u64) {
        self.files.insert(filename, file_size);
    }

    pub fn total_file_size(&self) -> u64 {
        self.files.values().sum()
    }

}

#[derive(Debug)]
pub struct DirectoryManager {
    current_directory: Vec<usize>,
    directories: Vec<Directory>,
}

impl DirectoryManager {
    pub fn new() -> Self {
        let mut directories = Vec::new();
        let mut current_directory = Vec::new();

        directories.push(Directory::new("root".to_string(), 0));
        current_directory.push(0);

        DirectoryManager {
            current_directory,
            directories,
        }
    }

    pub fn cd(&mut self, dir: &str) {
        match dir {
            ".." => {
                self.current_directory.pop();
            }
            "/" => {
                self.current_directory.clear();
                self.current_directory.push(0);
            }
            dir => {
                let i = self.get_dir_index(dir);
                self.current_directory.push(i);
            }
        }
    }

    pub fn create_dir(&mut self, dir: &str) {
        let index = self.directories.len();
        let cwd = self.get_current_directory_mut();

        if let None = cwd.get_directory_index(&dir.to_string()) {
            cwd.add_subdirectory(dir.to_string(), index);
            self.directories
                .push(Directory::new(dir.to_string(), index));
        }
    }

    pub fn create_file(&mut self, filename:&str, filesize: u64) {
        let cwd = self.get_current_directory_mut();
        cwd.add_file(filename.to_string(), filesize);
    }

    pub fn compute_dir_sizes(&self) -> Vec<u64> {
        self.directories.iter()
                        .map(|dir| self.recursive_dir_size(&dir))
                        .collect()
    }

    fn recursive_dir_size(&self, dir: &Directory) -> u64 {

        fn f(d: &Directory, ds: &Vec<Directory>) -> u64 {
            let file_sizes = d.total_file_size();

            if d.subdirectories.is_empty() {
                return file_sizes
            }

            return file_sizes + d.subdirectories
                                 .iter()
                                 .map(|(_, i)| &ds[*i])
                                 .map(|d| f(d, ds))
                                 .sum::<u64>()
        }

        f(dir, &self.directories)
    }

    fn get_current_directory(&self) -> &Directory {
        let i = *self.current_directory.last().unwrap();
        &self.directories[i]
    }

    fn get_current_directory_mut(&mut self) -> &mut Directory {
        let i = *self.current_directory.last().unwrap();
        &mut self.directories[i]
    }

    fn get_dir_index(&self, dir: &str) -> usize {
        let cwd = self.get_current_directory();
        *cwd.get_directory_index(&dir.to_string()).unwrap()
    }
}
