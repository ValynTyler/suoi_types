use crate::{Deg, Matrix, Quaternion};
use crate::Matrix4;
use crate::Vector3;

pub struct Transform {
    position: Vector3,
    rotation: Quaternion,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Default::default(),
            rotation: Quaternion::axis_angle(Vector3::up(), Deg(0.0)),
        }
    }
}

impl Into<Matrix4> for Transform {
    fn into(self) -> Matrix4 {
        self.mat()
    }
}

impl Transform {
    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn rotation(&self) -> Quaternion {
        self.rotation
    }

    pub fn forward(&self) -> Vector3 {
        let mat = self.rotation.mat();
        Vector3 {
            x: mat.get(0, 2).unwrap(),
            y: mat.get(1, 2).unwrap(),
            z: mat.get(2, 2).unwrap(),
        }
    }

    pub fn right(&self) -> Vector3 {
        let mat = self.rotation.mat();
        Vector3 {
            x: mat.get(0, 0).unwrap(),
            y: mat.get(1, 0).unwrap(),
            z: mat.get(2, 0).unwrap(),
        }
    }

    pub fn up(&self) -> Vector3 {
        let mat = self.rotation.mat();
        Vector3 {
            x: mat.get(0, 1).unwrap(),
            y: mat.get(1, 1).unwrap(),
            z: mat.get(2, 1).unwrap(),
        }
    }

    pub fn mat(&self) -> Matrix4 {
        &Matrix4::translate(self.position) * &self.rotation.mat()
    }

    pub fn translate(&mut self, translation: Vector3) {
        self.position += translation;
    }
    
    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.rotation = rotation;
    }
}
