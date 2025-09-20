#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

impl Intersect {
    pub fn new(distance: f32, impact: char) -> Self {
        Intersect { distance, impact }
    }
}
