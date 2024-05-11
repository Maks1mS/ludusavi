//! # Note on version 0.10.3
//! 
//! This version of `self_cell` is a re-export of the `self_cell` version `1`
//! meant for backwards-compatibly allowing `0.10` users to get current and
//! future soundness fixes. If you are adding `self_cell` as a dependency,
//! please use an up-to-date version directly.
//! 
//! # Overview
//!
//! `self_cell` provides one macro-rules macro: [`self_cell`][`self_cell!`]. With this macro
//! you can create self-referential structs that are safe-to-use in stable Rust,
//! without leaking the struct internal lifetime.
//!
//! In a nutshell, the API looks *roughly* like this:
//!
//! ```ignore
//! // User code:
//!
//! self_cell!(
//!     struct NewStructName {
//!         owner: Owner,
//!
//!         #[covariant]
//!         dependent: Dependent,
//!     }
//!
//!     impl {Debug}
//! );
//!
//! // Generated by macro:
//!
//! struct NewStructName(...);
//!
//! impl NewStructName {
//!     fn new(
//!         owner: Owner,
//!         dependent_builder: impl for<'a> FnOnce(&'a Owner) -> Dependent<'a>
//!     ) -> NewStructName { ... }
//!     fn borrow_owner<'a>(&'a self) -> &'a Owner { ... }
//!     fn borrow_dependent<'a>(&'a self) -> &'a Dependent<'a> { ... }
//! }
//!
//! impl Debug for NewStructName { ... }
//! ```
//!
//! Self-referential structs are currently not supported with safe vanilla Rust.
//! The only reasonable safe alternative is to have the user juggle 2 separate
//! data structures which is a mess. The library solution ouroboros is really
//! expensive to compile due to its use of procedural macros.
//!
//! This alternative is `no_std`, uses no proc-macros, some self contained
//! unsafe and works on stable Rust, and is miri tested. With a total of less
//! than 300 lines of implementation code, which consists mostly of type and
//! trait implementations, this crate aims to be a good minimal solution to the
//! problem of self-referential structs.
//!
//! It has undergone [community code
//! review](https://users.rust-lang.org/t/experimental-safe-to-use-proc-macro-free-self-referential-structs-in-stable-rust/52775)
//! from experienced Rust users.
//!
//! ### Fast compile times
//!
//! ```txt
//! $ rm -rf target && cargo +nightly build -Z timings
//!
//! Compiling self_cell v0.7.0
//! Completed self_cell v0.7.0 in 0.2s
//! ```
//!
//! Because it does **not** use proc-macros, and has 0 dependencies
//! compile-times are fast.
//!
//! Measurements done on a slow laptop.
//!
//! ### A motivating use case
//!
//! ```rust
//! use self_cell::self_cell;
//!
//! #[derive(Debug, Eq, PartialEq)]
//! struct Ast<'a>(pub Vec<&'a str>);
//!
//! self_cell!(
//!     struct AstCell {
//!         owner: String,
//!
//!         #[covariant]
//!         dependent: Ast,
//!     }
//!
//!     impl {Debug, Eq, PartialEq}
//! );
//!
//! fn build_ast_cell(code: &str) -> AstCell {
//!     // Create owning String on stack.
//!     let pre_processed_code = code.trim().to_string();
//!
//!     // Move String into AstCell, then build Ast inplace.
//!     AstCell::new(
//!        pre_processed_code,
//!        |code| Ast(code.split(' ').filter(|word| word.len() > 1).collect())
//!     )
//! }
//!
//! fn main() {
//!     let ast_cell = build_ast_cell("fox = cat + dog");
//!
//!     println!("ast_cell -> {:?}", &ast_cell);
//!     println!("ast_cell.borrow_owner() -> {:?}", ast_cell.borrow_owner());
//!     println!("ast_cell.borrow_dependent().0[1] -> {:?}", ast_cell.borrow_dependent().0[1]);
//! }
//! ```
//!
//! ```txt
//! $ cargo run
//!
//! ast_cell -> AstCell { owner: "fox = cat + dog", dependent: Ast(["fox", "cat", "dog"]) }
//! ast_cell.borrow_owner() -> "fox = cat + dog"
//! ast_cell.borrow_dependent().0[1] -> "cat"
//! ```
//!
//! There is no way in safe Rust to have an API like `build_ast_cell`, as soon
//! as `Ast` depends on stack variables like `pre_processed_code` you can't
//! return the value out of the function anymore. You could move the
//! pre-processing into the caller but that gets ugly quickly because you can't
//! encapsulate things anymore. Note this is a somewhat niche use case,
//! self-referential structs should only be used when there is no good
//! alternative.
//!
//! Under the hood, it heap allocates a struct which it initializes first by
//! moving the owner value to it and then using the reference to this now
//! Pin/Immovable owner to construct the dependent inplace next to it. This
//! makes it safe to move the generated SelfCell but you have to pay for the
//! heap allocation.
//!
//! See the documentation for [`self_cell`][`self_cell!`] to dive further into the details.
//!
//! Or take a look at the advanced examples:
//! - [Example how to handle dependent construction that can
//!   fail](https://github.com/Voultapher/self_cell/tree/main/examples/fallible_dependent_construction)
//!
//! - [How to build a lazy AST with
//!   self_cell](https://github.com/Voultapher/self_cell/tree/main/examples/lazy_ast)
//!
//! - [How to use an owner type with
//!     lifetime](https://github.com/Voultapher/self_cell/tree/main/examples/owner_with_lifetime)
//!
//! ### Min required rustc version
//!
//! By default the minimum required rustc version is 1.51.
//!
//! There is an optional feature you can enable called "old_rust" that enables
//! support down to rustc version 1.36. However this requires polyfilling std
//! library functionality for older rustc with technically UB versions. Testing
//! does not show older rustc versions (ab)using this. Use at your own risk.
//!
//! The minimum versions are a best effor and may change with any new major
//! release.

#![no_std]

pub use new_self_cell::self_cell;