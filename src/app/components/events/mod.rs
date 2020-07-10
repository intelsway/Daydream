use matrix_sdk::events::AnyMessageEventStub;
use matrix_sdk::Room;
use url::Url;

use crate::app::matrix::types::get_media_download_url;

pub mod image;
pub mod notice;
pub mod text;
pub mod video;

pub trait EventExt {
    fn is_new_user(&self, prev_event: Option<&AnyMessageEventStub>) -> bool;
}

impl EventExt for AnyMessageEventStub {
    fn is_new_user(&self, prev_event: Option<&AnyMessageEventStub>) -> bool {
        if let Some(prev_event) = prev_event {
            prev_event.sender() != self.sender()
        } else {
            true
        }
    }
}

pub trait RoomExt {
    fn get_sender_displayname<'a>(&self, event: &'a AnyMessageEventStub) -> &'a str;
    fn get_sender_avatar<'a>(
        &self,
        homeserver_url: &'a Url,
        event: &'a AnyMessageEventStub,
    ) -> Option<Url>;
}

impl RoomExt for Room {
    fn get_sender_displayname<'a>(&self, event: &'a AnyMessageEventStub) -> &'a str {
        self.joined_members
            .get(&event.sender())
            .or_else(|| self.invited_members.get(&event.sender()))
            .and_then(|member| member.display_name.as_deref())
            .unwrap_or_else(|| event.sender().as_str())
    }

    fn get_sender_avatar<'a>(
        &self,
        homeserver_url: &'a Url,
        event: &'a AnyMessageEventStub,
    ) -> Option<Url> {
        let member = self
            .joined_members
            .get(&event.sender())
            .or_else(|| self.invited_members.get(&event.sender()))?;

        Some(get_media_download_url(
            homeserver_url,
            member.avatar_url.as_deref()?,
        ))
    }

}

