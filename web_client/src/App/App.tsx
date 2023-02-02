import { useState } from "react";
import "./App.css";
import { WorkflowNodes } from "./WorkflowNodes";
import {
  WorkflowNodes as WorkflowNodesType,
  WorkflowNode,
} from "./WorkflowNodes/types";

const workflowNodesInBrowser: WorkflowNodesType = [
  {
    index: 0,
    title: "NA",
    file_name: "NA",
    wasm_binary: [],
    inputs: [],
  },
  {
    index: 1,
    title: "NA",
    file_name: "NA",
    wasm_binary: [],
    inputs: [],
  },
  {
    index: 2,
    title: "NA",
    file_name: "NA",
    wasm_binary: [],
    inputs: [],
  },
  {
    index: 3,
    title: "NA",
    file_name: "NA",
    wasm_binary: [],
    inputs: [],
  },
];

export function App() {
  const [nodes, setNodes] = useState(workflowNodesInBrowser);

  const setWorkflowNode = (workflowNode: WorkflowNode) => {
    const previousNodesState = [...nodes];
    previousNodesState[workflowNode.index] = workflowNode;
    setNodes([...previousNodesState]);
  };

  const uploadNodes = () => {
    const asyncBlock = async () => {
      const body = JSON.stringify({
        workflow_nodes: nodes,
      });
      console.log(nodes[0].wasm_binary.entries());
      console.log(body);
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
      const flowStepsResults = await flowStepsResponse.json();
      console.log(flowStepsResults, "flowStepsResults");
    };
    asyncBlock();
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1 style={{ color: "white" }}>WASM workflow executor</h1>
        <WorkflowNodes
          workflowNodes={nodes}
          setWorkflowNode={setWorkflowNode}
        />
        <button
          onClick={() => {
            uploadNodes();
          }}
        >
          Run Nodes
        </button>
      </header>
    </div>
  );
}
