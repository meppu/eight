#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod embedded;

#[cfg(feature = "client")]
#[cfg_attr(docsrs, doc(cfg(feature = "client")))]
pub mod client;

#[cfg(feature = "expose")]
#[cfg_attr(docsrs, doc(cfg(feature = "expose")))]
pub mod expose;

#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
mod macros;

// crate-only macros here
macro_rules! err {
    ($module: ident, $name:ident) => {
        crate::$module::Error::$name
    };

    ($fmt:expr, $token:expr) => {
        crate::embedded::Error::CommandError($fmt.to_string(), $token.line, $token.column)
    };
}

pub(crate) use err;
