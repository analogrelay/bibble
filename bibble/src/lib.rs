#[macro_use]
mod macros;

mod collection;
mod document;
mod field;

pub use collection::Collection;
pub use document::{Document, DocumentId};
pub use field::{Field, FieldValue};