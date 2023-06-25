mod workflow_coordinator;
mod workflow_json_parser;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::process::exit;

use workflow_coordinator::run_workflow;
use workflow_json_parser::{Workflow, WorkflowNodes};

const HARDCODED_WORKFLOW_JSON: &str = "/Users/memas/development7/07_wasm-workflow-poc/wasm-workflow-poc/wasm_executor/crates/workflow_manager/src/exampleWorkflowNodes.json";

#[tokio::main]
async fn main() {
    // try to read the JSON file or exit
    let workflow_as_json = fs_file_to_string(HARDCODED_WORKFLOW_JSON).unwrap_or_else(|_| {
        println!("failed to read the workflow JSON file");
        exit(1)
    });

    // try to parse the JSON to Workflow
    let workflow_definition = Workflow::try_create_workflow_from_json(workflow_as_json)
        .unwrap_or_else(|_| {
            println!("failed to parse the workflow JSON file to `Workflow`");
            exit(1)
        });

    print_start_executing_banner();
    print_found_workflow_banner(&workflow_definition);

    // TODO: the real input data comes from a JSON file in a similar way as the
    // 		 work flow definition
    let mut workflow_data = HashMap::<String, i32>::new();
    workflow_data.insert("node_id_1".to_string(), 1);
    workflow_data.insert("node_id_2".to_string(), 200);
    workflow_data.insert("node_id_3".to_string(), 3_000);
    workflow_data.insert("node_id_4".to_string(), 40_000);

    // create workflow, we do this as we will add the data for the current
    // workflow to it later
    let workflow = Workflow {
        workflow_definition,
        workflow_data,
    };

    run_workflow(workflow).await;

    // // var for holding the workflow node result
    // let mut output_from_previous_call = 1;

    // // var for holding the default 2nd input
    // let default_input_2 = 1;

    // // start looping through the nodes
    // for workflow_node in parsed_workflow {
    //     let wasm_as_byte_array =
    //         try_read_file_to_byte_array(&workflow_node.wasm_file_path).unwrap();

    //     let computed_value_result = execute_binary_with_parameters(
    //         wasm_as_byte_array,
    //         vec![output_from_previous_call, default_input_2],
    //     )
    //     .await;
    //     if let Ok(computed_value) = computed_value_result {
    //         print_execution_result(
    //             &workflow_node,
    //             vec![output_from_previous_call, default_input_2],
    //             computed_value,
    //         );
    //         output_from_previous_call = computed_value;
    //     }
    // }
}

fn fs_file_to_string(absolute_path: &str) -> Result<String, String> {
    let file_content_as_string =
        read_to_string(absolute_path).map_err(|_| "could not read the file".to_string())?;

    Ok(file_content_as_string)
}

fn print_start_executing_banner() {
    println!();
    println!("##########");
    println!("Executing workflow");
    println!("##########");
    println!();
}

fn print_found_workflow_banner(workflow: &WorkflowNodes) {
    println!();
    println!("##########");
    println!("found the below workflow 👇");
    println!("{:?}", workflow);
    println!("##########");
    println!();
}
