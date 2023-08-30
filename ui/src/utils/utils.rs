use web_sys::window;

pub fn fetch_notifs() -> Option<Result<serde_json::Value, serde_json::Error>> {
    if let Some(win) = window() {
        if let Ok(Some(store)) = win.local_storage() {
            if let Ok(Some(notifs)) = store.get_item("notifs") {
                let parsed: Result<serde_json::Value, _> = serde_json::from_str(&notifs);
                Some(parsed)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub fn clear_notifs() {
    if let Some(win) = window() {
        if let Ok(Some(store)) = win.local_storage() {
            if let Ok(_item) = store.set_item("notifs", "[]") { }
        }
    }
}

pub fn consume_notifs() -> Option<Result<serde_json::Value, serde_json::Error>> {
    let notifs = fetch_notifs();
    clear_notifs();
    notifs
}

pub fn add_delayed_notif(title: &str, content: &str) -> bool {
    let mut notif_string = format!("{{ \"title\": \"{}\", \"content\": \"{}\" }}", title, content);

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
