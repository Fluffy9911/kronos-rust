
pub struct Vec2 {

    pub x: f64,
    pub y: f64,

}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn zero() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    pub fn add(&mut self, other: &Vec2)  {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn subtract(&mut self, other: &Vec2)  {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn square(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn dot(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn distance(&self, other: &Vec2) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    pub fn mul(&mut self,scalar: f64){
        self.x *= scalar;
        self.y *= scalar;
    }



}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
       Self::new(self.x + other.x, self.y + other.y)

    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, scalar: f64) -> Vec2 {
    Self::new(self.x / scalar, self.y / scalar)}
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Self::new(-self.x, -self.y)
    }
}

impl std::ops::BitXor<f64> for Vec2 {


    type Output = Vec2;
    fn bitxor(self, scalar: f64) -> Vec2 {
        Self::new(self.x.powf(scalar), self.y.powf(scalar))
    }
}