use crate::box2d::Box2d;
use crate::vec2::Vec2;

pub struct View {
    center:Vec2,
    width:usize,
    height:usize
}
impl View {

    pub fn new_view(center:Vec2,width:usize,height:usize)-> Self {
        View {center,width,height}
    }

    pub fn get_bounds(&self) -> Box2d{


        Box2d::new(self.center.x  -(  self.width / 2) as f64, self.center.y -( self.height / 2) as f64, self.width as f64, self.height as f64)
    }
    pub fn center(&self) -> &Vec2 {
        &self.center
    }

    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
    }

    pub fn move_by(&mut self, offset: Vec2) {
        self.center.x += offset.x;
        self.center.y += offset.y;
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.center.x = x;
        self.center.y = y;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.set_size(width, height);
    }

    pub fn half_width(&self) -> f64 {
        self.width as f64 / 2.0
    }

    pub fn half_height(&self) -> f64 {
        self.height as f64 / 2.0
    }

    pub fn left(&self) -> f64 {
        self.center.x - self.half_width()
    }

    pub fn right(&self) -> f64 {
        self.center.x + self.half_width()
    }

    pub fn bottom(&self) -> f64 {
        self.center.y - self.half_height()
    }

    pub fn top(&self) -> f64 {
        self.center.y + self.half_height()
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y >= self.bottom()
            && point.y <= self.top()
    }

    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.left()
            && x <= self.right()
            && y >= self.bottom()
            && y <= self.top()
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn world_to_view(&self, point: &Vec2) -> Vec2 {
        Vec2::new(
            point.x - self.left(),
            point.y - self.bottom(),
        )
    }

    pub fn view_to_world(&self, point: &Vec2) -> Vec2 {
        Vec2::new(
            point.x + self.left(),
            point.y + self.bottom(),
        )
    }

}

pub fn transform_point(view:&View,point: &Vec2) -> Vec2{
    view.view_to_world(point)
}

