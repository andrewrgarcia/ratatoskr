<p align="center">
  <img width="200" alt="rat logo" src="https://github.com/user-attachments/assets/9ab21bcd-2af3-4f56-b93d-f62625daedc9" />
</p>

<h1 align="center">RATATOSKR</h1>

<p align="center">
  <a href="#"><img src="https://img.shields.io/badge/status-v0.3.0--contracted-blue" /></a>
  <a href="#"><img src="https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-blue" /></a>
  <a href="#"><img src="https://img.shields.io/badge/execution-local--first-green" /></a>
  <a href="#"><img src="https://img.shields.io/badge/license-Apache--2.0-lightgrey" /></a>
</p>

<p align="center">
  <strong>Local-first execution layer for language models with enforced memory explicitness and verifiable provenance.</strong>
</p>

---

## Overview

RATATOSKR executes language models **locally** as pure, replaceable functions.

Everything else—tasks, documents, conversations, prompts, outputs—exists as **explicit, frozen material on disk**.
Nothing is inferred. Nothing is hidden. Nothing persists outside the trace.

Each execution produces a complete, inspectable record:
what information was available, how it was assembled, what the model generated,
and which material atoms were cited.

If an output cannot be grounded in declared material, execution fails.

---

## Execution Model

1. **Intent frozen**  
   Task specification (`task.yaml`) defines question, scope, engine, and memory references.

2. **Memory explicit**  
   Conversations and documents resolve to immutable material atoms
   (e.g. FUR messages, document excerpts).

3. **Context assembled deterministically**  
   Prompts are constructed in a fully specified, reproducible order.

4. **Model executes**  
   The language model is treated as an opaque executor: input → output.

5. **Provenance enforced**  
   Outputs must cite material atoms. Invalid or missing citations fail execution.

6. **Trace finalized**  
   Inputs, outputs, validation results, and metadata are persisted as durable artifacts.

No hidden state.  
If it is not on disk, it did not happen.

---

## Core Principle

Every answer must prove where it came from.  
Enforced mechanically, not by convention.

---

## Local-First Execution

RATATOSKR is built for **offline and air-gapped environments**.

- Models are downloaded once and run locally
- No network access is required for execution
- No telemetry, background calls, or hidden dependencies
- Engines are invoked explicitly by path

Local-first execution is not an optimization or deployment choice.
It is a **hard architectural constraint**.

---

## Local Inference Setup (llama.cpp + Hugging Face)

RATATOSKR requires a **local inference engine** and a **local model file**.
It does not download, install, or manage either.

The reference local-first setup uses:

- **llama.cpp** — inference engine
- **GGUF model files** — downloaded from Hugging Face

---

### 1. Install llama.cpp (Linux)

Download a prebuilt llama.cpp release:

https://github.com/ggerganov/llama.cpp/releases

Extract the archive and move it to a stable location:

```bash
mkdir -p ~/engines
mv ~/Downloads/llama-b7684 ~/engines/llama.cpp
```

---

### 2. Create a local wrapper (required)

llama.cpp ships shared libraries.
To avoid system-wide installation, create a local wrapper.

```bash
cd ~/engines/llama.cpp
nano llama
```

Paste:

```bash
#!/usr/bin/env bash
DIR="$(cd "$(dirname "$0")" && pwd)"
export LD_LIBRARY_PATH="$DIR"
exec "$DIR/llama-cli" "$@"
```

Make it executable:

```bash
chmod +x llama
```

Verify:

```bash
~/engines/llama.cpp/llama --help
```

---

### 3. Download a GGUF model (Hugging Face)

Models are **not included**.
Download one GGUF file explicitly.

Recommended (balanced quality, instruction-tuned):

```bash
mkdir -p ~/models
cd ~/models

wget https://huggingface.co/TheBloke/CapybaraHermes-2.5-Mistral-7B-GGUF/resolve/main/capybarahermes-2.5-mistral-7b.Q4_K_M.gguf
```

Verify:

```bash
ls -lh ~/models
```

---

### 4. Test the engine directly

```bash
~/engines/llama.cpp/llama \
  -m ~/models/capybarahermes-2.5-mistral-7b.Q4_K_M.gguf \
  -p "Test output [FUR:test]." \
  --n-predict 128
```

If text prints, local inference is working.

---

### 5. Configure RATATOSKR

Reference the engine and model explicitly in `task.yaml`:

```yaml
engine:
  type: llama.cpp
  name: /home/youruser/engines/llama.cpp/llama
  model: /home/youruser/models/capybarahermes-2.5-mistral-7b.Q4_K_M.gguf
```

RATATOSKR treats the engine as an opaque executor.
All execution must be reproducible from disk.


## Integration

RATATOSKR operates in concert with:

- **FUR** — durable conversation and memory logging  
- **Yggdrasil-CLI** — project and codebase flattening  
- **Inference engines** — external executors invoked explicitly

RATATOSKR defines execution truth.  
Other systems produce or consume its traces.

---

## Output

Each run creates a trace directory containing:

- frozen task specification
- resolved memory and context artifacts
- assembled prompt
- engine declaration
- raw model output
- material usage ledger
- execution metadata and status

Traces are append-only, inspectable, and replayable.

---

## Direction

RATATOSKR is evolving toward a complete local-first execution stack
where humans curate knowledge explicitly, models remain frozen and replaceable,
and reasoning is reproducible by construction.

Future work extends execution patterns and supported engines
without weakening the core contract.

---

<p align="center">
  <strong>The execution record your model cannot escape.</strong>
</p>
