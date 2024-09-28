use napi_derive::napi;

use crate::vec3f::Vector3f;

#[napi(object)]
pub struct HitResult {
	pub distance: f64,
    pub position: Vector3f,
}