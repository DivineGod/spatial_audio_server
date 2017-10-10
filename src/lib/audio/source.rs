use atomic::Atomic;
use audio::Wav;
use metres::Metres;
use time_calc::Ms;

/// Items related to audio sources.
///
/// Audio sources come in two kinds:
///
/// 1. WAV - pre-rendered n-channel .wav files and
/// 2. Realtime - input from some other currently running program (e.g. MSP, Live, etc).
#[derive(Deserialize, Serialize)]
pub struct Source {
    pub kind: Kind,
    #[serde(default, with = "::serde_extra::atomic")]
    pub role: Atomic<Option<Role>>,
    /// The distance with which the channels should be spread from the source position.
    ///
    /// If the source only has one channel, `spread` is ignored.
    #[serde(default = "default_spread", with = "::serde_extra::atomic")]
    pub spread: Atomic<Metres>,
    /// The rotation of the channels around the source position in radians.
    ///
    /// If the source only has one channel, `radians` is ignored.
    #[serde(default, with = "::serde_extra::atomic")]
    pub radians: Atomic<f32>,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Role {
    Soundscape,
    Installation,
    Scribbles,
}

#[derive(Deserialize, Serialize)]
pub enum Kind {
    Wav(Wav),
    Realtime(Realtime),
}

#[derive(Deserialize, Serialize)]
pub struct Realtime {
    pub channels: usize,
    // Durationn for which the realtime input is played.
    pub duration: Ms,
    // Need some input type
}

fn default_spread() -> Atomic<Metres> {
    Atomic::new(Metres(2.5))
}
