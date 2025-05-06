#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]

mod combinators;
mod committed_status;
mod element;
mod parse_context;
mod parse_error;
mod parse_result;
mod parser;
pub mod util;

pub mod prelude {
  pub use crate::combinators::*;
  pub use crate::committed_status::*;
  pub use crate::element::*;
  pub use crate::parse_context::*;
  pub use crate::parse_error::*;
  pub use crate::parse_result::*;
  pub use crate::parser::*;
}
