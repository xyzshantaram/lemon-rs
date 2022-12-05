use crate::{define_emitter, emitter::Emitted};
use chrono::{Local, Timelike};

define_emitter!(
    ClockEmitter,
    "clock",
    |alignment, fg_color, _bgcolor, icon| {
        let now = Local::now();
        let hour = now.hour();
        let bg_color = if hour < 6 {
            "#632b6c"
        } else if (6..12).contains(&hour) {
            "#f0888c"
        } else if (12..18).contains(&hour) {
            "#f28e59"
        } else {
            "#2a0e37"
        };
        Emitted {
            content: format!("{}", now.format("%Y-%m-%d (%A) %H:%M:%S")),
            bg_color: bg_color.to_owned(),
            fg_color: String::from(fg_color),
            icon: String::from(icon),
            kind: String::from("clock"),
            alignment: alignment.clone(),
        }
    }
);
