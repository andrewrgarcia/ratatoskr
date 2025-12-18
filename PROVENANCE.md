# RATATOSKR — Provenance Goals and End-State

This document freezes the *end-state intent* of RATATOSKR so development does not drift.

It is not a roadmap. It is a constraint.

---

## 1. What RATATOSKR Ultimately Guarantees

For **every AI-assisted output**, RATATOSKR must be able to answer, using files on disk:

* What task was requested
* Which engine/model/configuration was used
* Which documents were considered
* Which document *sections / chunks* were actually used
* Which prior conversations or memories influenced the result
* What the system produced
* Whether the execution succeeded or failed

If any of these cannot be answered **after the fact**, the execution is considered invalid.

---

## 2. Core Design Principle

> **Provenance is not metadata. Provenance is the product.**

RATATOSKR does not try to make models smarter.
It makes *reasoning auditable*.

Models remain replaceable engines.
Trust lives in artifacts.

---

## 3. Trace as the Unit of Truth

A RATATOSKR trace is the atomic unit of accountability.

A trace must fully describe:

* Intent (`input.yaml`)
* Context resolution (`resolved_context/`)
* Prompt materialization (`prompt.txt`)
* Model output (`response.txt`)
* Memory effects (`memory_delta.yaml`)
* Execution state (`metadata.yaml`)

Nothing outside the trace may be required to explain an answer.

---

## 4. Document Provenance (Future, Locked-In)

When document-assisted inference occurs, the trace **must** contain:

```
resolved_context/
├── documents/
│   └── <original source files>
├── chunks/
│   └── <exact sections used>
└── selection.yaml
```

`selection.yaml` records *why* material was selected, not just *what*.

This enables institutional audit and post-hoc review.

---

## 5. Conversation & Memory Provenance (via FUR)

RATATOSKR does not store long-term memory.

Instead, it records **references** to memory artifacts maintained by FUR.

Example:

```yaml
conversation_context:
  fur_trace_ids:
    - fur/2024-11-03/session-17
    - fur/2024-11-04/session-02
```

This cleanly separates:

* execution (RATATOSKR)
* memory (FUR)

---

## 6. Institutional Readiness

This architecture is designed to satisfy:

* audit requirements
* regulatory review
* reproducibility
* accountability

RATATOSKR is intentionally *boring* so that institutions can trust it.

---

## 7. Non-Goals (Explicit)

RATATOSKR does not:

* claim model correctness
* judge truthfulness
* hide uncertainty
* optimize for UX
* abstract away provenance

If a feature weakens auditability, it is rejected.

---

## 8. Development Constraint

All future development **must preserve** the ability to reconstruct:

> “Why did the system say this, using these materials, at this time?”

If that question cannot be answered with files on disk, the design is wrong.

---

**This document is now part of the RATATOSKR contract.**

