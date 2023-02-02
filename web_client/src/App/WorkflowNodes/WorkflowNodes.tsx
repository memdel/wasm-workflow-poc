import {
  WorkflowNodesContainer,
  NodeContainer,
  Title,
  Filename,
  FileSelectionInput,
} from "./WorkflowNodes.components";
import { WorkflowNodes as WorkflowNodesType, WorkflowNode } from "./types";

type Props = {
  workflowNodes: WorkflowNodesType;
  setWorkflowNode: (workflowNode: WorkflowNode) => void;
};

export function WorkflowNodes(props: Props) {
  const { workflowNodes, setWorkflowNode } = props;

  const fileToByteArray = async (nodeIndex: number, file: File | null) => {
    const arrayBuffer = await file?.arrayBuffer();

    if (file && arrayBuffer) {
      const fileAsByteArray = new Uint8Array(arrayBuffer);
      const updatedWorkflowNode: WorkflowNode = {
        index: nodeIndex,
        file_name: file.name,
        title: file.name,
        wasm_binary: Array.from(fileAsByteArray),
        inputs: [],
      };

      setWorkflowNode(updatedWorkflowNode);
    }
  };

  const renderedWorkflowNodes = workflowNodes.map((workflowNode) => {
    const { index, file_name, title } = workflowNode;
    return (
      <NodeContainer key={index}>
        <Title>{title}</Title>
        <FileSelectionInput
          type="file"
          onChange={(event) => {
            fileToByteArray(index, event.target.files && event.target.files[0]);
          }}
        ></FileSelectionInput>
      </NodeContainer>
    );
  });

  return (
    <WorkflowNodesContainer>{renderedWorkflowNodes}</WorkflowNodesContainer>
  );
}
