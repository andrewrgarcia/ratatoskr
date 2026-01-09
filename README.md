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
  <strong>Execution layer for language models that enforces explicit memory and verifiable provenance.</strong>
</p>

---

## Overview

RATATOSKR runs language models as pure executors. Everything else—tasks, documents, conversations, outputs—exists as frozen, inspectable material on disk.

Each execution produces a complete record: what information was available, how it was assembled, what the model generated, which materials were cited. If output can't be grounded in material, execution fails.

---

## Execution Model

1. **Intent frozen**  
   Task spec (`task.yaml`) defines question, scope, engine, memory references.

2. **Memory explicit**  
   Conversations and documents resolve to immutable atoms (FUR messages, document excerpts).

3. **Context assembled deterministically**  
   Prompts constructed in fully specified order.

4. **Model executes**  
   Language model treated as opaque function: input → output.

5. **Provenance enforced**  
   Outputs cite material atoms. Invalid citations fail execution.

6. **Trace finalized**  
   Inputs, outputs, validation persisted as durable artifacts.

No hidden state. If it's not on disk, it didn't happen.

---

## Core Principle

Every answer proves where it came from. Enforced mechanically, not by convention.

---

## Local-First

Built for offline and air-gapped environments.

* Models downloaded once, run locally
* No network required for execution
* No telemetry, external dependencies, background calls

Suitable for research, policy analysis, regulated environments where trust and reproducibility matter.

---

## Integration

Works with:

* **FUR** — durable conversation and memory logging  
* **Yggdrasil-CLI** — project and codebase flattening  
* **Inference engines** — local or remote, interchangeable

RATATOSKR defines execution truth. Other systems produce or consume its traces.

---

## Output

Each run creates trace directory with:

* frozen task specification
* resolved memory and context artifacts
* assembled prompt
* engine declaration
* raw model output
* material usage ledger
* execution metadata and status

Traces are append-only, inspectable, replayable.

---

## Direction

Moving toward complete local-first execution stack where humans curate knowledge explicitly, models stay frozen and replaceable, reasoning is reproducible, accountability non-optional.

Future work expands execution patterns and engines without weakening the contract.

---

## Status

* **v0.3.0** — provenance contract enforced
* Message-level material atoms
* Mandatory citation validation
* Deterministic prompt assembly
* Minimal, explicit engine interface

---

<p align="center">
  <strong>The execution record your model cannot escape.</strong>
</p>
