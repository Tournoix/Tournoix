use web_sys::window;

use crate::components::notification::{Notif, NotifType};

// Very simple wrapper to make the code more concise for setting colors
// Example:
// input: "#bdfa14"
// output: "hello_world"
pub fn team_color_wrapper(text: String) -> String {
    format!("--team-color: {}", string_to_unique_color(text))
}

// Takes a string, hash it, and create a "unique" color from it
// There can be collisions, but it's fine
// This function returns a color in hex format
// Example:
// input: "Hello World"
// output: "#bdfa14"
pub fn string_to_unique_color(text: String) -> String {
    let mut hash = 0u32;
    for c in text.chars() {
        hash = c as u32 + hash.wrapping_mul(31);
    }

    let mut hsl = colorsys::Hsl::default();
    hsl.set_hue((hash % 360) as f64); //hue 0 -> 360
    hsl.set_saturation(((hash.wrapping_mul(17) % 20) + 80) as f64); //chroma 0 -> approximately 131
    hsl.set_lightness(((hash.wrapping_mul(23) % 10) + 30) as f64); //ligthness 0 -> 100

    let rgb = colorsys::Rgb::from(hsl);
    format!("#{:02x}{:02x}{:02x}", rgb.red() as u32, rgb.green() as u32, rgb.blue() as u32)
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
