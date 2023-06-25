use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use wasm_executor::execute_binary_with_parameters;

/// a single node that is defined in the client, representing one step in a the workflow
/// is a single WASM file
#[derive(Serialize, Deserialize, Clone)]
pub struct WorkflowNode {
    pub wasm_binary: Vec<u8>,
    pub title: String,
    pub index: i32,
    pub file_name: String,
    pub inputs: Vec<i32>,
}

/// the input containing all the nodes that belong to a single workflow
#[derive(Serialize, Deserialize, Clone)]
pub struct WorkflowNodes {
    pub workflow_nodes: Vec<WorkflowNode>,
}

#[derive(Serialize, Deserialize)]
struct NodeResult {
    index: i32,
    output: i32,
}

#[derive(Serialize, Deserialize)]
struct WorkflowExecutionResponse {
    node_results: Vec<NodeResult>,
}

/// flow executor, takes the binary and passed it further to be executed
/// TODO: coordinate the passing of the data and other possible control flow between nodes
pub async fn execute_flow(post: web::Json<WorkflowNodes>) -> impl Responder {
    // look through the node and execute them
    let mut node_results: Vec<NodeResult> = vec![];
    let nodes = post.workflow_nodes.clone();
    for (_index, node) in nodes.iter().enumerate() {
        // inputs
        let wasm_binary = node.wasm_binary.clone();
        let inputs = node.inputs.clone();
        let index = node.index;

        // if the byte array has content we naively expect it to be WASM
        if !wasm_binary.is_empty() {
            // execute WASM
            let output_result = execute_binary_with_parameters(wasm_binary, inputs).await;

            // outputs
            let output = output_result.unwrap();
            let node_result = NodeResult { index, output };
            node_results.push(node_result);
        }
    }

    let execute_flow_response = WorkflowExecutionResponse { node_results };
    HttpResponse::Ok().json(execute_flow_response)
}
