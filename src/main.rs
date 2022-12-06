pub mod emitter;
pub mod emitters;
pub mod util;

use std::collections::HashMap;

use emitter::{Alignment, Emitted};
use emitters::*;
use futures::{stream::select_all, StreamExt};
use lazy_static::lazy_static;

use systemstat::{Platform, System};

use crate::emitter::Emitter;

lazy_static! {
    pub static ref SYS: System = System::new();
}

fn out(emitted: &Emitted) -> String {
    format!(
        "{}%{{F{}}}%{{B{}}} {}{}{}",
        if emitted.alignment == Alignment::Left {
            "%{l}"
        } else if emitted.alignment == Alignment::Right {
            "%{r}"
        } else if emitted.alignment == Alignment::Center {
            "%{c}"
        } else {
            ""
        },
        emitted.fg_color,
        emitted.bg_color,
        emitted.icon,
        if emitted.icon.is_empty() { "" } else { " " },
        emitted.content
    )
}
#[async_std::main]
async fn main() {
    let emitters: HashMap<&str, Emitter> = HashMap::from([
        (
            "title",
            TitleEmitter::new(50, String::from("\u{f2d0}"), Alignment::Left).0,
        ),
        (
            "clock",
            ClockEmitter::new(100, String::from("\u{f017}"), Alignment::Center).0,
        ),
        (
            "media",
            MediaEmitter::new(1000, String::from("\u{f8cf}"), Alignment::Continue).0,
        ),
        (
            "mem",
            MemoryEmitter::new(1000, String::from("\u{f85a}"), Alignment::Right).0,
        ),
        (
            "cpu",
            CpuEmitter::new(1000, String::from("\u{f2c7}"), Alignment::Continue).0,
        ),
        (
            "volume",
            VolumeEmitter::new(100, String::from("\u{f485}"), Alignment::Continue).0,
        ),
        (
            "net",
            NetworkEmitter::new(1000, String::from("\u{f0ac}"), Alignment::Continue).0,
        ),
        (
            "pwr",
            PowerEmitter::new(100, String::from("BAT"), Alignment::Continue).0,
        ),
    ]);

    let order = vec![
        "title", "clock", "media", "mem", "cpu", "volume", "net", "pwr",
    ];
    let mut oups: Vec<String> = Vec::new();
    for _ in 0..order.len() {
        oups.push(String::new());
    }

    let mut streams: Vec<Emitter> = Vec::new();

    for emitter in &order {
        let tup = emitters.get(emitter).unwrap();
        streams.push(tup.clone());
    }

    let mut selector = select_all(streams);
    loop {
        let emitted_op = selector.next().await;
        if emitted_op.is_none() {
            continue;
        }

        let emitted = emitted_op.unwrap();
        if !order.contains(&emitted.kind.as_str()) {
            panic!("Unknown emitter kind {}", &emitted.kind);
        }

        let idx = order.iter().position(|&i| i == emitted.kind).unwrap();
        oups[idx] = out(&emitted);

        println!("{} ", oups.join(" "));
    }
}
