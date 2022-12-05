use std::collections::HashMap;
use xdotool::window::{get_window_focus, get_window_name, get_window_pid};

use std::fs;

use crate::{define_emitter, emitter::Emitted};

const TITLE_MAX_LEN: usize = 20 + TITLE_SHORTENER.len();
const TITLE_SHORTENER: &str = "…";

define_emitter!(
    TitleEmitter,
    "title",
    |alignment, fg_color, bg_color, _icon| {
        let name = Self::get_name();
        let mut display_name = name.clone();
        if name.len() > TITLE_MAX_LEN {
            display_name.truncate(TITLE_MAX_LEN - TITLE_SHORTENER.len());
            display_name += TITLE_SHORTENER;
        };

        let pid = Self::get_pid();
        let proc_path = format!("/proc/{}/task/{}/cmdline", pid, pid);
        let cmdline = fs::read_to_string(proc_path).unwrap_or_else(|_| "ERROR".to_string());

        Emitted {
            content: display_name,
            bg_color: String::from(bg_color),
            fg_color: String::from(fg_color),
            icon: Self::get_icon(name, cmdline),
            kind: String::from("title"),
            alignment: alignment.clone(),
        }
    }
);

impl TitleEmitter {
    fn get_focused_window() -> String {
        String::from_utf8(get_window_focus("").stdout).unwrap_or_else(|_| String::from("ERROR"))
    }

    fn get_name() -> String {
        let focused = Self::get_focused_window();
        String::from_utf8(get_window_name(focused.trim()).stdout)
            .unwrap_or_default()
            .trim()
            .to_owned()
    }

    fn get_pid() -> i32 {
        let focused = Self::get_focused_window();
        String::from_utf8(get_window_pid(focused.trim()).stdout)
            .unwrap_or_else(|_| String::from("-1"))
            .trim()
            .parse::<i32>()
            .unwrap_or(0)
    }

    fn get_icon(title: String, cmdline: String) -> String {
        let by_cmdline: HashMap<&str, &str> = HashMap::from([
            ("firefox", "\u{f269}"),
            ("codium", "\u{f121}"),
            ("Discord", "\u{f392}"),
            ("urxvt", "\u{f120}"),
            ("chromium", "\u{f268}"),
            ("openbox", "\u{f015}"),
            ("telegram-desktop", "\u{f2c6}"),
            ("mpv", "\u{f144}"),
        ]);

        let sub_apps: HashMap<&str, &str> = HashMap::from([
            ("weechat", "\u{f27a}"),
            ("WhatsApp", "\u{f232}"),
            ("DuckDuckGo", "\u{f002}"),
            ("YouTube", "\u{f167}"),
            ("GitHub", "\u{f09b}"),
            ("Wikipedia", "\u{f266}"),
        ]);

        for (k, v) in sub_apps {
            if title.contains(k) {
                return v.to_owned();
            }
        }

        for (k, v) in by_cmdline {
            if cmdline.contains(k) {
                return v.to_owned();
            }
        }

        String::from("\u{f2d0}")
    }
}
