use systemstat::Platform;

use crate::{define_emitter, emitter::Emitted, SYS};

define_emitter!(NetworkEmitter, "net", |alignment, _, bg_color, icon| {
    let mut res = String::new();
    let mut fg_color = String::from("EE3333");
    let mut connected_int: Option<String> = None;
    let mut icon = icon;

    if online::check(None).is_ok() {
        fg_color = String::from("#809847");
        if let Ok(networks) = SYS.networks() {
            for (interface, _) in networks {
                if vec!["eth0", "wlan0", "enp2s0", "wlp3s0"].contains(&interface.as_str()) {
                    if let Ok(stats) = SYS.network_stats(&interface) {
                        if stats.rx_bytes > systemstat::ByteSize(0) {
                            connected_int = Some(interface.clone());
                        };
                    }
                }

                if connected_int.is_some() {
                    let ci_tmp = connected_int.clone().unwrap();
                    let ci_str = ci_tmp.as_str();
                    match ci_str {
                        "wlan0" | "wlp3s0" => {
                            icon = "\u{f1eb}";
                            let contents = std::fs::read_to_string("/proc/net/wireless");
                            if let Ok(contents) = contents {
                                let mut split = contents.split('\n');
                                let ln = split.nth(2);
                                if let Some(val) = ln {
                                    let mut cleaned = val.replace(':', "");
                                    cleaned = cleaned.replace('.', "");
                                    let mut parts = cleaned.split(' ').filter(|e| !e.is_empty());
                                    if let Some(i) = parts.nth(2) {
                                        let quality =
                                            (str::parse::<f32>(i).unwrap() / 70.0) * 100.0;
                                        res += format!("{:.0}%", quality).as_str();
                                    }
                                }
                            } else {
                                res += "ERROR";
                            }
                        }
                        "eth0" | "enp2s0" => {
                            icon = "\u{f6ff}";
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
});
