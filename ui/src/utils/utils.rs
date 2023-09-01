use web_sys::window;

use crate::components::notification::{Notif, NotifType};

pub fn fetch_tournament(id: i32) -> Tournament {
    Tournament {
        id,
        name: "World Championship pétanque 2023".to_string(),
        description: "Ne manquez pas le rendez-vous incontournable pour tout les amateurs de boules qui se respecte. Au programme, suze, grillades, bière et bien sûr pétanque.".to_string(),
        date: chrono::NaiveDateTime::new(chrono::NaiveDate::from_ymd(2023, 9, 9), chrono::NaiveTime::from_hms(0, 0, 0)),
        location: "1450 Sainte-Croix, Avenue de la Gare 14".to_string(),
        is_qualif: true,
        is_elim: true,
        phase: 0,
        code: "mCCx34d".to_string(),
    }
}

pub struct Tournament {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub date: chrono::NaiveDateTime,
    pub location: String,
    pub is_qualif: bool,
    pub is_elim: bool,
    pub phase: i32,
    pub code: String,
}

pub fn fetch_notifs() -> Option<Vec<Notif>> {
    let store = window()?.local_storage().ok().flatten()?;
    let notifs = store.get_item("notifs").ok().flatten()?;
    log::info!("yo wtf {:?}", notifs);
    serde_json::from_str(&notifs).ok()
}

pub fn clear_notifs() {
    if let Some(win) = window() {
        if let Ok(Some(store)) = win.local_storage() {
            if let Ok(_item) = store.set_item("notifs", "[]") { }
        }
    }
}

pub fn consume_notifs() -> Option<Vec<Notif>> {
    let notifs = fetch_notifs();
    clear_notifs();
    notifs
}

pub fn add_delayed_notif(title: &str, content: &str, notif_type: NotifType) -> bool {
    let mut notif_string = format!("{{ \"id\": 0, \"title\": \"{}\", \"content\": \"{}\", \"type_notif\": \"{}\" }}", title, content, notif_type.to_string());
    log::info!("you goo bro? {:?}", notif_string);
    if let Some(win) = window() {
        if let Ok(Some(store)) = win.local_storage() {
            match store.get_item("notifs") {
                Ok(Some(mut item)) => {
                    if item == "[]" {
                        notif_string = format!("[{}]", notif_string);
                    } else {
                        item.pop(); // Remove last char ']'
                        notif_string = format!("{},{}]", item, notif_string);
                    }
                    if let Ok(_item) = store.set_item("notifs", notif_string.as_str()) {
                        ()
                    }
                }
                _ => {
                    if let Ok(_item) = store.set_item("notifs", format!("[{}]", notif_string).as_str()) {
                        ()
                    }
                }
            }
        }
    }

    false
}
