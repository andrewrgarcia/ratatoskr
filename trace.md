# Trace

## Execution Trace v0

A **Trace** is the authoritative record of a single Ratatoskr task execution.

A trace freezes **what was executed**, **with which inputs**, **under which constraints**,
and **what changed as a result**. If an execution has no trace, it did not occur.

Traces are immutable once written.

## Purpose

The trace exists to guarantee:

- Accountability — every output has an origin
- Inspectability — every influence is visible
- Reproducibility — executions can be replayed
- Blame assignment — failures are attributable
- Temporal stability — meaning does not drift over time

The trace is not a log.  
It is the execution itself, rendered durable.

## Trace Structure

Each task execution produces a dedicated trace directory:

```

trace/<trace_id>/
├── input.yaml
├── resolved_context/
│   ├── file_1.md
│   └── file_2.md
├── prompt.txt
├── response.txt
├── memory_delta.yaml
├── engine.yaml
└── metadata.yaml

```

All files are mandatory unless explicitly stated otherwise.

## Trace Components

### `input.yaml`

The exact task specification as provided by the user.
No normalization, inference, or substitution is permitted.

### `resolved_context/`

A snapshot of every context artifact used during execution,
copied verbatim at execution time.

Context is never referenced indirectly.

### `prompt.txt`

The fully assembled prompt sent to the inference engine,
after memory and context injection.

This is the final string seen by the model.

### `response.txt`

The raw output returned by the inference engine.
No post-processing is applied unless explicitly recorded.

### `memory_delta.yaml`

Append-only changes produced by the task.
Memory is never rewritten, summarized, or compacted implicitly.

If no memory changes occur, this file is present and empty.

### `engine.yaml`

The declared inference engine and configuration used for execution,
including model name and relevant parameters.

### `metadata.yaml`

Execution metadata, including:

- `trace_id`
- timestamp
- Ratatoskr version
- execution status (success / failure)
- error information, if applicable

## Immutability Rules

- Trace contents MUST NOT be modified after creation
- Re-execution produces a new trace
- Manual edits invalidate the trace
- Derived artifacts must reference their source trace

## Replay Semantics

A trace is **replayable** if:

- the same inference engine is available
- the same context artifacts are present
- the same prompt assembly rules apply

Exact output equivalence is not guaranteed.
Input equivalence is guaranteed.

## Enforcement

Ratatoskr enforces the following:

- No task executes without creating a trace
- No output exists outside a trace
- No memory mutation occurs outside a trace
- No context influences execution unless captured in the trace

If execution cannot be traced, execution is aborted.

## Principle

A trace answers the question:

> “What exactly happened here?”

Without interpretation.
Without reconstruction.
Without trust.

If it is not in the trace, it did not happen.

