pub mod clock_emitter;
pub mod emitter;
pub mod media_emitter;
pub mod title_emitter;
pub mod util;

use crate::{emitter::Emitted, media_emitter::MediaEmitter};
use std::collections::HashMap;

use emitter::Alignment;
use futures::{stream::select_all, StreamExt};

use clock_emitter::ClockEmitter;
use title_emitter::TitleEmitter;

use crate::emitter::Emitter;

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
    ]);

    let order = vec!["title", "clock", "media"];
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

        println!("{}", oups.join(" "));
    }
}
