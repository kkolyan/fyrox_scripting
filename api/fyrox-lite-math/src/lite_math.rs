use std::fmt::{Debug, Display};

use fyrox::core::{
    algebra::{Unit, UnitQuaternion, Vector3},
    num_traits::Zero,
};
use fyrox_lite::lite_math::{PodQuaternion, PodVector2, PodVector2I, PodVector3};
use lite_macro::lite_api;

use fyrox::core::algebra::Vector2;
use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct LiteQuaternion(UnitQuaternion<f32>);

#[lite_api(class=Quaternion)]
impl LiteQuaternion {
    pub fn face_towards(dir: LiteVector3, up: LiteVector3) -> LiteQuaternion {
        LiteQuaternion(UnitQuaternion::face_towards(&dir.into(), &up.into()))
    }
    pub fn from_axis_angle(axis: LiteVector3, angle: f32) -> LiteQuaternion {
        LiteQuaternion(UnitQuaternion::from_axis_angle(
            &Unit::new_normalize(Vector3::from(axis)),
            angle,
        ))
    }

    #[allow(non_snake_case)]
    pub fn mul_vec(&self, o: LiteVector3) -> LiteVector3 {
        LiteVector3(self.0.mul(&o.0))
    }

    #[allow(non_snake_case)]
    pub fn mul_quat(&self, rot_delta: LiteQuaternion) -> LiteQuaternion {
        LiteQuaternion(self.0.mul(&rot_delta.0))
    }
}

impl Debug for LiteQuaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl From<LiteQuaternion> for UnitQuaternion<f32> {
    fn from(value: LiteQuaternion) -> Self {
        value.0
    }
}

impl From<UnitQuaternion<f32>> for LiteQuaternion {
    fn from(value: UnitQuaternion<f32>) -> Self {
        LiteQuaternion(value)
    }
}

impl From<LiteQuaternion> for PodQuaternion {
    fn from(v: LiteQuaternion) -> Self {
        UnitQuaternion::<f32>::from(v).into()
    }
}

impl From<PodQuaternion> for LiteQuaternion {
    fn from(v: PodQuaternion) -> Self {
        UnitQuaternion::<f32>::from(v).into()
    }
}

#[derive(Clone, Copy)]
pub struct LiteVector3(pub Vector3<f32>);

impl Debug for LiteVector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for LiteVector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(non_snake_case)]
#[lite_api(class=Vector3)]
impl LiteVector3 {
    #[rustfmt::skip]    pub fn get_x(&self) -> f32 { self.0.x }
    #[rustfmt::skip]    pub fn get_y(&self) -> f32 { self.0.y }
    #[rustfmt::skip]    pub fn get_z(&self) -> f32 { self.0.z }
    #[rustfmt::skip]    pub fn set_x(&mut self, value: f32) { self.0.x = value; }
    #[rustfmt::skip]    pub fn set_y(&mut self, value: f32) { self.0.y = value; }
    #[rustfmt::skip]    pub fn set_z(&mut self, value: f32) { self.0.z = value; }

    #[rustfmt::skip]    pub fn get_X() -> LiteVector3 { Vector3::x_axis().into_inner().into() }
    #[rustfmt::skip]    pub fn get_Y() -> LiteVector3 { Vector3::y_axis().into_inner().into() }
    #[rustfmt::skip]    pub fn get_Z() -> LiteVector3 { Vector3::z_axis().into_inner().into() }

    pub fn get_ZERO() -> LiteVector3 {
        Vector3::zero().into()
    }
    pub fn new(x: f32, y: f32, z: f32) -> LiteVector3 {
        LiteVector3(Vector3::new(x, y, z))
    }
    pub fn mul(&self, o: f32) -> LiteVector3 {
        LiteVector3(self.0 * o)
    }

    pub fn add(&self, o: LiteVector3) -> LiteVector3 {
        LiteVector3(self.0 + o.0)
    }

    pub fn normalize(&self) -> LiteVector3 {
        LiteVector3(self.0.normalize())
    }

    pub fn sub(&self, o: LiteVector3) -> LiteVector3 {
        LiteVector3(self.0 - o.0)
    }

    pub fn magnitude(&self) -> f32 {
        self.0.magnitude()
    }

    pub fn normalize_inplace(&mut self) {
        self.0.normalize_mut();
    }
}

impl From<Vector3<f32>> for LiteVector3 {
    fn from(value: Vector3<f32>) -> Self {
        LiteVector3(value)
    }
}

impl From<LiteVector3> for Vector3<f32> {
    fn from(value: LiteVector3) -> Self {
        value.0
    }
}

impl From<LiteVector3> for PodVector3 {
    fn from(v: LiteVector3) -> Self {
        Vector3::<f32>::from(v).into()
    }
}

impl From<PodVector3> for LiteVector3 {
    fn from(v: PodVector3) -> Self {
        Vector3::<f32>::from(v).into()
    }
}

#[derive(Clone, Copy)]
pub struct LiteVector2(pub Vector2<f32>);

impl Debug for LiteVector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for LiteVector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(non_snake_case)]
#[lite_api(class=Vector2)]
impl LiteVector2 {
    #[rustfmt::skip]    pub fn get_x(&self) -> f32 { self.0.x }
    #[rustfmt::skip]    pub fn get_y(&self) -> f32 { self.0.y }
    #[rustfmt::skip]    pub fn set_x(&mut self, value: f32) { self.0.x = value; }
    #[rustfmt::skip]    pub fn set_y(&mut self, value: f32) { self.0.y = value; }

    pub fn get_ZERO() -> LiteVector2 {
        Vector2::zero().into()
    }
    pub fn new(x: f32, y: f32) -> LiteVector2 {
        LiteVector2(Vector2::new(x, y))
    }
    pub fn mul(&self, o: f32) -> LiteVector2 {
        LiteVector2(self.0 * o)
    }

    pub fn add(&self, o: LiteVector2) -> LiteVector2 {
        LiteVector2(self.0 + o.0)
    }

    pub fn normalize(&self) -> LiteVector2 {
        LiteVector2(self.0.normalize())
    }

    pub fn sub(&self, o: LiteVector2) -> LiteVector2 {
        LiteVector2(self.0 - o.0)
    }

    pub fn magnitude(&self) -> f32 {
        self.0.magnitude()
    }

    pub fn normalize_inplace(&mut self) {
        self.0.normalize_mut();
    }
}

impl From<Vector2<f32>> for LiteVector2 {
    fn from(value: Vector2<f32>) -> Self {
        LiteVector2(value)
    }
}

impl From<LiteVector2> for Vector2<f32> {
    fn from(value: LiteVector2) -> Self {
        value.0
    }
}

impl From<LiteVector2> for PodVector2 {
    fn from(v: LiteVector2) -> Self {
        Vector2::<f32>::from(v).into()
    }
}

impl From<PodVector2> for LiteVector2 {
    fn from(v: PodVector2) -> Self {
        Vector2::<f32>::from(v).into()
    }
}

#[derive(Clone, Copy)]
pub struct LiteVector2I(pub Vector2<i32>);

impl Debug for LiteVector2I {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for LiteVector2I {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(non_snake_case)]
#[lite_api(class=Vector2I)]
impl LiteVector2I {
    #[rustfmt::skip]    pub fn get_x(&self) -> i32 { self.0.x }
    #[rustfmt::skip]    pub fn get_y(&self) -> i32 { self.0.y }
    #[rustfmt::skip]    pub fn set_x(&mut self, value: i32) { self.0.x = value; }
    #[rustfmt::skip]    pub fn set_y(&mut self, value: i32) { self.0.y = value; }

    pub fn get_ZERO() -> LiteVector2I {
        Vector2::zero().into()
    }
    pub fn new(x: i32, y: i32) -> LiteVector2I {
        LiteVector2I(Vector2::new(x, y))
    }
    pub fn mul(&self, o: i32) -> LiteVector2I {
        LiteVector2I(self.0 * o)
    }

    pub fn add(&self, o: LiteVector2I) -> LiteVector2I {
        LiteVector2I(self.0 + o.0)
    }

    pub fn sub(&self, o: LiteVector2I) -> LiteVector2I {
        LiteVector2I(self.0 - o.0)
    }
}

impl From<Vector2<i32>> for LiteVector2I {
    fn from(value: Vector2<i32>) -> Self {
        LiteVector2I(value)
    }
}

impl From<LiteVector2I> for Vector2<i32> {
    fn from(value: LiteVector2I) -> Self {
        value.0
    }
}

impl From<LiteVector2I> for PodVector2I {
    fn from(v: LiteVector2I) -> Self {
        Vector2::<i32>::from(v).into()
    }
}

impl From<PodVector2I> for LiteVector2I {
    fn from(v: PodVector2I) -> Self {
        Vector2::<i32>::from(v).into()
    }
}
