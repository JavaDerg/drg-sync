use std::time::Duration;
use tokio::time::{Instant, Interval, interval};

pub struct Controller {
    ticker: Interval,

    start: Instant,
    offset: Duration,

    playing: bool,

    dirty: bool,
}

#[derive(Clone, serde::Serialize)]
pub enum PlayerEvent {
    Fix {
        playing: bool,
        // when playing the player should try to skip to withing <0.1 of this position,
        // if not playing it should skip to the precise position
        position: f64,
    },
    // when playing the player should try to skip to withing <0.1 of this position,
    Tick(f64),
}

impl Controller {
    pub fn new() -> Self {
        Self {
            ticker: interval(Duration::from_secs(1)),

            start: Instant::now(),
            offset: Default::default(),

            playing: false,

            dirty: false,
        }
    }

    pub async fn next_update(&mut self) -> PlayerEvent {
        if self.dirty {
            self.dirty = false;

            let (pos, playing) = self.state();
            return PlayerEvent::Fix { playing, position: pos.as_secs_f64() };
        }

        let _ = self.ticker.tick().await;

        let (pos, _) = self.state();
        PlayerEvent::Tick(pos.as_secs_f64())
    }

    pub fn state(&self) -> (Duration, bool) {
        let dur = self
            .playing
            .then(|| self.start.elapsed() + self.offset)
            .unwrap_or(self.offset);

        (dur, self.playing)
    }

    pub fn play(&mut self) {
        if self.playing {
            return;
        }

        self.playing = true;
        self.start = Instant::now();

        self.dirty = true;
    }

    pub fn pause(&mut self) {
        if !self.playing {
            return;
        }

        self.playing = false;
        self.offset += self.start.elapsed();

        self.dirty = true;
    }

    pub fn reset(&mut self) {
        self.playing = false;
        self.set(Duration::ZERO);
    }

    pub fn set(&mut self, dur: Duration) {
        self.offset = dur;

        self.dirty = true;
    }

}
