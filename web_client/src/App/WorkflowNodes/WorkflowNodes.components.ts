import styled from "styled-components";

export const WorkflowNodesContainer = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  width: 100%;
`;
export const NodeContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;

  width: 80%;
  max-width: 512px;

  background-color: white;
  border-radius: 8px;
  margin: 0 0 32px;
  padding: 16px 0 16px;
`;

export const Title = styled.h3``;
export const Filename = styled.span``;

export const FileSelectionInput = styled.input`
  color: transparent;
  max-width: 85px;
  margin: 0 0 16px;
`;
