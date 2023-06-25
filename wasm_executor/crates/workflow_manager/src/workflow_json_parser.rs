use std::collections::HashMap;

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

pub type WorkflowNodes = Vec<WorkflowNode>;

pub struct Workflow {
    pub workflow_definition: WorkflowNodes,

    /// this is just a placeholder for providing input data
    /// it maps node id to fixed value that is being passed into workflow nodes
    pub workflow_data: HashMap<String, i32>,
}

impl Workflow {
    /// tries to read a JSON and based on that create a workflow
    pub fn try_create_workflow_from_json(workflow_json: String) -> Result<WorkflowNodes, String> {
        // let nodes_as_json_string = Self::fs_file_to_string(workflow_json_absolute_path)?;
        // dbg!(&fs_file_to_string);
        let workflow_nodes =
            Self::json_string_to_vector_of_nodes(workflow_json).map_err(|error| {
                format!("got serde_json::Error when trying to parse the string {error:?}😬")
            })?;

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
}
