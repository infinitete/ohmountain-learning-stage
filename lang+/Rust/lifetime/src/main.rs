fn main() {

    let p1 = Pointer {
        x: &(1.0),
        y: &(1.0),
    };

    let p2 = Pointer {
        x: &(1.5),
        y: &(-4.3),
    };

    let distance = p2.distance(&p1);

    println!("{:?}, {:?}, distance: {}", p1, p2, distance);

}

#[derive(Debug)]
struct Pointer<'a> {
    x: &'a f32,
    y: &'a f32,
}

trait Distance<'a> {
    fn distance(&self, p1: &'a Pointer) -> f32;
}

impl<'a> Pointer<'a> {
    fn distance(&self, p1: &'a Pointer) -> f32 {
        let m: f32 = (self.x - p1.x) * (self.x - p1.x) + (self.y - p1.y) * (self.y - p1.y);

        m.sqrt()
    }
}
