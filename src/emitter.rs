use async_std::task::{self, Poll};
use chrono::Local;
use futures::Stream;

const DEFAULT_BG_COLOR: &str = "#191430";
const DEFAULT_FG_COLOR: &str = "#FFFFFF";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Right,
    Center,
    Continue,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Continue
    }
}

#[derive(Default, Clone, Debug)]
pub struct Emitted {
    pub(crate) content: String,
    pub(crate) bg_color: String,
    pub(crate) fg_color: String,
    pub(crate) icon: String,
    pub(crate) kind: String,
    pub(crate) alignment: Alignment,
}

pub type EmitFunction =
    fn(alignment: &Alignment, fg_color: &str, bg_color: &str, icon: &str) -> Emitted;

#[derive(Clone)]
pub struct Emitter {
    duration_millis: i64,
    pub(crate) fg_color: String,
    pub(crate) bg_color: String,
    pub(crate) icon: String,
    pub(crate) alignment: Alignment,
    emit: Option<EmitFunction>,
    last_call: i64,
    called_before: bool,
    pub(crate) last_out: Emitted,
}

impl Emitter {
    pub fn new(millis: i64, icon: String, alignment: Alignment) -> Emitter {
        Emitter {
            fg_color: String::from(DEFAULT_FG_COLOR),
            bg_color: String::from(DEFAULT_BG_COLOR),
            icon,
            alignment,
            duration_millis: millis,
            emit: None,
            last_call: 0,
            called_before: false,
            last_out: Emitted::default(),
        }
    }

    pub fn set_emitter(&mut self, emit: EmitFunction) {
        self.emit = Some(emit);
    }
}

impl Stream for Emitter {
    type Item = Emitted;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let now = Local::now().timestamp_millis();
        if !self.called_before || (now - self.last_call) > self.duration_millis {
            self.called_before = true;
            self.last_call = now;
            if let Some(val) = self.emit {
                self.last_out = val(
                    &self.alignment,
                    self.fg_color.as_str(),
                    self.bg_color.as_str(),
                    self.icon.as_str(),
                )
            } else {
            }
        };
        Poll::Ready(Some(self.last_out.clone()))
    }
}
