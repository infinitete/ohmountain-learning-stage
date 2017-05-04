fn main() {
    let c = CircleBuilder::new()
        .x(1.4)
        .y(1.5)
        .radius(1.4)
        .finalize();

    println!("{}", c.area());
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

struct CircleBuilder {
    x: f32,
    y: f32,
    radius: f32,
}

/// Builder
impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0f32,
            y: 0f32,
            radius: 1f32,
        }
    }

    fn x(&mut self, x: f32) -> &mut CircleBuilder {
        self.x = x;

        self
    }

    fn y(&mut self, y: f32) -> &mut CircleBuilder {
        self.y = y;

        self
    }

    fn radius(&mut self, radius: f32) -> &mut CircleBuilder {
        self.radius = radius;

        self
    }

    fn finalize(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius
        }
    }
}

impl Circle {
    fn area(&self) -> f64 {
        (self.x as f64) * (self.y as f64) * std::f64::consts::PI
    }
}
