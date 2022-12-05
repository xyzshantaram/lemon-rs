use std::process::Command;

pub fn get_stdout(mut cmd: Command) -> String {
    let oup = cmd.output();
    let mut res = String::from("ERROR");

    if let Ok(val) = oup {
        if val.status.success() {
            res = String::from_utf8(val.stdout)
                .unwrap_or_else(|_| String::from("ERROR"))
                .trim()
                .to_string();
        }
    };

    res
}

#[macro_export]
macro_rules! color {
    ($hex: expr) => {
        format!("#{}", $hex)
    };
}
