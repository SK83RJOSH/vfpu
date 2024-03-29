pub use anyhow::anyhow;
pub use anyhow::Context;

pub type Error = anyhow::Error;
pub type Result<T, E = Error> = anyhow::Result<T, E>;
