use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};

/*
{
    "id": "node_id_1",
    "name": "Node 1",
    "description": "Sum of 2 integers",
    "wasm_file": "/Users/i587107/Documents/development_2/08_wasm-workflow-poc/wasm-workflow-poc/main.wasm",
    "inputs": {
        "parameter_name_1": "i32",
        "parameter_name_2": "i32"
    },
    "output": "i32"
},
 */

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum DataType {
    I32,
    #[default]
    U32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct WorkflowNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub wasm_file_path: String,
    pub inputs: HashMap<String, DataType>,
    pub output: DataType,
}

pub type _WorkflowNodes = Vec<WorkflowNode>;

impl WorkflowNode {
    pub fn try_from_json_absulte_path(
        workflow_json_absolute_path: &str,
    ) -> Result<Vec<WorkflowNode>, String> {
        // try to parse the input string with serde

        let nodes_as_json_string = Self::fs_file_to_string(workflow_json_absolute_path)?;

        let workflow_nodes = Self::json_string_to_vector_of_nodes(nodes_as_json_string)
            .map_err(|_| "got serde_json::Error when trying to parse the string 😬")?;

        Ok(workflow_nodes)
    }

    fn json_string_to_vector_of_nodes(
        unvalidated_string: String,
    ) -> Result<Vec<WorkflowNode>, serde_json::Error> {
        // we try to parse the string and cast it to our type in one go
        let parsed_workflow_nodes: Vec<WorkflowNode> =
            serde_json::from_str(unvalidated_string.as_str())?;
        Ok(parsed_workflow_nodes)
    }

    fn fs_file_to_string(absolute_path: &str) -> Result<String, String> {
        let file_content_as_string =
            fs::read_to_string(absolute_path).map_err(|_| "could not read the file".to_string())?;

        Ok(file_content_as_string)
    }
}
