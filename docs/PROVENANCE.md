# RATATOSKR — PROVENANCE CONTRACT

This document freezes the **non-negotiable guarantees** of RATATOSKR.

It is not a roadmap.  
It is not descriptive.  
It is a **contract**.

Any implementation that violates this document is not RATATOSKR.

---

## 1. What RATATOSKR Guarantees

For **every AI-assisted output**, RATATOSKR guarantees that the following questions can be answered **using files on disk**:

- What task was requested
- Which engine / model / configuration was used
- Which documents were permitted as context
- Which document sections or chunks were actually used
- Which prior conversations or memory artifacts were referenced
- What output was produced
- Whether execution succeeded or failed

If any of these cannot be reconstructed **after execution**, the run is invalid.

---

## 2. Core Principle

> **Provenance is not metadata. Provenance is the product.**

RATATOSKR does not attempt to improve model quality.
It does not interpret model internals.
It does not explain attention, weights, or reasoning.

It enforces **material accountability**.

Models are replaceable engines.  
Trust lives in artifacts.

---

## 3. Trace Is the Unit of Truth

A **trace** is the atomic unit of execution.

A trace fully defines:

- intent
- allowed material
- prompt construction
- engine invocation
- outputs
- memory effects
- execution status

Nothing outside the trace is required to explain an answer.

If an output exists without a trace, it did not happen.

---

## 4. Document Provenance (Guaranteed Interface)

When document-assisted execution occurs, the trace MUST contain:

```

resolved_context/
├── documents/
│   └── original source files (verbatim copies)
├── chunks/
│   └── exact excerpts used
└── selection.yaml

```

`selection.yaml` records **why** material was selected, not only **what**.

This enables audit, review, and institutional accountability.

---

## 5. Conversation & Memory Provenance (via FUR)

RATATOSKR does not store long-term conversational memory.

Instead, it records **explicit references** to memory artifacts maintained by FUR.

Example:

```yaml
memory_refs:
  - system: fur
    convo_id: 2024-11-03-session-17
```

Separation is intentional:

* RATATOSKR → execution truth
* FUR → memory truth

---

## 6. Institutional Readiness

This architecture is designed for:

* audits
* regulatory review
* reproducibility
* blame assignment
* long-term stability

RATATOSKR favors explicitness over convenience.

---

## 7. Explicit Non-Goals

RATATOSKR does NOT:

* claim correctness or truth
* infer model reasoning
* hide uncertainty
* optimize UX
* perform silent retrieval
* mutate memory implicitly

If a feature weakens auditability, it is rejected.

---

## 8. Multi-Step and External Execution

A trace may represent:

* a single execution
* a sequence of constrained executions
* staged or chained inference
* reuse of derived outputs as new material

In all cases:

* every material is typed and persisted
* derived materials are distinguished from primary sources
* execution order is reconstructible from disk

Provenance applies to the **entire chain**, not individual steps.

---

## 9. Final Constraint

All future development MUST preserve the ability to answer:

> “Why did the system say this, using these materials, at this time?”

If that answer is not fully reconstructible from files on disk, the design is invalid.
