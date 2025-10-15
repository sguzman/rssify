/*
File: crates/repos/fs/src/tx.rs
Purpose: Trivial transaction/context handle for the FS adapter.
Inputs: rssify_core::Tx trait.
Outputs: FsTx implementing Tx.
Side effects: None.
*/

use rssify_core::Tx as TxTrait;

#[derive(Debug, Clone)]
pub struct FsTx {
    pub(crate) active: bool,
}

impl TxTrait for FsTx {
    fn is_active(&self) -> bool {
        self.active
    }
}

