/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format models for Discord API resources.
//!
//! Every type in this crate mirrors the JSON shape documented by Discord;
//! normalized state, caches, and other Cordion-specific runtime data belong
//! elsewhere. Optional fields are omitted during serialization when `None`
//! unless Discord requires an explicit `null`, which is documented per field.
//!
//! # Wire conventions
//!
//! - Snowflakes are decimal strings in JSON and [`id::Id`] values in Rust.
//! - Permission bitfields are decimal strings in JSON and
//!   [`permission::Permission`] values in Rust. Unknown bits are preserved.
//! - Timestamps are ISO 8601 strings exposed as [`std::time::SystemTime`].
//! - Values Discord may extend, such as enum-like integers and bitfields, keep
//!   the unrecognized value instead of failing to decode.
//! - Request and response payloads whose shape differs from a resource object
//!   get their own type in the owning resource's module, named after the
//!   endpoint they belong to.
//!
//! # Module map
//!
//! Resources Discord documents as objects live in the module named after that
//! resource. Related objects, partial variants, flags, and the request and
//! response payloads of that resource's endpoints live in submodules of it and
//! are re-exported at the module root, so `cordion_model::<resource>::Type` is
//! the path to remember.
//!
//! | Category | Modules | Contents |
//! | - | - | - |
//! | Core resources | [`user`], [`guild`], [`channel`], [`role`], [`emoji`], [`sticker`], [`message`], [`component`], [`interaction`], [`voice`], [`invite`], [`webhook`] | The resources most applications work with |
//! | Application resources | [`application`], [`application_command`], [`auto_moderation`], [`audit_log`] | Application configuration, its commands, moderation, and audit data |
//! | Events | [`event`] | Gateway payloads ([`event::gateway`]), HTTP event payloads ([`event::webhook_event`]), and presence and activity data ([`event::presence`]) |
//! | Monetization | [`monetization`] | [`monetization::sku`], [`monetization::subscription`], and [`monetization::entitlement`] |
//! | Social SDK | [`social`] | [`social::lobby`] and [`social::identity_profile`], used only by Discord Social SDK integrations |
//! | Cross-cutting | [`id`], [`permission`] | ID and permission types that every other module builds on |
//!
//! [`user`]: user
//! [`guild`]: guild
//! [`channel`]: channel
//! [`role`]: role
//! [`emoji`]: emoji
//! [`sticker`]: sticker
//! [`message`]: message
//! [`component`]: component
//! [`interaction`]: interaction
//! [`voice`]: voice
//! [`invite`]: invite
//! [`webhook`]: webhook
//! [`application`]: application
//! [`application_command`]: application_command
//! [`auto_moderation`]: auto_moderation
//! [`audit_log`]: audit_log
//! [`event`]: event
//! [`monetization`]: monetization
//! [`social`]: social
//! [`id`]: id
//! [`permission`]: permission
//!
//! See [Discord's API documentation](https://docs.discord.com/developers) for
//! the authoritative resource definitions.

extern crate self as cordion_model;

pub mod application;
pub mod application_command;
pub mod audit_log;
pub mod auto_moderation;
pub mod channel;
pub mod component;
pub mod emoji;
pub mod event;
pub mod guild;
pub mod id;
pub mod interaction;
pub mod invite;
pub mod message;
pub mod monetization;
pub mod permission;
pub mod role;
pub mod social;
pub mod sticker;
mod timestamp;
pub mod user;
pub mod voice;
pub mod webhook;
