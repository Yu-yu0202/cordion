/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use std::time::SystemTime;

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::guild::{GuildId, GuildMember};
use crate::id::{Id, Identifiable};
use crate::user::{User, UserId};

/// The ID of a [`GuildScheduledEvent`].
pub type GuildScheduledEventId = Id<GuildScheduledEvent>;

/// The target of [`GuildScheduledEvent::entity_id`].
///
/// The entity is a stage instance or voice channel depending on
/// [`GuildScheduledEvent::entity_type`], so no concrete resource type is
/// associated with this marker.
#[derive(Clone, Copy, PartialEq, Eq, IdTarget, Debug)]
pub struct GuildScheduledEventEntity;

/// The ID of the entity associated with a [`GuildScheduledEvent`].
pub type GuildScheduledEventEntityId = Id<GuildScheduledEventEntity>;

/// The privacy level of a [`GuildScheduledEvent`].
///
/// Unrecognized values are preserved. See [Discord's scheduled event privacy
/// level][level] for details.
///
/// [level]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-privacy-level
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GuildScheduledEventPrivacyLevel(u8);

impl GuildScheduledEventPrivacyLevel {
    /// The event is only accessible to guild members.
    pub const GUILD_ONLY: Self = Self(2);
}

/// The status of a [`GuildScheduledEvent`].
///
/// Unrecognized values are preserved. Discord only permits the transitions
/// `SCHEDULED -> ACTIVE`, `ACTIVE -> COMPLETED`, and `SCHEDULED -> CANCELED`.
/// See [Discord's scheduled event status][status] for details.
///
/// [status]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-status
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GuildScheduledEventStatus(u8);

impl GuildScheduledEventStatus {
    /// The event is scheduled to run.
    pub const SCHEDULED: Self = Self(1);

    /// The event is currently running.
    pub const ACTIVE: Self = Self(2);

    /// The event has completed.
    pub const COMPLETED: Self = Self(3);

    /// The event was canceled.
    pub const CANCELED: Self = Self(4);
}

/// The entity type of a [`GuildScheduledEvent`].
///
/// Unrecognized values are preserved. See [Discord's scheduled event entity
/// types][types] for details.
///
/// [types]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GuildScheduledEventEntityType(u8);

impl GuildScheduledEventEntityType {
    /// The event is hosted in a stage channel.
    pub const STAGE_INSTANCE: Self = Self(1);

    /// The event is hosted in a voice channel.
    pub const VOICE: Self = Self(2);

    /// The event is hosted outside of Discord.
    pub const EXTERNAL: Self = Self(3);
}

/// Additional metadata for a [`GuildScheduledEvent`].
///
/// See [Discord's entity metadata object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-metadata
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildScheduledEventEntityMetadata {
    /// The location of an external event, between 1 and 100 characters.
    ///
    /// Required by Discord for events with
    /// [`GuildScheduledEventEntityType::EXTERNAL`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// A scheduled event in a guild.
///
/// [`channel_id`](Self::channel_id), [`scheduled_end_time`](Self::scheduled_end_time),
/// [`entity_id`](Self::entity_id), [`entity_metadata`](Self::entity_metadata), and
/// [`recurrence_rule`](Self::recurrence_rule) are always present and serialize
/// as `null` when unused. Which of them must be non-null depends on
/// [`entity_type`](Self::entity_type).
///
/// See [Discord's scheduled event object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct GuildScheduledEvent {
    /// The event's ID.
    pub id: GuildScheduledEventId,

    /// The guild this event belongs to.
    pub guild_id: GuildId,

    /// The channel where the event is hosted.
    ///
    /// `None` for events with [`GuildScheduledEventEntityType::EXTERNAL`].
    pub channel_id: Option<ChannelId>,

    /// The user who created the event.
    ///
    /// Discord sends `null` for events created before October 25, 2021, and may
    /// omit this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserId>,

    /// The event's name, between 1 and 100 characters.
    pub name: String,

    /// The event's description, between 1 and 1000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// When the event starts.
    #[serde(with = "crate::timestamp")]
    pub scheduled_start_time: SystemTime,

    /// When the event ends.
    ///
    /// Required by Discord for events with
    /// [`GuildScheduledEventEntityType::EXTERNAL`]. This field is always
    /// present and serializes as `null` when unused.
    #[serde(with = "crate::timestamp::option")]
    pub scheduled_end_time: Option<SystemTime>,

    /// The event's privacy level.
    pub privacy_level: GuildScheduledEventPrivacyLevel,

    /// The event's status.
    pub status: GuildScheduledEventStatus,

    /// The event's entity type.
    pub entity_type: GuildScheduledEventEntityType,

    /// The ID of the entity associated with the event.
    ///
    /// This field is always present and serializes as `null` when unused.
    pub entity_id: Option<GuildScheduledEventEntityId>,

    /// Additional metadata for the event.
    ///
    /// Required with a non-null
    /// [`location`](GuildScheduledEventEntityMetadata::location) for events
    /// with [`GuildScheduledEventEntityType::EXTERNAL`]. This field is always
    /// present and serializes as `null` when unused.
    pub entity_metadata: Option<GuildScheduledEventEntityMetadata>,

    /// The user who created the event.
    ///
    /// Not included for events created before October 25, 2021.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,

    /// The number of users subscribed to the event.
    ///
    /// Only included when the request uses `with_user_count=true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<u64>,

    /// The event's cover image hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// How often the event recurs.
    ///
    /// This field is always present and serializes as `null` for non-recurring
    /// events.
    pub recurrence_rule: Option<GuildScheduledEventRecurrenceRule>,
}

impl Identifiable for GuildScheduledEvent {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// A user subscribed to a [`GuildScheduledEvent`].
///
/// Returned by `GET /guilds/{guild.id}/scheduled-events/{guild_scheduled_event.id}/users`.
/// [`member`](Self::member) is only included when the request sets
/// `with_member=true` and the guild member data exists, so it is optional and
/// omitted during serialization when `None`.
///
/// See [Discord's guild scheduled event user object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-user-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildScheduledEventUser {
    /// The ID of the event the user subscribed to.
    pub guild_scheduled_event_id: GuildScheduledEventId,

    /// The user who subscribed to the event.
    pub user: User,

    /// The user's guild member data for the event's guild, when included.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
}

/// How often a [`GuildScheduledEventRecurrenceRule`] repeats.
///
/// Unrecognized values are preserved. See [Discord's recurrence frequencies][frequencies]
/// for details.
///
/// [frequencies]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-frequency
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GuildScheduledEventRecurrenceFrequency(u8);

impl GuildScheduledEventRecurrenceFrequency {
    /// The event recurs yearly.
    pub const YEARLY: Self = Self(0);

    /// The event recurs monthly.
    pub const MONTHLY: Self = Self(1);

    /// The event recurs weekly.
    pub const WEEKLY: Self = Self(2);

    /// The event recurs daily.
    pub const DAILY: Self = Self(3);
}

/// A day of the week in a [`GuildScheduledEventRecurrenceRule`].
///
/// Unrecognized values are preserved. See [Discord's weekday values][weekdays]
/// for details.
///
/// [weekdays]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-weekday
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GuildScheduledEventRecurrenceWeekday(u8);

impl GuildScheduledEventRecurrenceWeekday {
    /// Monday.
    pub const MONDAY: Self = Self(0);

    /// Tuesday.
    pub const TUESDAY: Self = Self(1);

    /// Wednesday.
    pub const WEDNESDAY: Self = Self(2);

    /// Thursday.
    pub const THURSDAY: Self = Self(3);

    /// Friday.
    pub const FRIDAY: Self = Self(4);

    /// Saturday.
    pub const SATURDAY: Self = Self(5);

    /// Sunday.
    pub const SUNDAY: Self = Self(6);
}

/// A month in a [`GuildScheduledEventRecurrenceRule`].
///
/// Unrecognized values are preserved. See [Discord's month values][months] for
/// details.
///
/// [months]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-month
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GuildScheduledEventRecurrenceMonth(u8);

impl GuildScheduledEventRecurrenceMonth {
    /// January.
    pub const JANUARY: Self = Self(1);

    /// February.
    pub const FEBRUARY: Self = Self(2);

    /// March.
    pub const MARCH: Self = Self(3);

    /// April.
    pub const APRIL: Self = Self(4);

    /// May.
    pub const MAY: Self = Self(5);

    /// June.
    pub const JUNE: Self = Self(6);

    /// July.
    pub const JULY: Self = Self(7);

    /// August.
    pub const AUGUST: Self = Self(8);

    /// September.
    pub const SEPTEMBER: Self = Self(9);

    /// October.
    pub const OCTOBER: Self = Self(10);

    /// November.
    pub const NOVEMBER: Self = Self(11);

    /// December.
    pub const DECEMBER: Self = Self(12);
}

/// A specific weekday within a specific week of a month.
///
/// See [Discord's `n_weekday` structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-nweekday-structure
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildScheduledEventRecurrenceNWeekday {
    /// The week of the month, from 1 to 5.
    pub n: u8,

    /// The day of the week.
    pub day: GuildScheduledEventRecurrenceWeekday,
}

/// The definition for how often a [`GuildScheduledEvent`] recurs.
///
/// Fields are a subset of the iCalendar `RRULE` behavior. Discord currently
/// does not allow clients to set `end`, `by_year_day`, or `count`.
///
/// See [Discord's recurrence rule object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildScheduledEventRecurrenceRule {
    /// The start of the recurrence interval.
    #[serde(with = "crate::timestamp")]
    pub start: SystemTime,

    /// The end of the recurrence interval.
    ///
    /// Clients cannot currently set this field. It is always present and
    /// serializes as `null` when unused.
    #[serde(with = "crate::timestamp::option")]
    pub end: Option<SystemTime>,

    /// How often the event occurs.
    pub frequency: GuildScheduledEventRecurrenceFrequency,

    /// The spacing between occurrences, defined in units of
    /// [`frequency`](Self::frequency).
    pub interval: u32,

    /// Days within a week for the event to recur on.
    ///
    /// This field is always present and serializes as `null` when unused.
    pub by_weekday: Option<Vec<GuildScheduledEventRecurrenceWeekday>>,

    /// Specific weekdays within a month for the event to recur on.
    ///
    /// This field is always present and serializes as `null` when unused.
    pub by_n_weekday: Option<Vec<GuildScheduledEventRecurrenceNWeekday>>,

    /// Months for the event to recur on.
    ///
    /// This field is always present and serializes as `null` when unused.
    pub by_month: Option<Vec<GuildScheduledEventRecurrenceMonth>>,

    /// Dates within a month for the event to recur on.
    ///
    /// This field is always present and serializes as `null` when unused.
    pub by_month_day: Option<Vec<u8>>,

    /// Days within a year for the event to recur on, from 1 to 364.
    ///
    /// Clients cannot currently set this field. It is always present and
    /// serializes as `null` when unused.
    pub by_year_day: Option<Vec<u16>>,

    /// The total number of times the event may recur.
    ///
    /// Clients cannot currently set this field. It is always present and
    /// serializes as `null` when unused.
    pub count: Option<u32>,
}

/// The body of `POST /guilds/{guild.id}/scheduled-events`.
///
/// Discord documents [`name`](Self::name),
/// [`privacy_level`](Self::privacy_level),
/// [`scheduled_start_time`](Self::scheduled_start_time), and
/// [`entity_type`](Self::entity_type) as required and the remaining fields as
/// optional, so the optional fields are omitted during serialization when
/// `None`.
///
/// [`channel_id`](Self::channel_id) and [`entity_metadata`](Self::entity_metadata)
/// depend on [`entity_type`](Self::entity_type): Discord requires a non-null
/// `channel_id` and a null `entity_metadata` for `STAGE_INSTANCE` and `VOICE`
/// events, and the opposite for `EXTERNAL` events, whose
/// [`entity_metadata`](Self::entity_metadata) must carry a `location`.
/// [`scheduled_end_time`](Self::scheduled_end_time) is required for `EXTERNAL`
/// events.
///
/// [`image`](Self::image) carries image data as a [Data URI scheme] string, for
/// example `data:image/png;base64,BASE64_ENCODED_PNG_IMAGE_DATA`.
///
/// See [Discord's create guild scheduled event endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild-scheduled-event#create-guild-scheduled-event
/// [Data URI scheme]: https://docs.discord.com/developers/reference#image-data
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateGuildScheduledEventRequest {
    /// The channel where the event is hosted.
    ///
    /// Required for events with [`GuildScheduledEventEntityType::STAGE_INSTANCE`]
    /// and [`GuildScheduledEventEntityType::VOICE`]; Discord documents it as
    /// optional for events with [`GuildScheduledEventEntityType::EXTERNAL`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// Additional metadata for the event.
    ///
    /// Required with a non-null `location` for events with
    /// [`GuildScheduledEventEntityType::EXTERNAL`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_metadata: Option<GuildScheduledEventEntityMetadata>,

    /// The event's name, between 1 and 100 characters.
    pub name: String,

    /// The event's privacy level.
    pub privacy_level: GuildScheduledEventPrivacyLevel,

    /// When the event starts.
    #[serde(with = "crate::timestamp")]
    pub scheduled_start_time: SystemTime,

    /// When the event ends.
    ///
    /// Required by Discord for events with
    /// [`GuildScheduledEventEntityType::EXTERNAL`].
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_end_time: Option<SystemTime>,

    /// The event's description, between 1 and 1000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The event's entity type.
    pub entity_type: GuildScheduledEventEntityType,

    /// The event's cover image as image data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// The definition for how often the event recurs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence_rule: Option<GuildScheduledEventRecurrenceRule>,
}

/// The body of `PATCH /guilds/{guild.id}/scheduled-events/{guild_scheduled_event.id}`.
///
/// Discord documents every parameter of this endpoint as optional, so every
/// field is omitted during serialization when `None` and an omitted field
/// leaves the event unchanged. Because omission means "leave unchanged", the
/// fields Discord documents as nullable cannot be cleared by sending `null`
/// through this type.
///
/// Discord silently discards [`entity_metadata`](Self::entity_metadata) for
/// events that are not [`GuildScheduledEventEntityType::EXTERNAL`].
///
/// [`image`](Self::image) carries image data as a [Data URI scheme] string, for
/// example `data:image/png;base64,BASE64_ENCODED_PNG_IMAGE_DATA`.
///
/// See [Discord's modify guild scheduled event endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild-scheduled-event#modify-guild-scheduled-event
/// [Data URI scheme]: https://docs.discord.com/developers/reference#image-data
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildScheduledEventRequest {
    /// The channel where the event is hosted.
    ///
    /// Discord documents this field as nullable and requires `null` when the
    /// event's [`entity_type`](Self::entity_type) changes to
    /// [`GuildScheduledEventEntityType::EXTERNAL`]; an omitted field leaves the
    /// channel unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// Additional metadata for the event.
    ///
    /// Required with a `location` field when the event's
    /// [`entity_type`](Self::entity_type) changes to
    /// [`GuildScheduledEventEntityType::EXTERNAL`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_metadata: Option<GuildScheduledEventEntityMetadata>,

    /// The event's name, between 1 and 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The event's privacy level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<GuildScheduledEventPrivacyLevel>,

    /// When the event starts.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_start_time: Option<SystemTime>,

    /// When the event ends.
    ///
    /// Required by Discord when the event's
    /// [`entity_type`](Self::entity_type) changes to
    /// [`GuildScheduledEventEntityType::EXTERNAL`].
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_end_time: Option<SystemTime>,

    /// The event's description, between 1 and 1000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The event's entity type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<GuildScheduledEventEntityType>,

    /// The event's status.
    ///
    /// Use this field to start or end an event. Discord only permits the
    /// transitions `SCHEDULED -> ACTIVE`, `ACTIVE -> COMPLETED`, and
    /// `SCHEDULED -> CANCELED`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GuildScheduledEventStatus>,

    /// The event's cover image as image data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// The definition for how often the event recurs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence_rule: Option<GuildScheduledEventRecurrenceRule>,
}
