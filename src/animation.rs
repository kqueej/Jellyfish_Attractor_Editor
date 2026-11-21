pub struct AnimationController {
    visible_vertices: f64,
    total_vertices: u32,
    paused: bool,
    speed: f64,
}

impl AnimationController {
    pub fn new(total_vertices: u32) -> Self {
        Self {
            visible_vertices: 2.0_f64.min(total_vertices as f64),
            total_vertices,
            paused: false,
            speed: 5000.0,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if !self.paused {
            self.visible_vertices =
                (self.visible_vertices + delta_time * self.speed).min(self.total_vertices as f64);
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn reset(&mut self) {
        self.visible_vertices = 2.0_f64.min(self.total_vertices as f64);
        self.paused = false;
    }

    pub fn speed_up(&mut self) {
        self.speed = (self.speed * 1.25).min(100_000.0);
    }

    pub fn slow_down(&mut self) {
        self.speed = (self.speed / 1.25).max(100.0);
    }

    pub fn visible_vertices(&self) -> u32 {
        self.visible_vertices as u32
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }
}
