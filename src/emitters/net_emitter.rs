use systemstat::Platform;

use crate::{color, define_emitter, emitter::Emitted, SYS};

fn is_supported_interface(interface: &str) -> bool {
    vec!["eth0", "wlan0", "enp2s0", "wlp3s0"].contains(&interface)
}

fn get_quality() -> Result<f32, &'static str> {
    let contents = std::fs::read_to_string("/proc/net/wireless").map_err(|_| "ERROR")?;
    let mut split = contents.split('\n');
    let cleaned_line = split
        .nth(2)
        .ok_or("ERROR")?
        .replace(':', "")
        .replace('.', "");
    let mut parts = cleaned_line.split(' ').filter(|e| !e.is_empty());
    Ok((str::parse::<f32>(parts.nth(2).ok_or("ERROR")?).unwrap() / 70.0) * 100.0)
}

fn is_if_valid(interface: &str) -> Option<String> {
    is_supported_interface(interface).then(|| {
        let stats = SYS.network_stats(interface).ok()?;
        (stats.rx_bytes > systemstat::ByteSize(0)).then_some(String::from(interface))
    })?
}

define_emitter!(
    NetworkEmitter,
    "net",
    |alignment, _, bg_color, icon| {
        let mut res = String::new();
        let mut fg_color = color!("EE3333");
        let mut icon = icon;

        if online::check(None).is_ok() {
            fg_color = color!("809847");
            if let Ok(networks) = SYS.networks() {
                for (interface, _) in networks {
                    if let Some(connected) = is_if_valid(&interface) {
                        match connected.as_str() {
                            "wlan0" | "wlp3s0" => {
                                icon = "\u{f1eb}";
                                if let Ok(quality) = get_quality() {
                                    res += format!("{:.0}%", quality).as_str();
                                } else {
                                    res += "ERROR";
                                }
                                break;
                            }
                            "eth0" | "enp2s0" => {
                                icon = "\u{f6ff}";
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            } else {
                res += "ERROR";
            }
        } else {
            res += "\u{f00d}";
        }

        Emitted {
            bg_color: String::from(bg_color),
            icon: String::from(icon),
            fg_color,
            content: res,
            kind: String::from("net"),
            alignment: alignment.clone(),
        }
    },
    1000,
    String::from("\u{f0ac}"),
    Alignment::Continue,
    {}
);
