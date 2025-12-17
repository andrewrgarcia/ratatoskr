# Ratatoskr

Ratatoskr is a **local-first execution enforcer for language model workflows**.

It does not train models, host models, retrieve knowledge, or attempt to be intelligent.  
Instead, Ratatoskr **refuses to execute** unless all inputs, context, memory, and outputs
are made explicit, recorded, and persisted.

Language models are treated as **replaceable engines**.
Memory and context live **outside the model**.
Every run produces **durable artifacts** that can be inspected, replayed, and audited.

Ratatoskr is RAG-adjacent by nature: it may assemble external text to condition generation.
However, its purpose is not to improve answers, but to **enforce execution semantics**.

If something influenced an output, Ratatoskr requires it to be written down.
If something changed, Ratatoskr records it.
If it is not visible on disk, it did not happen.

Ratatoskr is infrastructure, not an assistant.
