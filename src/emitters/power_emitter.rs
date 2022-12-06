use systemstat::{Duration, Platform};

use crate::{color, define_emitter, Emitted, SYS};

struct BatteryInfo {
    on_ac: bool,
    capacity: f32,
    remaining_time: Duration,
}

fn get_battery_info() -> Result<BatteryInfo, String> {
    let life = SYS
        .battery_life()
        .map_err(|x| format!("Error getting battery life: {}", x))?;
    let on_ac = SYS
        .on_ac_power()
        .map_err(|x| format!("Error getting power status: {}", x))?;

    Ok(BatteryInfo {
        on_ac,
        capacity: life.remaining_capacity,
        remaining_time: life.remaining_time,
    })
}

define_emitter!(PowerEmitter, "pwr", |alignment, _, bg_color, _| {
    let mut fg_color = String::new();
    let mut content = String::new();
    let mut icon = String::new();

    match get_battery_info() {
        Ok(info) => {
            let perc = info.capacity * 100.0;
            let bat_secs = info.remaining_time.as_secs();
            content += &format!("{:.0}%", perc);
            if !info.on_ac {
                content += &format!(", {}:{}", bat_secs / 3600, bat_secs % 60);
            }
            if info.on_ac {
                icon = String::from("\u{f0e7}");
                fg_color = color!("809847");
            } else if perc >= 75.0 {
                icon = String::from("\u{f240}");
                fg_color = color!("809847");
            } else if perc >= 50.0 {
                icon = String::from("\u{f241}");
                fg_color = color!("fb9f02");
            } else if perc >= 25.0 {
                icon = String::from("\u{f243}");
                fg_color = color!("cc4500");
            } else {
                icon = String::from("\u{f244}");
                fg_color = color!("e63e00");
            }
        }
        Err(x) => {
            eprintln!("bat: {}", x);
        }
    };

    Emitted {
        bg_color: String::from(bg_color),
        fg_color,
        content,
        icon,
        kind: String::from("pwr"),
        alignment: alignment.clone(),
    }
});
