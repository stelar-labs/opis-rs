use crate::Integer;
use std::fmt::Debug;

impl Debug for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Integer")
            .field("value", &self.to_dec())
            .field("hex", &self.to_hex())
            .field("bin", &self.to_bin())
            .finish()
    }
}
