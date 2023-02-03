import { useState } from "react";
import "./App.css";
import { WorkflowNodes } from "./WorkflowNodes";
import {
  WorkflowNodes as WorkflowNodesType,
  WorkflowNode,
} from "./WorkflowNodes/types";

// initial data for the nodes
const workflowNodesInBrowser: WorkflowNodesType = [
  {
    index: 0,
    title: "NA",
    file_name: "NA",
    wasm_binary: [],
    inputs: [1, 1],
  },
  {
    index: 1,
    title: "NA",
    file_name: "NA",
    wasm_binary: [],
    inputs: [1, 1],
  },
  {
    index: 2,
    title: "NA",
    file_name: "NA",
    wasm_binary: [],
    inputs: [1, 1],
  },
  {
    index: 3,
    title: "NA",
    file_name: "NA",
    wasm_binary: [],
    inputs: [1, 1],
  },
];

// TODO: unify the vocabulary (execute vs. run, node vs. step, ...)
export function App() {
  const [nodes, setNodes] = useState(workflowNodesInBrowser);

  // callback that is passed to WorkflowNodes, can update a node
  const setWorkflowNode = (workflowNode: WorkflowNode) => {
    const previousNodesState = [...nodes];
    previousNodesState[workflowNode.index] = workflowNode;
    setNodes([...previousNodesState]);
  };

  // uploads nodes and waits for execution response
  const uploadNodesToBeRun = () => {
    const asyncBlock = async () => {
      const body = JSON.stringify({
        workflow_nodes: nodes,
      });

      const flowStepsResponse = await fetch(
        "http://127.0.0.1:8080/execute-flow",
        {
          method: "PATCH",
          body: body,
          headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
          },
        }
      );
      const flowStepsOutputs = await flowStepsResponse.json();
      const flowStepsOutputsCasted = flowStepsOutputs as {
        node_results: { index: number; output: number }[];
      };

      const previousNodesState = [...nodes];
      flowStepsOutputsCasted.node_results.forEach((nodeResult) => {
        const updatedNode = nodes[nodeResult.index];
        previousNodesState[nodeResult.index] = {
          ...updatedNode,
          output: nodeResult.output,
        };
      });
      setNodes([...previousNodesState]);
    };
    asyncBlock();
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1 style={{ color: "white" }}>🏃‍♀️🏃‍♀️WASM Workflow Runner 🏃‍♀️🏃‍♀️</h1>
        <WorkflowNodes
          workflowNodes={nodes}
          setWorkflowNode={setWorkflowNode}
        />
        <button
          onClick={() => {
            uploadNodesToBeRun();
          }}
        >
          Run Nodes
        </button>
      </header>
    </div>
  );
}
