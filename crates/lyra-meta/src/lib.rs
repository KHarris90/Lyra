pub mod diagnostic;
pub mod span;
pub mod version;

// Re-exports (ergonomic for downstream crates)
pub use diagnostic::{Diagnostic, Severity};
pub use span::Span;

pub mod prelude {
    pub use crate::diagnostic::*;
    pub use crate::span::*;
    pub use crate::version::*;
}
