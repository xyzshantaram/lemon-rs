use std::process::Command;

use crate::{color, define_emitter, emitter::Emitted, util};

define_emitter!(
    VolumeEmitter,
    "volume",
    |alignment, fg_color, bg_color, _| {
        let home = dirs::home_dir().expect("Error: home dir couldn't be determined!");
        let script_path = home.join(".local").join("bin").join("volume");
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(script_path);
        let stdout = util::get_stdout(cmd);

        Emitted {
            fg_color: if stdout.contains('\u{f466}') {
                color!("EE3333")
            } else {
                String::from(fg_color)
            },
            icon: String::from(""),
            alignment: alignment.clone(),
            bg_color: String::from(bg_color),
            content: stdout,
            kind: String::from("volume"),
        }
    },
    100,
    String::from("\u{f485}"),
    Alignment::Continue,
    {}
);
