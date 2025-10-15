// File: crates/repos/fs/src/tx.rs
// Purpose: No-op transaction type and Tx impl for the FS backend.
// Inputs: rssify_core::Tx trait.
// Outputs: FsTx implementing Tx.
// Side effects: None.

use rssify_core::Tx;

#[derive(Clone, Copy, Debug, Default)]
pub struct FsTx {
    pub(crate) active: bool,
}

impl Tx for FsTx {
    fn is_active(&self) -> bool {
        self.active
    }
}

