use crate::{color, define_emitter, Emitted, SYS};
use systemstat::Platform;

fn get_temp() -> Result<f32, String> {
    match SYS.cpu_temp() {
        Ok(val) => Ok(val),
        Err(_) => {
            // fallback to reading /sys
            let tmp = std::fs::read_to_string(
                "/sys/bus/platform/devices/thinkpad_hwmon/hwmon/hwmon3/temp1_input",
            )
            .map_err(|e| format!("Error reading fallback file: {}", e))?;
            str::parse::<f32>(tmp.trim())
                .map_err(|e| format!("Error parsing float: {}", e))
                .map(|f| f / 1000.0)
        }
    }
}

define_emitter!(CpuEmitter, "cpu", |alignment, _, bg_color, icon| {
    let mut fg_color = color!("809847");
    let mut content = String::new();

    match get_temp() {
        Ok(temp) => {
            content += &format!("{:.1}°C", temp);
            if temp >= 60.0 {
                fg_color = color!("cc4500")
            };
            if temp >= 70.0 {
                fg_color = color!("e63e00")
            };
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }

    Emitted {
        bg_color: String::from(bg_color),
        fg_color,
        content,
        icon: String::from(icon),
        kind: String::from("cpu"),
        alignment: alignment.clone(),
    }
});
