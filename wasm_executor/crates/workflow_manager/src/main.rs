mod workflow_json_parser;

use std::fs::File;
use std::io::Read;

use wasm_executor::execute_binary_with_parameters;
use workflow_json_parser::WorkflowNode;

#[tokio::main]
async fn main() {
    let parsed_workflow_maybe = WorkflowNode::try_from_json_absulte_path("/Users/i587107/Documents/development_2/08_wasm-workflow-poc/wasm-workflow-poc/wasm_executor/crates/workflow_manager/src/exampleWorkflowNodes.json");

    if let Ok(parsed_workflow) = parsed_workflow_maybe {
        print_start_executing_banner();

        print_found_workflow_banner(&parsed_workflow);

        let mut output_from_previous_call = 1;
        let default_input_2 = 1;

        println!();
        println!("##########");
        for workflow_node in parsed_workflow {
            let wasm_as_byte_array =
                try_read_file_to_byte_array(&workflow_node.wasm_file_path).unwrap();

            let computed_value_result = execute_binary_with_parameters(
                wasm_as_byte_array,
                vec![output_from_previous_call, default_input_2],
            )
            .await;
            if let Ok(computed_value) = computed_value_result {
                print_execution_result(
                    &workflow_node,
                    vec![output_from_previous_call, default_input_2],
                    computed_value,
                );
                output_from_previous_call = computed_value;
            }
        }
    };
}

fn print_start_executing_banner() {
    println!();
    println!("##########");
    println!("Executing workflow");
    println!("##########");
    println!();
}

fn print_found_workflow_banner(workflow: &Vec<WorkflowNode>) {
    println!();
    println!("##########");
    println!("found the below workflow 👇");
    println!("{:?}", workflow);
    println!("##########");
    println!();
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

fn try_read_file_to_byte_array(absolute_path: &str) -> Result<Vec<u8>, String> {
    let mut file = File::open(absolute_path).map_err(|_| "could not read WASM file".to_string())?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|_| "could not read WASM file".to_string())?;
    Ok(data)
}
