
# Task Invocation

## Ratatoskr Task Invocation v0

A **Task Invocation** is the atomic unit of execution in Ratatoskr.
Nothing happens outside a task. Nothing persists without one.

A task defines **what is executed**, **with what inputs**, and **under which constraints**.
Each task produces a trace that freezes execution semantics in time.

## Task Types

- `chat`
- `code`
- `summarize`
- `query`

Task types are descriptive only. They do not alter enforcement behavior.

## Inputs

```yaml
task_type: chat
prompt: "User prompt text"

memory_scope: global | project | session

context:
  - path: codex/project_x.md
  - path: notes/design.md

engine:
  type: local
  name: ollama
  model: mistral
```

All inputs are mandatory.
No input may be inferred or substituted implicitly.

## Execution Semantics

A task executes as follows:

1. Load task specification
2. Resolve declared memory scope
3. Load declared context artifacts exactly as specified
4. Assemble a deterministic prompt from inputs, memory, and context
5. Invoke the declared inference engine
6. Persist outputs and memory deltas

Failure at any step aborts execution.

## Outputs

Each task produces the following artifacts:

* `response` — raw model output
* `resolved_context` — exact context used
* `memory_delta` — append-only memory changes
* `trace_id` — unique execution identifier

Outputs are written to disk and never discarded implicitly.

## Guarantees

* No hidden state
* No implicit retrieval
* No silent memory mutation
* No background execution
* No execution without a trace

If it is not recorded, it did not occur.
