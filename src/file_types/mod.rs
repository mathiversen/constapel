pub mod css;
pub mod js;
pub mod scss;

use crate::file_creator::Constants;
use serde_yaml::Value;

const STR_DONT_EDIT: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

fn get_reference_value(reference: &str, all_constants: &Constants) -> Option<Value> {
    let mut keys: Vec<&str> = reference.split('.').collect();
    keys[0] = keys[0].trim_start_matches('*');

    if let Some(category) = all_constants.get(keys[0]) {
        if let Some(value) = category.get(&Value::String(keys[1].to_string())) {
            Some(value.clone())
        } else {
            None
        }
    } else {
        None
    }
}
