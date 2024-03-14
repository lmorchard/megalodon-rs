use serde::Deserialize;

use super::Account;
use crate::entities as MegalodonEntities;

#[derive(Debug, Deserialize, Clone)]
pub struct Instance {
    pub uri: String,
    pub title: String,
    pub short_description: String,
    pub description: String,
    pub email: String,
    pub version: String,
    pub thumbnail: Option<String>,
    pub urls: URLs,
    pub stats: Stats,
    pub languages: Vec<String>,
    pub registrations: bool,
    pub approval_required: bool,
    pub invites_enabled: bool,
    pub max_toot_chars: Option<u32>,
    pub configuration: InstanceConfig,
    pub contact_account: Account,
}

impl From<Instance> for MegalodonEntities::Instance {
    fn from(val: Instance) -> Self {
        MegalodonEntities::Instance {
            uri: val.uri,
            title: val.title,
            description: val.description,
            email: val.email,
            version: val.version,
            thumbnail: val.thumbnail,
            urls: Some(val.urls.into()),
            stats: val.stats.into(),
            languages: val.languages,
            registrations: val.registrations,
            approval_required: val.approval_required,
            invites_enabled: Some(val.invites_enabled),
            configuration: val.configuration.into(),
            contact_account: Some(val.contact_account.into()),
            rules: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct URLs {
    pub streaming_api: String,
}

impl From<URLs> for MegalodonEntities::URLs {
    fn from(val: URLs) -> Self {
        MegalodonEntities::URLs {
            streaming_api: val.streaming_api,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Stats {
    pub user_count: u32,
    pub status_count: u64,
    pub domain_count: u32,
}

impl From<Stats> for MegalodonEntities::Stats {
    fn from(val: Stats) -> Self {
        MegalodonEntities::Stats {
            user_count: val.user_count,
            status_count: val.status_count,
            domain_count: val.domain_count,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct InstanceConfig {
    pub statuses: Statuses,
    pub media_attachments: MediaAttachments,
    pub polls: Polls,
}

impl From<InstanceConfig> for MegalodonEntities::instance::InstanceConfig {
    fn from(val: InstanceConfig) -> Self {
        MegalodonEntities::instance::InstanceConfig {
            statuses: val.statuses.into(),
            polls: Some(val.polls.into()),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Statuses {
    pub max_characters: u32,
    pub max_media_attachments: u32,
    pub characters_reserved_per_url: u32,
}

impl From<Statuses> for MegalodonEntities::instance::Statuses {
    fn from(val: Statuses) -> Self {
        MegalodonEntities::instance::Statuses {
            max_characters: val.max_characters,
            max_media_attachments: Some(val.max_media_attachments),
            characters_reserved_per_url: Some(val.characters_reserved_per_url),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaAttachments {
    pub supported_mime_types: Vec<String>,
    pub image_size_limit: u32,
    pub image_matrix_limit: u32,
    pub video_size_limit: u32,
    pub video_frame_rate_limit: u32,
    pub video_matrix_limit: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Polls {
    pub max_options: u32,
    pub max_characters_per_option: u32,
    pub min_expiration: u32,
    pub max_expiration: u32,
}

impl From<Polls> for MegalodonEntities::instance::Polls {
    fn from(val: Polls) -> Self {
        MegalodonEntities::instance::Polls {
            max_options: val.max_options,
            max_characters_per_option: val.max_characters_per_option,
            min_expiration: val.min_expiration,
            max_expiration: val.max_expiration,
        }
    }
}
