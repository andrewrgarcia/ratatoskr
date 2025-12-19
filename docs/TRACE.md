# EXECUTION TRACE — SPECIFICATION

A **Trace** is the authoritative, immutable record of a single RATATOSKR task execution.

If an execution has no trace, it did not occur.

---

## Purpose

A trace guarantees:

- accountability
- inspectability
- reproducibility
- failure attribution
- temporal stability

A trace is **not a log**.  
It **is the execution**, rendered durable.

---

## Trace Structure

Each task execution produces:

```

trace/<trace_id>/
├── input.yaml
├── resolved_context/
│   ├── documents/
│   └── chunks/
├── prompt.txt
├── response.txt
├── memory_delta.yaml
├── engine.yaml
└── metadata.yaml

```

All files are mandatory unless explicitly stated otherwise.

---

## Component Definitions

### input.yaml
Exact task specification as provided.
No inference, normalization, or substitution.

### resolved_context/
Verbatim copies of all context artifacts used.
Indirect references are forbidden.

### prompt.txt
The final prompt string sent to the inference engine.
This is the exact model input.

### response.txt
Raw engine output.
No post-processing unless explicitly recorded.

### memory_delta.yaml
Append-only memory effects.
If none occur, the file exists and is empty.

### engine.yaml
Engine identity and configuration.
Models are treated as replaceable executors.

### metadata.yaml
Trace metadata:
- trace_id
- timestamp
- RATATOSKR version
- execution status
- error information (if any)

---

## Immutability Rules

- Trace contents MUST NOT be modified
- Re-execution creates a new trace
- Manual edits invalidate provenance
- Derived artifacts must reference their source trace

---

## Replay Semantics

A trace is replayable if:

- the same engine is available
- the same materials exist
- the same assembly rules apply

Output equivalence is NOT guaranteed.  
Input equivalence IS guaranteed.

---

## Enforcement

RATATOSKR enforces:

- no execution without a trace
- no output outside a trace
- no memory mutation outside a trace
- no hidden context

If execution cannot be traced, execution is aborted.

---

## Principle

A trace answers:

> “What exactly happened here?”

Without reconstruction.  
Without interpretation.  
Without trust.

If it is not in the trace, it did not happen.