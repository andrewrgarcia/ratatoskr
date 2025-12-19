<p align="center">
  <img src="https://github.com/user-attachments/assets/fea23bc0-30c5-4946-a054-00dfc29385eb" width="200" alt="ratatoskr logo"/>
</p>

<h1 align="center">RATATOSKR</h1>

<p align="center">
  <a href="#"><img src="https://img.shields.io/badge/status-v0.3.0--contracted-blue" /></a>
  <a href="#"><img src="https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-blue" /></a>
  <a href="#"><img src="https://img.shields.io/badge/execution-local--first-green" /></a>
  <a href="#"><img src="https://img.shields.io/badge/license-Apache--2.0-lightgrey" /></a>
</p>

<p align="center">
  <strong>Deterministic, trace-first execution for language model workflows.</strong><br/>
  If it influenced an output, it must be written down.
</p>

---

## What RATATOSKR Is

RATATOSKR is a **local-first execution enforcer for language-model workflows**.

It does **not** train models.  
It does **not** host models.  
It does **not** retrieve knowledge to “improve answers.”  
It does **not** attempt to be intelligent.

Instead, RATATOSKR **refuses to execute** unless *all* inputs, context, memory references,
engine configuration, and outputs are made **explicit, persisted, and inspectable**.

**If a model produces an output without producing a verifiable material usage record, execution fails.**

Language models are treated as **replaceable engines**.  
Memory and context live **outside the model**.  
Every run produces **durable artifacts** that can be replayed, audited, and explained.

RATATOSKR is infrastructure — not an assistant.

---

## Execution Is Protocol-Driven

RATATOSKR does not assume a single execution pattern.

How context is assembled, ordered, or reused is treated as an **explicit execution protocol**,
not an implicit behavior of the system.

This allows:
- deterministic single-shot execution
- staged or sequential execution
- controlled reuse of prior outputs as derived materials
- reproducible experimentation across different execution strategies

RATATOSKR enforces provenance and validation regardless of protocol.

---
## Core Principle

> **If something influenced an output, RATATOSKR requires it to be written down.**  
> **If something changed, RATATOSKR records it.**  
> **If it is not visible on disk, it did not happen.**

There is no hidden state.  
There is no implicit memory.  
There is no silent retrieval.  
There is no background execution.

---

## RAG, Clarified (No Hand-Waving)

RATATOSKR is **RAG-adjacent by nature**.

It may assemble external text (documents, excerpts, notes) to condition generation.
However, its purpose is **not** to improve answer quality.

Its purpose is to **enforce execution semantics**.

Where most RAG pipelines treat provenance as optional,
RATATOSKR makes provenance **mandatory**.

RATATOSKR does not ask:  
> “What information should we retrieve?”

It asks:  
> “What information *actually* influenced this result — and where is the proof?”

---

## What a Run Produces

Each execution creates a **trace directory** containing:

- the frozen task specification (`input.yaml`)
- the fully assembled prompt (`prompt.txt`)
- resolved context artifacts (documents + extracted chunks)
- explicit memory references
- engine declaration
- raw model output
- a material usage ledger proving which inputs were actually used
- trace metadata and lifecycle state

These artifacts are append-only, durable, and replayable.

---

## Relationship to Other Tools

RATATOSKR is designed to work **in concert**, not competition, with:

- **FUR** — durable AI conversation memory
- **Yggdrasil-CLI** — project and codebase flattening
- downstream engines (local or remote) treated as pure executors

RATATOSKR defines *execution truth*.  
Other systems may consume it.

---

## Project Status

- **v0.3.0** — Provenance contract enforced
- Message-level material atoms
- Mandatory post-inference validation
- Material usage ledger (auditable grounding)
- Engine abstraction intentionally minimal

Future versions will extend capability **without breaking the execution contract**.

---

## Non-Goals

RATATOSKR intentionally does **not**:

- optimize for scale or throughput
- provide SaaS or hosted inference
- perform federated or decentralized compute
- hide complexity behind convenience
- infer intent or “help” beyond what is specified

Any feature that violates these constraints requires a new scope.

---

## License

Licensed under the Apache License, Version 2.0.

---

<p align="center">
  <strong>RATATOSKR is the paper trail your language model cannot escape.</strong>
</p>
