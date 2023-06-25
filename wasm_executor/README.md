<h1> WASM Executor </h1>

## Vocabulary

* **tasks** - a single executable task. Persisted as a WASM and executed as a WASM
* **flow or workflow** - a whole work, containing one to n tasks
* **running a flow** - means the action of the user starting a flow where the
  whole flow is being run.

## Architecture

* **web_server** - handles the network calls from the clients
* **flow_manager** - manages the persistance and retrieval of workflows
* **wasm_executor** - executes the tasks. Note that it executes the tasks, it is
  not the responsibility of the executor to manage the complete flows. A flow
  can be interrupted mid way and continued if the client so wants. So it is the
  responsibility of the **flow_manager**
