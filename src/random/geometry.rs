pub mod geometry {
    const PI: f64 = 3.141592653589793;

    // 2D Figures

    pub fn rectangle_area(length: f64, width: f64) -> f64 {
        length * width
    }

    pub fn circle_area(radius: f64) -> f64 {
        PI * radius * radius
    }

    pub fn triangle_area(base: f64, height: f64) -> f64 {
        0.5 * base * height
    }

    // 3D Figures

    pub fn cube_volume(side: f64) -> f64 {
        side.powf(3.0)
    }

    pub fn sphere_volume(radius: f64) -> f64 {
        (4.0 / 3.0) * PI * radius.powf(3.0)
    }

    pub fn cylinder_volume(radius: f64, height: f64) -> f64 {
        PI * radius * radius * height
    }
}
