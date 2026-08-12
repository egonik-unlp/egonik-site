//! The router-driven query tool that replaces the Code, Papers and Tools sections.
//!
//! Every item comes from the `pb-router` index compiled into the binary, so this
//! module needs no database, no server function, and no HTTP at all. Query and route
//! are logged to the browser console for inspection — nothing is sent anywhere.

pub mod components;
