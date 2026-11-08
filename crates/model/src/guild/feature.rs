/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Guild feature strings and their documented constants.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// A guild feature string.
///
/// Discord sends features as an array of strings. Documented features are
/// exposed as associated constants, and unrecognized values are preserved when
/// decoding. Use [`as_str`](Self::as_str) to inspect the original string.
///
/// See [Discord's guild features][features] for the current list.
///
/// [features]: https://docs.discord.com/developers/resources/guild#guild-object-guild-features
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GuildFeature(Cow<'static, str>);

impl GuildFeature {
    /// Creates an owned feature from any string-like value.
    ///
    /// Prefer [`GuildFeature::from`] with a `&'static str` to borrow instead of
    /// allocating.
    #[inline]
    #[must_use]
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(Cow::Owned(value.into()))
    }

    /// Returns the feature string exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl GuildFeature {
    /// The guild has access to set an animated guild banner image.
    pub const ANIMATED_BANNER: Self = Self(Cow::Borrowed("ANIMATED_BANNER"));

    /// The guild has access to set an animated guild icon.
    pub const ANIMATED_ICON: Self = Self(Cow::Borrowed("ANIMATED_ICON"));

    /// The guild is using the old permissions configuration behavior.
    pub const APPLICATION_COMMAND_PERMISSIONS_V2: Self =
        Self(Cow::Borrowed("APPLICATION_COMMAND_PERMISSIONS_V2"));

    /// The guild has set up auto moderation rules.
    pub const AUTO_MODERATION: Self = Self(Cow::Borrowed("AUTO_MODERATION"));

    /// The guild has access to set a guild banner image.
    pub const BANNER: Self = Self(Cow::Borrowed("BANNER"));

    /// The guild can enable the welcome screen, Membership Screening, stage
    /// channels, and discovery, and receives community updates.
    pub const COMMUNITY: Self = Self(Cow::Borrowed("COMMUNITY"));

    /// The guild has enabled monetization.
    pub const CREATOR_MONETIZABLE_PROVISIONAL: Self =
        Self(Cow::Borrowed("CREATOR_MONETIZABLE_PROVISIONAL"));

    /// The guild has enabled the role subscription promo page.
    pub const CREATOR_STORE_PAGE: Self = Self(Cow::Borrowed("CREATOR_STORE_PAGE"));

    /// The guild has been set as a support server on the App Directory.
    pub const DEVELOPER_SUPPORT_SERVER: Self = Self(Cow::Borrowed("DEVELOPER_SUPPORT_SERVER"));

    /// The guild can be discovered in the directory.
    pub const DISCOVERABLE: Self = Self(Cow::Borrowed("DISCOVERABLE"));

    /// The guild is able to set gradient colors to roles.
    pub const ENHANCED_ROLE_COLORS: Self = Self(Cow::Borrowed("ENHANCED_ROLE_COLORS"));

    /// The guild is able to be featured in the directory.
    pub const FEATURABLE: Self = Self(Cow::Borrowed("FEATURABLE"));

    /// The guild has access to set guild tags.
    pub const GUILD_TAGS: Self = Self(Cow::Borrowed("GUILD_TAGS"));

    /// The guild has access to guest invites.
    pub const GUESTS_ENABLED: Self = Self(Cow::Borrowed("GUESTS_ENABLED"));

    /// The guild has paused invites, preventing new users from joining.
    pub const INVITES_DISABLED: Self = Self(Cow::Borrowed("INVITES_DISABLED"));

    /// The guild has access to set an invite splash background.
    pub const INVITE_SPLASH: Self = Self(Cow::Borrowed("INVITE_SPLASH"));

    /// The guild has enabled Membership Screening.
    pub const MEMBER_VERIFICATION_GATE_ENABLED: Self =
        Self(Cow::Borrowed("MEMBER_VERIFICATION_GATE_ENABLED"));

    /// The guild has increased custom soundboard sound slots.
    pub const MORE_SOUNDBOARD: Self = Self(Cow::Borrowed("MORE_SOUNDBOARD"));

    /// The guild has increased custom sticker slots.
    pub const MORE_STICKERS: Self = Self(Cow::Borrowed("MORE_STICKERS"));

    /// The guild has access to create announcement channels.
    pub const NEWS: Self = Self(Cow::Borrowed("NEWS"));

    /// The guild is partnered.
    pub const PARTNERED: Self = Self(Cow::Borrowed("PARTNERED"));

    /// The guild can be previewed before joining via Membership Screening or the directory.
    pub const PREVIEW_ENABLED: Self = Self(Cow::Borrowed("PREVIEW_ENABLED"));

    /// The guild has enabled requiring admin to prune members.
    pub const PRUNE_REQUIRES_ADMIN: Self = Self(Cow::Borrowed("PRUNE_REQUIRES_ADMIN"));

    /// The guild has disabled alerts for join raids in the configured safety
    /// alerts channel.
    pub const RAID_ALERTS_DISABLED: Self = Self(Cow::Borrowed("RAID_ALERTS_DISABLED"));

    /// The guild is able to set role icons.
    pub const ROLE_ICONS: Self = Self(Cow::Borrowed("ROLE_ICONS"));

    /// The guild has role subscriptions that can be purchased.
    pub const ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE: Self =
        Self(Cow::Borrowed("ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE"));

    /// The guild has enabled role subscriptions.
    pub const ROLE_SUBSCRIPTIONS_ENABLED: Self = Self(Cow::Borrowed("ROLE_SUBSCRIPTIONS_ENABLED"));

    /// The guild has created soundboard sounds.
    pub const SOUNDBOARD: Self = Self(Cow::Borrowed("SOUNDBOARD"));

    /// The guild has enabled ticketed events.
    pub const TICKETED_EVENTS_ENABLED: Self = Self(Cow::Borrowed("TICKETED_EVENTS_ENABLED"));

    /// The guild has access to set a vanity URL.
    pub const VANITY_URL: Self = Self(Cow::Borrowed("VANITY_URL"));

    /// The guild is verified.
    pub const VERIFIED: Self = Self(Cow::Borrowed("VERIFIED"));

    /// The guild has access to set 384kbps bitrate in voice.
    ///
    /// Previously called "VIP voice servers".
    pub const VIP_REGIONS: Self = Self(Cow::Borrowed("VIP_REGIONS"));

    /// The guild has enabled the welcome screen.
    pub const WELCOME_SCREEN_ENABLED: Self = Self(Cow::Borrowed("WELCOME_SCREEN_ENABLED"));
}

impl<T: Into<Cow<'static, str>>> From<T> for GuildFeature {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
