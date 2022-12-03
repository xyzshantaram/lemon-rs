use std::process::Command;

use crate::{define_emitter, emitter::Emitted, util};

define_emitter!(
    VolumeEmitter,
    "volume",
    |alignment: &Alignment, fg_color: &str, bg_color: &str, _: &str| {
        let home = dirs::home_dir().expect("Error: home dir couldn't be determined!");
        let script_path = home.join(".local").join("bin").join("volume");
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(script_path);
        let stdout = util::get_stdout(cmd);

        Emitted {
            fg_color: if stdout.contains('\u{f466}') {
                String::from("#EE3333")
            } else {
                String::from(fg_color)
            },
            icon: String::from(""),
            alignment: alignment.clone(),
            bg_color: String::from(bg_color),
            content: stdout,
            kind: String::from("volume"),
        }
    }
);
