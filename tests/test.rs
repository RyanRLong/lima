use std::fs;

use lima::file::comparator::{FileCreatedComparator, FileModifiedComparator};

const TEMP_PATH: &str = "./temp";

#[test]
fn it_adds_two() {
    set_up();
    let x: FileModifiedComparator::new(Path::new('.')).unwrap();
    assert!(true);
    tear_down();
}

fn create_test_file() {
    
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


