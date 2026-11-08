/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Procedural macros for Cordion.
//!
//! These macros generate implementations for traits defined by
//! `cordion-model`; they provide no standalone functionality.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derives `cordion_model::id::IdTarget` for a struct, enum, or union.
///
/// `IdTarget` is a marker trait that associates a resource type with
/// `cordion_model::id::Id<T>`, preventing IDs for different resources from
/// being mixed up. The generated implementation is empty.
#[proc_macro_derive(IdTarget)]
pub fn derive_id_target(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::cordion_model::id::IdTarget
            for #name #ty_generics #where_clause {}
    }
    .into()
}
