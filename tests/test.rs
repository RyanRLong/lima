use std::fs::File;
use std::path::Path;

use lima::file::comparator::{FileCreatedComparator, FileModifiedComparator};

const TEMP_PATH: &str = "./temp";
const TEMP_FILE: &str = "/test.txt";

#[test]
fn it_adds_two() {
    set_up();
    let path_string: String = TEMP_PATH.to_owned();
    path_string.push_str(TEMP_FILE);
    let path = Path::new(&path_string);
    let x: FileModifiedComparator = FileModifiedComparator::new(path);
    assert!(true);
    tear_down();
}

fn create_test_file() -> std::io::Result<()> {
    let mut file = File::create("test.txt").expect("Issue creating file:");
    file.write_all(b"This is a test file").expect("Could not write to file");
    Ok(())
}

fn create_test_directory(path: &str) -> std::io::Result<()> {
    fs::create_dir(path)?; 
    Ok(())
}

fn delete_test_directory(path: &str) -> std::io::Result<()> {
    fs::remove_dir_all(path).expect("Could not delete directory");
    Ok(())
}

fn set_up() {
    create_test_directory(TEMP_PATH).expect("Could not create directory");
}

fn tear_down() {
    delete_test_directory(TEMP_PATH).expect("Could not delete directory");
}


