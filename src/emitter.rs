use std::{thread, time::Duration};

use async_std::task::{self, Poll};
use chrono::Local;
use futures::Stream;

const DEFAULT_BG_COLOR: &str = "#000000";
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
}

impl Emitter {
    pub fn new(millis: i64, icon: String, alignment: Alignment) -> Self {
        Emitter {
            fg_color: String::from(DEFAULT_FG_COLOR),
            bg_color: String::from(DEFAULT_BG_COLOR),
            icon,
            alignment,
            duration_millis: millis,
            emit: None,
            last_call: 0,
            called_before: false,
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
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let now = Local::now().timestamp_millis();
        let mut oup: Emitted = Emitted::default();
        if !self.called_before || (now - self.last_call) > self.duration_millis {
            self.called_before = true;
            self.last_call = now;
            if let Some(emit_fn) = self.emit {
                oup = emit_fn(
                    &self.alignment,
                    self.fg_color.as_str(),
                    self.bg_color.as_str(),
                    self.icon.as_str(),
                );
            };
            Poll::Ready(Some(oup))
        } else {
            let time_until_next_call = self.last_call + self.duration_millis - now;
            let dur = Duration::from_millis(time_until_next_call.try_into().unwrap());
            let waker = cx.waker().clone();
            thread::spawn(move || {
                thread::sleep(dur);
                waker.wake();
            });
            Poll::Pending
        }
    }
}

#[macro_export]
macro_rules! define_emitter {
    ($name:ident, $kind:expr, $emitter:expr, $default_millis:expr, $default_icon:expr, $default_alignment:expr, $init:block) => {
        use $crate::emitter::{Alignment, Emitter};
        pub struct $name(pub Emitter);
        impl $name {
            pub fn new(millis: i64, icon: String, alignment: Alignment) -> Self {
                let mut emitter = Emitter::new(millis, icon, alignment);
                emitter.set_emitter($emitter);
                $init;
                $name(emitter)
            }

            pub fn default() -> Self {
                Self::new($default_millis, $default_icon, $default_alignment)
            }
        }
    };
}
