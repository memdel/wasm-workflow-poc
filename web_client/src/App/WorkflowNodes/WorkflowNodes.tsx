import {
  WorkflowNodesContainer,
  NodeContainer,
  Title,
  Filename,
  Output,
  Column,
  FileSelectionInput,
} from "./WorkflowNodes.components";
import { WorkflowNodes as WorkflowNodesType, WorkflowNode } from "./types";

type Props = {
  workflowNodes: WorkflowNodesType;
  setWorkflowNode: (workflowNode: WorkflowNode) => void;
};

// TODO: make number nodes dynamic
// TODO: make number of params dynamic
// TODO: fix type naming for the nodes and the components, it is not good now
export function WorkflowNodes(props: Props) {
  const { workflowNodes, setWorkflowNode } = props;

  // callback to when input is changed
  const setInputValues = async (nodeIndex: number, inputValues: number[]) => {
    const existingNode = workflowNodes[nodeIndex];
    const updatedWorkflowNode: WorkflowNode = {
      ...existingNode,
      inputs: inputValues,
    };
    setWorkflowNode(updatedWorkflowNode);
  };

  // callback to when file is selected
  const setWasmFile = async (nodeIndex: number, file: File | null) => {
    const arrayBuffer = await file?.arrayBuffer();

    if (file && arrayBuffer) {
      const fileAsByteArray = new Uint8Array(arrayBuffer);
      const existingNode = workflowNodes[nodeIndex];
      const updatedWorkflowNode: WorkflowNode = {
        ...existingNode,
        file_name: file.name,
        title: file.name,
        wasm_binary: Array.from(fileAsByteArray),
      };

      setWorkflowNode(updatedWorkflowNode);
    }
  };

  // render nodes
  const renderedWorkflowNodes = workflowNodes.map((workflowNode) => {
    const { index, title, output, inputs, wasm_binary } = workflowNode;
    const isWasmPresent = wasm_binary.length > 0;
    return (
      <NodeContainer key={index}>
        <Column style={{ opacity: isWasmPresent ? "" : "0.2" }}>
          <Title>Inputs</Title>
          <input
            style={{ margin: "0 0 8px" }}
            value={inputs[0]}
            onChange={(event) => {
              const updatedValue = event.target.value;
              // updated 1st parameter
              setInputValues(index, [Number(updatedValue), inputs[1]]);
            }}
          />
          <input
            value={inputs[1]}
            onChange={(event) => {
              const updatedValue = event.target.value;
              // updated 2nd parameter
              setInputValues(index, [inputs[0], Number(updatedValue)]);
            }}
          />
        </Column>
        <Column>
          <Title>WASM File</Title>
          <Filename>{title}</Filename>
          <FileSelectionInput
            type="file"
            onChange={(event) => {
              setWasmFile(index, event.target.files && event.target.files[0]);
            }}
          ></FileSelectionInput>
        </Column>

        <Column style={{ opacity: isWasmPresent ? "" : "0.2" }}>
          <Title>Output</Title>
          <Output>{output}</Output>
        </Column>
      </NodeContainer>
    );
  });

  // render all nodes
  return (
    <WorkflowNodesContainer>{renderedWorkflowNodes}</WorkflowNodesContainer>
  );
}
