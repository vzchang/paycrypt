//! Step traces for the explainer; the last step always equals the one-shot result.

use alloc::string::String;
use alloc::vec::Vec;

/// One frame of a derivation: a labelled snapshot of an intermediate value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// Short label for the frame (e.g. `"generate @ bit 20"`).
    pub label: String,
    /// The intermediate byte value at this step (e.g. the current key).
    pub bytes: Vec<u8>,
    /// Human-readable explanation of what happened at this step.
    pub note: String,
}

impl Step {
    /// Construct a step from owned/borrowed parts.
    pub fn new(label: impl Into<String>, bytes: &[u8], note: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            bytes: bytes.to_vec(),
            note: note.into(),
        }
    }
}
