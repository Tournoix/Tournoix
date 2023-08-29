use web_sys::{window, console};

pub fn add_notif(title: &str, content: &str) -> bool {
    let notif_string = format!("{{ \"title\": \"{}\", \"content\": \"{}\" }}", title, content);
    // let notif_string = r#"{ "title": "Test", "content": "This is a test notification" }"#;

    console::log_1(&notif_string.as_str().into());

    if let Some(win) = window() {
        if let Ok(Some(store)) = win.local_storage() {
            match store.get_item("notifs") {
                Ok(Some(item)) => {
                    // TODO add notif to notifs list
                    if let Ok(item) = store.set_item("notifs", notif_string.as_str()) {
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
            if let Ok(item) = store.set_item("notifs", "[]") { }
        }
    }
}

pub fn consume_notifs() -> Option<Result<serde_json::Value, serde_json::Error>> {
    let notifs = fetch_notifs();
    clear_notifs();
    notifs
}
