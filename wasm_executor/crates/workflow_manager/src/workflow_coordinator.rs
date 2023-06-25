use crate::workflow_json_parser::{Workflow, WorkflowNode};
use std::fs::File;
use std::io::Read;
use wasm_executor::execute_binary_with_parameters;
pub async fn run_workflow(workflow: Workflow) {
    // var for holding the workflow node result
    let mut output_from_previous_call = 1;

    // var for holding the default 2nd input
    // let default_input_2 = 1;

    // start looping through the nodes
    for workflow_node in workflow.workflow_definition {
        // TODO: if we could not find a value and the function expects one,
        //       break. but for this to make sense we need to know the signature
        //       of the node we are calling, so placeholder for now
        let fixed_value_for_node = workflow.workflow_data.get(&workflow_node.id).unwrap_or(&1);

        let wasm_as_byte_array =
            try_read_file_to_byte_array(&workflow_node.wasm_file_path).unwrap();

        let computed_value_result = execute_binary_with_parameters(
            wasm_as_byte_array,
            vec![output_from_previous_call, *fixed_value_for_node],
        )
        .await;
        if let Ok(computed_value) = computed_value_result {
            print_execution_result(
                &workflow_node,
                vec![output_from_previous_call, *fixed_value_for_node],
                computed_value,
            );
            output_from_previous_call = computed_value;
        }
    }
}

fn try_read_file_to_byte_array(absolute_path: &str) -> Result<Vec<u8>, String> {
    let mut file = File::open(absolute_path).map_err(|_| "could not read WASM file".to_string())?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|_| "could not read WASM file".to_string())?;
    Ok(data)
}

fn print_execution_result(workflow_node: &WorkflowNode, input_values: Vec<i32>, output: i32) {
    let workflow_node_name = &workflow_node.name;
    let workflow_node_description = &workflow_node.description;
    let inputs = &workflow_node.inputs;
    println!("called:");
    println!("name: {workflow_node_name}");
    println!("description: {workflow_node_description}");
    println!("with inputs (types): {inputs:?}");
    println!("with inputs (values): {input_values:?}");
    println!("got output: {output:?}");
    println!("##########");
}
