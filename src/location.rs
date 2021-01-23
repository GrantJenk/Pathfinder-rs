#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn dist(start: Location, dest: Location) -> f64 {
        let x_diff = (start.x - dest.x).pow(2);
        let y_diff = (start.y - dest.y).pow(2);
        ((x_diff + y_diff) as f64).sqrt()
    }
}