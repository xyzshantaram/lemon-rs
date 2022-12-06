use systemstat::Platform;

use crate::{color, define_emitter, Emitted, SYS};

define_emitter!(MemoryEmitter, "mem", |alignment, _, bg_color, icon| {
    let mut fg_color = String::new();
    let mut content = String::new();

    match SYS.memory() {
        Ok(mem) => {
            let total = mem.total.0 as f64;
            let free = mem.free.0 as f64;
            content += &format!("{:.1}G", (total - free) / 1_000_000_000.0);
            let ratio = free / total;
            if ratio > 0.7 {
                fg_color = color!("809847");
            } else if ratio > 0.5 {
                fg_color = color!("fb9f02");
            } else if ratio > 0.3 {
                fg_color = color!("cc4500");
            } else {
                fg_color = color!("e63e00");
            };
        }
        Err(_) => {
            content += "ERROR";
        }
    };

    Emitted {
        bg_color: String::from(bg_color),
        fg_color,
        content,
        icon: String::from(icon),
        kind: String::from("mem"),
        alignment: alignment.clone(),
    }
});
