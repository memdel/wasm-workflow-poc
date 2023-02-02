use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use wasm_executor::execute_binary_with_parameters;

/// a single node that is defined in the client, representing one step in a the workflow
/// is a single WASM file
#[derive(Serialize, Deserialize)]
pub struct WorkflowNode {
    pub wasm_binary: Vec<u8>,
    pub title: String,
    pub index: i32,
    pub file_name: String,
}

/// the input containing all the nodes that belong to a single workflow
#[derive(Serialize, Deserialize)]
pub struct WorkflowNodes {
    pub workflow_nodes: Vec<WorkflowNode>,
}

/// flow executor, takes the binary and passed it further to be executed
/// TODO: take params from the request
/// TODO: loop through all the nodes in the flow
/// TODO: coordinate the passing of the data and other possible control flow between nodes
pub async fn execute_flow(post: web::Json<WorkflowNodes>) -> impl Responder {
    let wasm_binary = post.workflow_nodes[0].wasm_binary.clone();
    let parameters: Vec<i32> = vec![1, 2];
    let output_result = execute_binary_with_parameters(wasm_binary, parameters).await;
    let output = output_result.unwrap();
    HttpResponse::Ok().json(output)
}
