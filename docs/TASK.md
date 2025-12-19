# TASK INVOCATION — SPECIFICATION

A **Task** is the only legal entry point for execution in RATATOSKR.

Nothing executes outside a task.  
Nothing persists without one.

---

## Definition

A task defines:

- intent
- allowed material
- memory scope
- engine configuration
- execution constraints

A task produces exactly one trace.

---

## Task Types

Examples:
- chat
- summarize
- query
- code

Task types are **descriptive only**.  
They do not alter enforcement semantics.

---

## Task Schema

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

All fields are explicit.
No defaults are inferred.

---

## Execution Semantics

Execution proceeds as follows:

1. Load task specification
2. Resolve declared memory references
3. Load declared context artifacts verbatim
4. Assemble deterministic prompt
5. Invoke engine
6. Persist outputs and memory deltas

Failure at any step aborts execution.

---

## Outputs

Each task produces:

* response artifact
* resolved context snapshot
* memory delta
* trace metadata

Nothing is discarded implicitly.

---

## Guarantees

* no hidden state
* no implicit retrieval
* no silent memory mutation
* no background execution
* no execution without a trace

If it is not recorded, it did not occur.
