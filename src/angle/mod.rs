#[derive(Debug, Clone, Copy)]
pub struct Rad(pub f32);

#[derive(Debug, Clone, Copy)]
pub struct Deg(pub f32);

impl Into<Deg> for Rad {
    fn into(self) -> Deg {
        Deg(self.0.to_degrees())
    }
}

impl Into<Rad> for Deg {
    fn into(self) -> Rad {
        Rad(self.0.to_radians())
    }
}

impl Into<f32> for Rad {
    fn into(self) -> f32 {
        self.0
    }
}

impl Into<f32> for Deg {
    fn into(self) -> f32 {
        self.0
    }
}

pub trait Angle: Copy {
    fn deg(self) -> Deg;
    fn rad(self) -> Rad;
}

impl Angle for Rad {
    fn deg(self) -> Deg {
        self.into()
    }

    fn rad(self) -> Rad {
        self
    }
}

impl Angle for Deg {
    fn deg(self) -> Deg {
        self
    }

    fn rad(self) -> Rad {
        self.into()
    }
}
