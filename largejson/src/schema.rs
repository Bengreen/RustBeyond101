use std::{fs::File, io::{Read, BufReader}};

use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use schemars::{schema_for, JsonSchema};
use serde_json::Value;

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
/// Can use this to generate the JsonSchema for any [Serialize] object rather than writing one function for each object.
/// This function can be used instead of [schema_person_string]
///
/// ```
/// # use largejson::error::MyError;
/// # use largejson::schema::Person;
/// # use largejson::schema::schema_string;
/// # fn main() -> Result<(), MyError> {
/// let person_schema = schema_string::<Person>()?;
///
/// assert!(person_schema.len() > 0);
/// println!("Schema = {}", person_schema);
/// # Ok(())
/// # }
/// ```
pub fn schema_string<MyType: JsonSchema>() -> Result<String, MyError> {
    let my_schema = schema_for!(MyType);
    let my_schema_json = serde_json::to_string_pretty(&my_schema)?;
    let my_schema_string = my_schema_json;
    Ok(my_schema_string)
}

pub fn validate_with_schema(filename: &str, error_limit: usize) -> Result<(), MyError> {

    let schema_str = schema_string::<Vec<Person>>()?;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    match validate(&schema_str, reader, error_limit) {
        Ok(data) => {
            if let Value::Array(array) = &data {
                let length = array.len();
                println!("Found array of length {length}")
            }
            let _my_content: Vec<Person> = serde_json::from_value(data)?;

            Ok(())
        },
        Err(err) => Err(err),
    }
}

pub fn validate<R: Read>(schema: &str, reader: R, error_limit: usize) -> Result<Value, MyError> {

    let schema_value = serde_json::from_str(schema)?;

    let compiled_schema = JSONSchema::compile(&schema_value)?;

    let data_value = serde_json::from_reader(reader)?;

    {
        let validation_result = compiled_schema.validate(&data_value);
        if let Err(errors) = validation_result {
            let error_stringified: Vec<_> = errors.take(error_limit).map(|value| {
                format!("Error: instance: {:?}, kind: {:?}, instance_path:: {}, schema_path: {}",
                    value.instance, value.kind, value.instance_path, value.schema_path)
            }).collect();

            return Err(MyError::JsonValidation(error_stringified));
        };
    }

    Ok(data_value)
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn validate_vec() {

        let schema = schema_string::<Vec<Person>>().expect("got schema");

        println!("my schema is {schema}");

        let error_limit = 2;

        let size = 220;

        let my_data_vec: Vec<_> = (0..size).map(|x| Person{ name: format!("name-{:08}", x), age: x, }).collect();
        let my_data_string = serde_json::to_string_pretty(&my_data_vec).expect("to string");
        let my_data_bytes = my_data_string.as_bytes();
        let validated = validate(&schema, my_data_bytes, error_limit).expect("validated");

        let validated_typed: Vec<Person> = serde_json::from_value(validated).expect("conversion to Explicit Type");
        assert!(validated_typed.len() == usize::try_from(size).unwrap());

        println!("There are {} records", validated_typed.len());
    }
}
