use fyrox::core::algebra::{Quaternion, UnitQuaternion, Vector2, Vector3};
use lite_macro::lite_api;

/// Plain internal representation of 3D vector. In scripts, it's represented by another type with reach functionality.
#[lite_api(class=Vector3)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PodVector3 {
    /// X component
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[lite_api(class=Vector2)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PodVector2 {
    pub x: f32,
    pub y: f32,
}

#[lite_api(class=Vector2I)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PodVector2I {
    pub x: i32,
    pub y: i32,
}

impl From<Vector3<f32>> for PodVector3 {
    fn from(v: Vector3<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<PodVector3> for Vector3<f32> {
    fn from(v: PodVector3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Vector2<f32>> for PodVector2 {
    fn from(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<PodVector2> for Vector2<f32> {
    fn from(v: PodVector2) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<Vector2<i32>> for PodVector2I {
    fn from(v: Vector2<i32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<PodVector2I> for Vector2<i32> {
    fn from(v: PodVector2I) -> Self {
        Self::new(v.x, v.y)
    }
}

#[lite_api(class=Quaternion)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PodQuaternion {
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub w: f32,
}

impl From<UnitQuaternion<f32>> for PodQuaternion {
    fn from(v: UnitQuaternion<f32>) -> Self {
        Self {
            i: v.i,
            j: v.j,
            k: v.k,
            w: v.w,
        }
    }
}

impl From<PodQuaternion> for UnitQuaternion<f32> {
    fn from(v: PodQuaternion) -> Self {
        UnitQuaternion::from_quaternion(Quaternion::new(v.w, v.i, v.j, v.k))
    }
}
