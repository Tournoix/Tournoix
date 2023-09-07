pub mod component;
pub mod factory;

use std::fmt;

use time::Duration;
use uuid::Uuid;
use yew_notifications::Notifiable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotifType {
    Success,
    Warning,
    Error,
}

impl fmt::Display for NotifType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotifType::Success => write!(f, "Success"),
            NotifType::Warning => write!(f, "Warning"),
            NotifType::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomNotification {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub type_notif: NotifType,
    pub lifetime: Duration,

    full_lifetime: Duration,
    is_paused: bool,
    is_alive: bool,
}

impl CustomNotification {
    pub fn new(
        title: impl Into<String>,
        content: impl Into<String>,
        type_notif: NotifType,
        lifetime: Duration,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            content: content.into(),
            type_notif,

            lifetime,
            full_lifetime: lifetime,
            is_paused: false,
            is_alive: true
        }
    }
}

impl Notifiable for CustomNotification {
    fn id(&self) -> Uuid {
        self.id
    }

    fn apply_tick(&mut self, time: Duration) {
        self.lifetime = self
            .lifetime
            .checked_sub(time)
            .unwrap_or(Duration::default());
    }

    fn is_alive(&self) -> bool {
        self.lifetime != Duration::default()
    }

    fn mouse_in(&mut self) {
        self.is_paused = true;
    }

    fn mouse_out(&mut self) {
        self.is_paused = false;
        self.lifetime = self.full_lifetime;
    }

    fn is_paused(&self) -> bool {
        self.is_paused
    }
}
