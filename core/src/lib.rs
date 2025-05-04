#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
pub mod core;
pub mod examples;

pub mod prelude {
  pub use crate::core::*;
}
