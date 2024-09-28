use napi::bindgen_prelude::FromNapiValue;
use napi_derive::napi;


#[derive(PartialEq)]
#[napi]
pub enum Axis {
	X,
    Y,
    Z
}

#[napi(js_name = "Vector3f")]
#[derive(Clone, Debug)]
pub struct Vector3f {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

#[napi]
impl Vector3f {

	#[napi(constructor)]	
	pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3f { x, y, z }
    }

	#[napi]
	pub fn dot(&self, other: &Vector3f) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

	#[napi]
	pub fn length(&self) -> f64 {
		self.dot(self).sqrt()
	}

	#[napi]
	pub fn square_length(&self) -> f64 {
		self.dot(self)
	}

	#[napi]
	pub fn normalize(&self) -> Self {
        let length = self.length();
        Vector3f::new(self.x / length, self.y / length, self.z / length)
    }

	#[napi]
	pub fn lerp(&self, other: &Vector3f, t: f64) -> Vector3f {
		Vector3f::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
            self.z + (other.z - self.z) * t
        )
	}

	#[napi]
	pub fn add(&self, other: &Vector3f) -> Vector3f {
		Vector3f::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}

	#[napi]
	pub fn subtract(&self, other: &Vector3f) -> Vector3f {
        Vector3f::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

	#[napi]
	pub fn multiply(&self, scalar: f64) -> Vector3f {
        Vector3f::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

	#[napi]
	pub fn cross(&self, other: &Vector3f) -> Vector3f {
		Vector3f::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
	}

	#[napi]
	pub fn absolute(&self) -> Vector3f {
		Vector3f::new(self.x.abs(), self.y.abs(), self.z.abs())
	}

	#[napi]
	pub fn distance(&self, other: &Vector3f) -> f64 {
		let diff = self.subtract(other);

		return ((diff.x * diff.x) + (diff.y * diff.y) + (diff.z * diff.z)).sqrt()
	}

	#[napi]
	pub fn floor(&self) -> Vector3f {
		Vector3f::new(self.x.floor(), self.y.floor(), self.z.floor())
	}

	#[napi]
	pub fn round(&self) -> Vector3f {
		Vector3f::new(self.x.round(), self.y.round(), self.z.round())
	}

	#[napi]
	pub fn ceil(&self) -> Vector3f {
		Vector3f::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
	}

	#[napi]
	pub fn slerp(&self, other: &Vector3f, t: f64) -> Vector3f {
		let dot: f64 = self.dot(other);
		let theta: f64 = dot.acos();
		let sin_theta: f64 = theta.sin();
		let a: f64 = ((1.0 - t) * theta).sin() / sin_theta;
		let b: f64 = (t * theta).sin() / sin_theta;

		return self.multiply(a).add(&other.multiply(b))
	}

	#[napi]
	pub fn equals(&self, other: &Vector3f) -> bool {
		self.x == other.x && self.y == other.y && self.z == other.z
	}

	#[napi]
	pub fn axis(&self, axis: Axis) -> f64 {
		match axis {
			Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z
		}
	}
}


impl FromNapiValue for Vector3f {
	unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
		println!("{:?}", f64::from_napi_value(env, napi_val));
		Ok(Vector3f::new(f64::from_napi_value(env, napi_val)?, 0.0, 0.0)) // Assuming x, y, and z are always 0.0 for now
	}
}

