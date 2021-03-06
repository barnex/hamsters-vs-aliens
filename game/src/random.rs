use super::prelude::*;
use rand::Rng;
use rand_distr::StandardNormal;

/// Return true once every `avg_period` (period in seconds, if dt is the tick time step in seconds).
/// E.g.:
///
///   rand_every(dt, 10.0)
///
/// returns true approximately every 10 seconds, on average.
/// `dt` is assumed to be small compared to	`avg_period`.
pub fn rand_every(dt: f32, avg_period: f32) -> bool {
	let x: f32 = rand::thread_rng().gen();
	(x * avg_period) < dt
}

pub fn rand_vec() -> vec3 {
	vec3(
		rand_normal(), //
		rand_normal(),
		rand_normal(),
	)
	.normalized()
}

pub fn rand_normal() -> f32 {
	rand::thread_rng().sample(StandardNormal)
}
