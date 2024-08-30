#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_void;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Fusion {
    fusion: Arc<Mutex<Ahrs>>,
    gravity: f64,
}

unsafe impl Send for Fusion {}
unsafe impl Sync for Fusion {}

struct Ahrs {
    ahrs: *mut c_void,
}

impl std::fmt::Debug for Ahrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ahrs").finish()
    }
}

impl Ahrs {
    fn new() -> Self {
        Self {
            ahrs: unsafe { create_ahrs() },
        }
    }

    fn get(&self) -> *mut c_void {
        self.ahrs
    }
}

/// Safety: Ahrs is not Sync nor Send, but the higher level Fusion ensures that it is only accessed from one thread.
unsafe impl Send for Ahrs {}
unsafe impl Sync for Ahrs {}

impl Fusion {
    pub fn new(gravity: f64) -> Self {
        Self {
            fusion: Arc::new(Mutex::new(Ahrs::new())),
            gravity,
        }
    }

    /// Update the AHRS with new sensor data.
    /// gyroscope units are degrees/s
    /// accelerometer units are g
    pub fn update_no_magnetometer(&mut self, dt: f64, gyro: [f64; 3], accel: [f64; 3]) {
        let lock = self.fusion.lock().unwrap();
        unsafe {
            update_no_magnetometer(
                lock.get(),
                dt,
                gyro[0],
                gyro[1],
                gyro[2],
                accel[0],
                accel[1],
                accel[2],
            );
        }
    }

    /// Update the AHRS with new sensor data.
    /// gyroscope units are radians/s
    /// accelerometer units are m/s^2
    pub fn update_no_magnetometer_ros(&mut self, dt: f64, gyro: [f64; 3], accel: [f64; 3]) {
        self.update_no_magnetometer(
            dt,
            gyro.map(|v| v.to_degrees()),
            accel.map(|a| a / self.gravity),
        )
    }

    /// Get orientation in world frame as a quaternion.
    /// The quaternion is in the form [w, x, y, z].
    pub fn get_quaternion(&self) -> [f64; 4] {
        let mut q = vec![0.0; 4]; // TODO test without vec on stack
        let lock = self.fusion.lock().unwrap();
        unsafe {
            get_quaternion(
                lock.get(),
                q.as_mut_ptr().offset(0),
                q.as_mut_ptr().offset(1),
                q.as_mut_ptr().offset(2),
                q.as_mut_ptr().offset(3),
            );
        }
        [q[0], q[1], q[2], q[3]]
    }

    /// Get linear acceleration in sensor frame.
    /// The acceleration is in the form [x, y, z].
    /// Units are g.
    pub fn get_linear_acceleration(&self) -> [f64; 3] {
        let mut a = vec![0.0; 3]; // TODO test without vec on stack
        let lock = self.fusion.lock().unwrap();
        unsafe {
            get_linear_acceleration(
                lock.get(),
                a.as_mut_ptr().offset(0),
                a.as_mut_ptr().offset(1),
                a.as_mut_ptr().offset(2),
            );
        }
        [a[0], a[1], a[2]]
    }

    pub fn get_linear_acceleration_ros(&self) -> [f64; 3] {
        self.get_linear_acceleration().map(|a| a * self.gravity)
    }
}

impl Drop for Fusion {
    fn drop(&mut self) {
        let lock = self.fusion.lock().unwrap();
        unsafe {
            free_ahrs(lock.get());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linking() {
        let mut fusion = Fusion::new(9.81);
        fusion.update_no_magnetometer(0.01, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let _ = fusion.get_quaternion();
    }
}
