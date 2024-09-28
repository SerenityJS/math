use core::panic;

use napi_derive::napi;

use crate::vec3f::{Axis, Vector3f};

use super::hit::HitResult;

/**
 * Represents an Axis-Aligned Bounding Box (AABB) in 3D space.
 * An AABB is defined by its minimum and maximum corners.
 */
#[napi(js_name = "AABB")]
pub struct AABB {
	pub min: Vector3f, // Minimum corner of the AABB
	pub max: Vector3f, // Maximum corner of the AABB
}

struct Face {
	axis: Axis,
	value: f64
}

#[napi]
impl AABB {
	/**
	 * Constructs an AABB with specified minimum and maximum corners.
	 *
	 * @param min - The minimum corner of the AABB (Vector3f).
	 * @param max - The maximum corner of the AABB (Vector3f).
	 */
	#[napi(constructor)]
	pub fn new(min: &Vector3f, max: &Vector3f) -> AABB {
		return AABB {
			min: min.clone(),
            max: max.clone()
		}
	}

	/**
	 * Moves the AABB by a given position vector.
	 *
	 * @param position - The vector to move the AABB by (Vector3f).
	 * @return A new AABB moved by the position vector.
	 */

	#[napi]
	pub fn translate(&self, v: &Vector3f) -> AABB {
		return AABB::new(&self.min.add(v), &self.max.add(v));
	}

	/**
	 * Expands the AABB by a given vector. The expansion affects the min and max points
	 * depending on whether the expansion vector is positive or negative.
	 *
	 * @param position - The expansion vector (Vector3f).
	 * @return A new AABB expanded by the position vector.
	 */
	#[napi]
	pub fn expand(&mut self, v: &Vector3f) -> AABB {
		let mut min_clone = self.min.clone();
		let mut max_clone = self.max.clone();

		if v.x < 0.0 {
			min_clone.x += v.x;
		} else {
			max_clone.x += v.x;
		}

		if v.y < 0.0 {
            min_clone.y += v.y;
        } else {
            max_clone.y += v.y;
        }

		if v.z < 0.0 {
			min_clone.z += v.z;
		} else {
			max_clone.z += v.z;
		}

		return AABB {
			min: min_clone,
            max: max_clone
		}
	}

	
	/**
	 * Checks if a given point is inside the AABB.
	 *
	 * @param position - The point to check (Vector3f).
	 * @return True if the point is inside the AABB; otherwise, false.
	 */
	#[napi]
	pub fn contains(&self, v: &Vector3f) -> bool {
		return self.min.x <= v.x && v.x <= self.max.x &&
               self.min.y <= v.y && v.y <= self.max.y &&
               self.min.z <= v.z && v.z <= self.max.z;
	}

	#[napi]
	pub fn within(&self, v: &Vector3f) -> bool {
		if v.x < self.min.x || v.x > self.max.x { return false }
		if v.y < self.min.y || v.y > self.max.y { return false }
		return v.z >= self.min.z && v.z <= self.max.z;
	}

	#[napi]
	pub fn grow(&self, grow_scale: f64) -> AABB {
		let v = Vector3f::new(grow_scale, grow_scale, grow_scale);
        AABB::new(&self.min.subtract(&v), &self.max.add(&v))
	}

	#[napi]
	pub fn intersects(&self, aabb: &AABB) -> bool {
		const EPSILON: f64 = 1e-7;

		if aabb.max.x - self.min.x < EPSILON || self.max.x - aabb.min.x < EPSILON { return false }
		if aabb.max.y - self.min.y < EPSILON || self.max.y - aabb.min.y < EPSILON { return false }
		return aabb.max.z - self.min.z > EPSILON && self.max.z - aabb.min.z > EPSILON;
	}

	/**
	 * Determines if a given value intersects a line segment defined by two vectors
	 * along a specified axis, and returns the intersection point if it falls within
	 * the segment bounds.
	 *
	 * @param axis - The axis along which to check the intersection (Axis).
	 * @param vecA - The start of the line segment (Vector3f).
	 * @param vecB - The end of the line segment (Vector3f).
	 * @param value - The value to check for intersection.
	 * @return The intersection point as a Vector3f if it is valid; otherwise, undefined.
	 */
	#[napi]
	pub fn on_line(axis: Axis, vec_a: &Vector3f, vec_b: &Vector3f, value: f64) -> Option<Vector3f> {
		let axis_a = vec_a.axis(axis);
		let axis_b = vec_b.axis(axis);

		let f = (value - axis_a) / (axis_b - axis_a);

		if f < 0.0 || f > 1.0 { return None }
		let vector = Vector3f {
			x: if axis == Axis::X { value } else { vec_a.x + (vec_b.x - vec_a.x) * f },
			y: if axis == Axis::Y { value } else { vec_a.y + (vec_b.y - vec_a.y) * f },
			z: if axis == Axis::Z { value } else { vec_a.z + (vec_b.z - vec_a.z) * f }
	   };
	   return Some(vector);
	}

	/**
	 * Checks if a given vector is within the AABB bounds along specified axes.
	 *
	 * @param axis - An array of axes to check against (Array<Axis>).
	 * @param vec - The vector to check (Vector3f).
	 * @return True if the vector is within the bounds on the specified axes; otherwise, false.
	 */
	#[napi]
	pub fn within_axis(&self, axis: Vec<Axis>, vector: &Vector3f) -> bool {
	    if axis.len() < 2 {
			panic!("You need to provide at least two axis!");
		}

		if let [ first, second ] = &axis[..] {
			let vec_axis_a = vector.axis(*first);
			let vec_axis_b = vector.axis(*second);
	
			if vec_axis_b < self.min.axis(*second) || vec_axis_b > self.max.axis(*second) { return false }
			return vec_axis_a >= self.min.axis(*first) && vec_axis_a <= self.max.axis(*first);
		}

		return false;
	}

	/**
	 * Determines if a ray defined by a start and end vector intersects with the AABB.
	 * Returns the hit result with the intersection details.
	 *
	 * @param aabb - The AABB to check for intersection (AABB).
	 * @param start - The start point of the ray (Vector3f).
	 * @param end - The end point of the ray (Vector3f).
	 * @return A HitResult if an intersection is found; otherwise, undefined.
	 */
	#[napi(js_name = "Intercept")]
	pub fn intercept(aabb: &AABB, start: &Vector3f, end: &Vector3f) -> Option<HitResult> {
		let AABB { min, max } = aabb;
		let mut min_distance: f64 = f64::INFINITY;
        let mut hit_position: Option<Vector3f> = None;
		let faces: Vec<Face> = vec![
			Face { axis: Axis::X, value: min.x },
            Face { axis: Axis::X, value: max.x },
            Face { axis: Axis::Y, value: min.y },
            Face { axis: Axis::Y, value: max.y },
            Face { axis: Axis::Z, value: min.z },
            Face { axis: Axis::Z, value: max.z }
		];

		for face in faces {
			let vector: Option<Vector3f> = AABB::on_line(face.axis, start, end, face.value);
			
			if vector.is_none() {continue};
			let vector: Vector3f = vector.unwrap();

			if !aabb.within_axis(AABB::get_axis(face.axis), &vector) {continue};
			let vector_distance: f64 = vector.square_length();

			if vector_distance > min_distance { continue };
			min_distance = vector_distance;
			hit_position = Some(vector);
		}

		return if hit_position.is_none() { return None } else {
			Some(HitResult {
				distance: min_distance,
				position: hit_position.unwrap()
			})
		}
	}

	fn get_axis(axis: Axis) -> Vec<Axis> {
		match axis {
			Axis::X => vec![Axis::Y, Axis::Z],
            Axis::Y => vec![Axis::X, Axis::Z],
            Axis::Z => vec![Axis::X, Axis::Y]
		}
	}
}