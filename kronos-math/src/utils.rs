use crate::vec2::Vec2;

pub trait Pointable2D<T> where T: num_traits::Num {

    fn points(&self) -> (T,T);

}

impl Pointable2D<f64> for Vec2 {
    fn points(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl Pointable2D<f32> for Vec2 {
    fn points(&self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}