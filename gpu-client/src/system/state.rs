#[derive(Debug, Default, Copy, Clone)]
pub struct SystemState {
    /// Duration of project run in seconds
    pub time_elapsed: f64,
    /// Duration between previous and current frame in seconds
    pub delta: f64,
    /// Number of frames rendered this run
    pub frame_num: u64,
    /// Whether run is paused
    _paused: bool,
    /// Nanosecond timestamp of the last frame rendered, None if no frames have been rendered yet
    last_frame: Option<i64>,
    /// Nanosecond timestamp of the last unpause, None if the run has yet to be unpaused
    _last_unpause: Option<i64>,
}

impl SystemState {
    pub fn tick(&mut self) {
        let now = chrono::Utc::now().timestamp_micros();
        let delta = match self.last_frame {
            Some(timestamp) => now - timestamp,
            None => 0,
        } as f64
            / 1_000_000_f64;

        self.time_elapsed += delta;
        self.delta = delta;
        self.frame_num += 1;
        self.last_frame = Some(now);

        gpu_log::debug!(
            "Ticking! [time]={}s -- delta={}ms -- frame={}",
            self.time_elapsed,
            delta * 1000.,
            self.frame_num
        );
    }
}
