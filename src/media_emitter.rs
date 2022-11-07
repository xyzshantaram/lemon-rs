use mpris::PlayerFinder;

use crate::{define_emitter, emitter::Emitted};

const MEDIA_MAX_LEN: usize = 20 + MEDIA_SHORTENER.len();
const MEDIA_SHORTENER: &str = "…";

define_emitter!(
    MediaEmitter,
    "media",
    |alignment: &Alignment, fg_color: &str, bg_color: &str, icon: &str| {
        let mut oup = "DBus connection error".to_owned();
        let mut icon = String::from(icon);
        let mut fg_color = String::from(fg_color);

        if let Ok(finder) = PlayerFinder::new() {
            oup = "Nothing playing".to_owned();
            if let Ok(active) = finder.find_active() {
                let status = active
                    .get_playback_status()
                    .unwrap_or(mpris::PlaybackStatus::Stopped);

                match status {
                    mpris::PlaybackStatus::Playing => {
                        icon = String::from("\u{f04b}");
                        fg_color = String::from("#007cdf");
                    }
                    mpris::PlaybackStatus::Paused => {
                        icon = String::from("\u{f04c}");
                        fg_color = String::from("#b18bde");
                    }
                    mpris::PlaybackStatus::Stopped => {
                        icon = String::from("\u{f04d}");
                        fg_color = String::from("#555555");
                    }
                };
                oup = active
                    .get_metadata()
                    .map(|v| v.title().unwrap_or_default().to_owned())
                    .unwrap_or_else(|_| "Error getting metadata".to_owned());
            }
        }

        if oup.len() > MEDIA_MAX_LEN {
            oup.truncate(MEDIA_MAX_LEN);
            oup += MEDIA_SHORTENER;
        }
        Emitted {
            content: oup,
            bg_color: String::from(bg_color),
            fg_color,
            icon,
            kind: String::from("media"),
            alignment: alignment.clone(),
        }
    }
);
