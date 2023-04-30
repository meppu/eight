#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/meppu/eight/main/.github/assets/eight.webp"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/meppu/eight/main/.github/assets/eight.webp"
)]
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
