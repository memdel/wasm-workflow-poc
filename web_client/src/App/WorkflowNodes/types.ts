export type WorkflowNode = {
  index: number;
  title: string;
  file_name: string;
  wasm_binary: number[];
  inputs: number[];
  output?: number;
};

export type WorkflowNodes = WorkflowNode[];

export type WorkflowNodeResponse = {
  index: number;
  output: number;
};
