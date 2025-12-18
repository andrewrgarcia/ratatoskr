use crate::fur_atom::FurAtom;
use std::collections::HashSet;

pub fn validate_citations(
    citations: &[String],
    atoms: &[FurAtom],
) -> Result<(), String> {
    if citations.is_empty() {
        return Err("model produced no material citations".into());
    }

    let allowed: HashSet<&String> =
        atoms.iter().map(|a| &a.message_id).collect();

    for c in citations {
        if !allowed.contains(c) {
            return Err(format!(
                "invalid citation: FUR:{} (not in material ledger)",
                c
            ));
        }
    }

    Ok(())
}
