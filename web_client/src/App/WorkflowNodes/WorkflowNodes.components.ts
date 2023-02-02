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

  width: 80%;
  max-width: 512px;

  background-color: white;
  border-radius: 8px;
  margin: 0 0 32px;
  padding: 16px 0;
`;

export const Column = styled.div`
  display: flex;
  flex-direction: column;

  width: 33%;

  padding: 0 16px;
  border-right: solid 1px black;
  &:last-child {
    border-right: none;
  }
`;

export const Title = styled.h3`
  margin: 0 0 8px;
`;

export const Filename = styled.h5`
  margin: 0 0 8px;
`;

export const Output = styled.h5`
  margin: 0 0 8px;
  color: blue;
`;

export const FileSelectionInput = styled.input`
  color: transparent;
  max-width: 85px;
  margin: 0 0 16px;
`;
