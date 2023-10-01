#![deny(missing_docs)]

//! Declarative components to represent control-flow and other useful
//! constructs in the [`leptos`] web framework not directly
//! provided by default.
//!
//! This crate provides 2 main components
//!
//! - [`If`](if_::If)
//! - [`PortalInput`](portal::PortalInput)
//!
//! # Usage
//! For more usage examples, please refer to the respective
//! components' documentation, but here's a taste.
//!
//! ## If
//! ```rust
//! use leptos::*;
//! use leptos_declarative::prelude::*;
//!
//! # let runtime = create_runtime();
//! let (a, _) = create_signal(true);
//! let (b, _) = create_signal(false);
//!
//! view! {
//! <If signal=a>
//!   <Then>"A is true!"</Then>
//!   <ElseIf signal=b>"B is true!"</ElseIf>
//!   <Else>"Both A and B are false!"</Else>
//! </If>
//! };
//! # runtime.dispose();
//! ```
//!
//! ## Portal
//! ```rust
//! use leptos::*;
//! use leptos_declarative::prelude::*;
//!
//! # let runtime = create_runtime();
//!
//! struct PortalId;
//!
//! view! {
//!   <PortalProvider>
//!     <div>
//!       <h1>"Portal goes here!"</h1>
//!       <PortalOutput id=PortalId />
//!     </div>
//!
//!     <PortalInput id=PortalId>
//!       <p>"I went through the portal!"</p>
//!     </PortalInput>
//!   </PortalProvider>
//! };
//! # runtime.dispose();
//! ```

#[macro_use]
mod util;
pub mod if_;
pub mod portal;

/// Convenient import of all components.
pub mod prelude {
    pub use crate::{if_::*, portal::*};
}
