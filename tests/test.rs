
extern crate lima;


use std::fs;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{time, thread};

use lima::file::comparator::FileModifiedComparator;

const TEMP_PATH: &str = "./temp/";

#[test]
fn it_adds_two() {
    // set_up();
    // create_test_file("test.txt").unwrap();
    // let mut path_string: String = TEMP_PATH.to_owned();
    // path_string.push_str(TEMP_FILE);
    // let path = Path::new(&path_string);
    // let x: FileModifiedComparator = FileModifiedComparator::new(path);
    // assert!(true);
    // delete_test_file().unwrap();
    // tear_down();
}

#[test]
fn it_gives_age_in_seconds() {
    set_up();
    let path = create_test_file("test_file.txt").unwrap();
    let x = FileModifiedComparator::new(Path::new(&path));
    thread::sleep(time::Duration::from_secs(6));
    assert_eq!(6, x.last_modified_in_secs());
    tear_down();
}


#[test]
fn it_can_tell_if_older_than_in_seconds() {
    set_up();
    let path = create_test_file("test_file.txt").unwrap();
    let x = FileModifiedComparator::new(Path::new(&path));
    thread::sleep(time::Duration::from_secs(6));
    assert!(true, x.is_older_than_secs(5));
    tear_down();
}

fn create_test_file(file_name: &str) -> std::io::Result<(String)> {
    let mut path_string: String = TEMP_PATH.to_owned();
    path_string.push_str(file_name);
    let mut file = File::create(&path_string).expect("Issue creating file:");
    file.write_all(b"This is a test file")
        .expect("Could not write to file");
    Ok(path_string)
}

#[warn(dead_code)]
fn delete_test_file(file_name: String) -> std::io::Result<()> {
    let mut path_string: String = TEMP_PATH.to_owned();
    path_string.push_str(&file_name);
    if Path::new(&path_string).exists() {
        fs::remove_file(path_string)
    } else {
        Ok(())
    }
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

