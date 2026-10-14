use crate::utils::Pointable2D;
use crate::vec2::Vec2;

pub struct Box2d {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}



impl Box2d {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Box2d {x,y,width,height}
    }

    pub fn new_unit(x:f64,y:f64,size:f64) -> Self {
        Self::new(x,y,size, size)
    }
   pub fn is_inside(&self, points: Box<dyn Pointable2D<f64>>) -> bool  {

       let p = points.points();

        p.0 > self.x && p.0 < self.width && p.1 > self.y && p.1 < self.height
}

    pub fn intersects(&self,other: &Box2d) -> bool {
       (self.x > other.x && self.x < other.x + self.width ) && (self.y > other.y && self.y < other.y + self.height ) || (self.width + self.x < other.width + other.x && self.height + self.y < other.height + other.y && self.width + self.x < other.x && self.height + self.x > other.y)
    }
    pub fn is_inside_entirely(&self, p0: &Box2d) -> bool {

        if self.area() < p0.area()
        {
            return false;
        }
        let tl_inside = self.x < p0.x && p0.x < self.x + self.width && self.y < p0.y && p0.y < self.y + self.height;
        let tr_inside = self.x < p0.left() && p0.left() < self.x + self.width && self.y < p0.y && p0.y < self.y + self.height;

        let bl_inside = self.x < p0.x && p0.x < self.x + self.width && self.y < p0.bottom() && p0.y < self.bottom() + self.height;
        let br_inside = self.x < p0.left() && p0.left() < self.x + self.width && self.y < p0.bottom() && p0.bottom() < self.y + self.height;

        tl_inside && tr_inside && bl_inside && br_inside

    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        (self.width * 2.0) + (self.height * 2.0)

    }
    pub fn left(&self) -> f64 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }
    pub fn half_width(&self) -> f64 {
        self.width / 2.0
    }

    pub fn half_height(&self) -> f64 {
        self.height / 2.0
    }

    pub fn half_width_pos(&self) -> f64 {
        self.half_width() + self.x
    }
    pub fn half_height_pos(&self) -> f64 {
        self.half_height() + self.y
    }

    pub fn split(&self) -> Vec<Box2d> {

        vec![Box2d::new(self.x,self.y,self.half_width(),self.half_height()),Box2d::new(self.half_width_pos(),self.y,self.half_width(),self.half_height()),
        Self::new(self.x,self.half_height_pos(),self.half_width(),self.half_height()),
             Self::new(self.half_width_pos(),self.half_height_pos(),self.half_width(),self.half_height())

        ]

    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.half_width_pos(),self.half_height_pos())
    }

    pub fn distance_to(&self,other:&Box2d) -> f32 {

         self.center().distance(&other.center()) as f32

    }

}