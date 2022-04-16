use std::time::{Duration, Instant};

pub struct FPSManager {
    last_time: Instant,
    current_time: Instant,
    frame_count: u32,
    fps: u32,
}

impl FPSManager {
    pub fn new() -> FPSManager {
        let fps_manager = FPSManager {
            last_time: Instant::now(),
            current_time: Instant::now(),
            frame_count: 0,
            fps: 0,
        };

        return fps_manager;
    }

    pub fn get_fps(&mut self) -> u32 {
        self.current_time = Instant::now();
        self.frame_count += 1;

        if self.current_time.duration_since(self.last_time).as_millis()
            > Duration::new(1, 0).as_millis()
        {
            self.last_time = self.current_time;
            self.fps = self.frame_count;
            self.frame_count = 0;
        }

        return self.fps;
    }
}
