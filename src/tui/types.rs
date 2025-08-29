#[derive(Clone, Copy)]
pub enum KeyStatus {
    Quit,
    TogglePlay,
    VolumeIncrease,
    VolumeDecrease,
    ProgressIncrease,
    ProgressDecrease,
    PickerNext,
    PickerPrev,
    SwitchMode,
    NextTrack,
    PrevTrack,
    PlaySelected,
    NoOp,
}
impl Default for KeyStatus {
    fn default() -> Self {
        Self::NoOp
    }
}
#[derive(Clone, Debug)]
pub enum PlayStatus {
    Playing,
    Stopped,
    Paused,
}
#[derive(Clone, Debug)]
pub enum PlaybackMode {
    Sequential,
    LoopAll,
    LoopOne,
    Shuffle,
}
impl PlaybackMode {
    pub fn next(&self) -> Self {
        match self {
            PlaybackMode::Sequential => PlaybackMode::LoopAll,
            PlaybackMode::LoopAll => PlaybackMode::LoopOne,
            PlaybackMode::LoopOne => PlaybackMode::Shuffle,
            PlaybackMode::Shuffle => PlaybackMode::Sequential,
        }
    }
}
