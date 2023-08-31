use web_sys::window;

use crate::components::notification::{Notif, NotifType};

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
