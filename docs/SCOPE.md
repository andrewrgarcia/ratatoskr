# Scope

This document defines what Ratatoskr **is**, and more importantly, what it **refuses to be**.
Violating this scope means you are no longer using Ratatoskr.

## In Scope (v0)

Ratatoskr **does**:

- Run locally and offline
- Execute a single, explicit task at a time
- Assemble prompts from user-declared inputs, memory, and context artifacts
- Treat all context as explicit and user-owned
- Persist all inputs, resolved context, outputs, and memory deltas to disk
- Treat inference engines as interchangeable, non-authoritative components
- Favor determinism, traceability, and auditability over convenience

## Out of Scope (v0)

Ratatoskr **does not**:

- Train, fine-tune, or modify models
- Host models or provide inference as a service
- Perform implicit retrieval or ranking
- Infer context without being told
- Mutate memory silently or automatically
- Hide execution steps behind abstractions
- Require cloud services, accounts, or subscriptions
- Implement decentralization, federation, incentives, or marketplaces
- Optimize for scale, growth, monetization, or user engagement

## Semantic Guarantees

Ratatoskr guarantees that:

- Every execution produces a trace
- No execution occurs without persisted artifacts
- Memory is append-only and scoped
- Context is explicit and enumerable
- Engine selection is declared, not inferred

Any feature that weakens these guarantees requires a new scope definition.
