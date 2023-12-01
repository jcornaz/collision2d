#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple 2d collision detection library supporting simple primitive shapes only,
//! and that is suitable for development on `no_std` targets.
//!
//! # Features
//!
//! `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.
