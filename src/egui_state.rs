pub struct EguiState {
    pub clear_color: vek::Rgba<f32>,
    pub load_progress: f32,
}
impl EguiState {
    pub fn new() -> Self {
        Self {
            clear_color: vek::Rgba {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
            load_progress: 0.0,
        }
    }
}
