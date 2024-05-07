use nerd::{matrix::{Matrix, Matrix4}, vector::Vector3};

use crate::{Deg, Quaternion};

pub struct Transform {
    position: Vector3,
    rotation: Quaternion,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::ZERO,
            rotation: Quaternion::axis_angle(Vector3::UP, Deg(0.0)),
        }
    }
}

impl Into<Matrix4> for Transform {
    fn into(self) -> Matrix4 {
        Matrix4::from_translation(self.position) * self.rotation.mat()
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
            x: mat.get(1, 3),
            y: mat.get(2, 3),
            z: mat.get(3, 3),
        }
    }

    pub fn right(&self) -> Vector3 {
        let mat = self.rotation.mat();
        Vector3 {
            x: mat.get(1, 1),
            y: mat.get(2, 1),
            z: mat.get(3, 1),
        }
    }

    pub fn up(&self) -> Vector3 {
        let mat = self.rotation.mat();
        Vector3 {
            x: mat.get(1, 2),
            y: mat.get(2, 2),
            z: mat.get(3, 2),
        }
    }

    pub fn translate(&mut self, translation: Vector3) {
        self.position += translation;
    }
    
    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.rotation = rotation;
    }
}
