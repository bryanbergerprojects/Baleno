//! Types shared by every bounded context: typed identifiers and log-safe wrappers.

mod ids;
mod redacted;

pub use ids::{ServerId, StackId, UserId};
pub use redacted::Redacted;
