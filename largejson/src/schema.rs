use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::error::MyError;

// Describe a person using a simple data structure
#[derive(Serialize, Deserialize)]
pub struct Person {
    /// Name of the Person
    pub name: String,
    /// Persons age
    pub age: u32,
}

// Write a JSON file with with [count] copies of the [Person] object
///
/// * `filename` - A string slice that naem fo the file to generate
/// * `count` - a unsigned int to define how many copies of [Person] to generate
pub fn write_records(filename: &str, count: u32) -> Result<(), MyError> {

    let mydata: Vec<_> = (0..count).map(|x| Person{ name: format!("name-{:08}", x), age: x, }).collect();
    let file = File::create(filename)?;

    serde_json::to_writer(file,&mydata)?;

    Ok(())
}
