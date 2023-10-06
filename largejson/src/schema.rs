use std::fs::File;

use serde::{Deserialize, Serialize};
use schemars::{schema_for, JsonSchema};

use crate::error::MyError;

// Describe a person using a simple data structure
#[derive(JsonSchema, Serialize, Deserialize)]
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

/// Create and return the JsonSchema for the [Person] object
pub fn schema_person_string() -> Result<String, MyError> {
    let my_schema = schema_for!(Person);
    let my_schema_json = serde_json::to_string_pretty(&my_schema)?;
    let my_schema_string = my_schema_json;
    // println!("{my_schema_string}");
    Ok(my_schema_string)
}

/// Dynamic generate of JsonSchema
///
/// Can use this to generate the JsonSchema for any [Serialisable] object rather than writing one function for each object.
/// This function can be used instead of [stdout_schema_list] and [schema_person_string]
pub fn schema_string<MyType: JsonSchema>() -> Result<String, MyError> {
    let my_schema = schema_for!(MyType);
    let my_schema_json = serde_json::to_string_pretty(&my_schema)?;
    let my_schema_string = my_schema_json;
    Ok(my_schema_string)
}
