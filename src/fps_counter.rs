use std::time::{Duration, Instant};

pub struct FPSCounter {
    updated_at: Instant,
    refresh_rate: Duration,
    render_time: u64,
    frames_rendered: u64,
    last_fps: u64,
}

impl FPSCounter {
    pub fn new(refresh_rate: Duration) -> FPSCounter {
        FPSCounter {
            updated_at: Instant::now(),
            refresh_rate: refresh_rate,
            render_time: 0,
            frames_rendered: 0,
            last_fps: 0,
        }
    }
    pub fn end_frame(&mut self) {
        self.frames_rendered += 1;
        let elapsed = Instant::now().duration_since(self.updated_at);
        if elapsed > self.refresh_rate {
            self.updated_at = Instant::now();
            self.render_time = elapsed.as_nanos() as u64 / self.frames_rendered;
            self.last_fps = 1_000_000_000 as u64 / self.render_time;
            self.frames_rendered = 0;
        }
    }
    pub fn current_fps(&self) -> u64 {
        self.last_fps
    }
    pub fn average_render_time(&self) -> u64 {
        self.render_time
    }
}
