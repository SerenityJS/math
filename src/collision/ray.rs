use napi_derive::napi;
use napi::{Error, JsFunction};

use crate::vec3f::Vector3f;

#[napi]
pub struct Raycaster {}

#[napi]
impl Raycaster {

	/**
	 * Traverses blocks along a line segment defined by start and end vectors.
	 * Applies a given condition function to each block and stops if the condition is met.
	 *
	 * @param start - The starting point of the line segment (Vector3f).
	 * @param end - The ending point of the line segment (Vector3f).
	 * @param condition - A function that takes BlockCoordinates and returns a boolean,
	 *                    defining the condition to stop traversing when met.
	 */
	#[napi(ts_args_type = "start: Vector3f, end: Vector3f, condition: (position: Vector3f) => bool")]
	pub fn transverse_blocks(start: &Vector3f, end: &Vector3f, condition: JsFunction) {
		if start.equals(end) { // No traversal needed if start and end are the same.
			return;
		}
		let direction: Vector3f = end.subtract(start);
		let mut current_position: Vector3f = start.floor();

		// Check if the initial block position meets the condition.
		if Raycaster::check_callback(current_position.clone(), &condition) { return };

		// Determine the step sizes for each axis.
		let step: Vector3f = Raycaster::sign(&direction);
		let step_size: Vector3f = Raycaster::step_size(&step, &direction);

		// Calculate the initial tMax values for each axis.
		let mut tmax: Vector3f = Vector3f::new(
			step_size.x * if step.x > 0.0 { 1.0 - Raycaster::boundary(direction.x) } else { Raycaster::boundary(direction.x) },
			step_size.y * if step.y > 0.0 { 1.0 - Raycaster::boundary(direction.y) } else { Raycaster::boundary(direction.y) },
			step_size.z * if step.z > 0.0 { 1.0 - Raycaster::boundary(direction.z) } else { Raycaster::boundary(direction.z) }
		);

		while tmax.x <= 1.0 || tmax.y <= 1.0 || tmax.z <= 1.0 {
			// Determine the axis to step along based on the smallest tMax value.
			if tmax.x < tmax.y && tmax.x < tmax.z {
                current_position.x += step.x;
				tmax.x += step_size.x;
			} else if tmax.y < tmax.z {
				current_position.y += step.y;
                tmax.y += step_size.y;
			} else {
				current_position.z += step.z;
                tmax.z += step_size.z;
			}

			// Check if the current block position meets the condition.
			if Raycaster::check_callback(current_position.clone(), &condition) { return };
		}
	}

	/**
	 * Returns a Vector3f containing the sign of each component of the given vector.
	 *
	 * @param vec - The vector to get the sign from.
	 * @return A new Vector3f with the sign of each component.
	 */
	#[napi]
	pub fn sign(vector: &Vector3f) -> Vector3f {
		Vector3f::new(
            vector.x.signum(),
            vector.y.signum(),
            vector.z.signum()
        )
	}

	
	#[napi]
	pub fn step_size(step: &Vector3f, direction: &Vector3f) -> Vector3f {
		Vector3f::new(
            if step.x == 0.0 { f64::INFINITY } else { step.x / direction.x },
            if step.y == 0.0 { f64::INFINITY } else { step.y / direction.y },
            if step.z == 0.0 { f64::INFINITY } else { step.z / direction.z },
        )
	}

	/**
	 * Calculates the distance from the given number to the next lower integer (boundary).
	 *
	 * @param number - The number to calculate the boundary for.
	 * @return The distance from the number to the nearest lower integer.
	 */
	#[napi]
	pub fn boundary(n: f64) -> f64 {
		return n.floor() - n;
	}

	fn check_callback(argument: Vector3f, callback: &JsFunction) -> bool {
		let callback_result: Result<bool, Error> = callback.call1(argument);

		match callback_result {
            Ok(value) => {
                value
            },
            Err(_) => {
                panic!("Callback throwed error");
            }
        }
	}

}