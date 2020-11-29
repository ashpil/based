use glitz::vec::Vec3;

pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Ray { o, d }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.o + t * self.d
    }
}

