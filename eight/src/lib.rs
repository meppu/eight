#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod result;
pub use result::*;

pub mod messaging;

#[cfg(feature = "embedded")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded")))]
pub mod embedded;

#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
mod macros;
