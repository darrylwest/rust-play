use core::ops::{Add, Mul};
use num::traits::Signed;
// use super::velocity::Velocity;

/// Attitude values for pitch, roll and yaw are in degrees to -90..90.  For example
/// a pitch down of -45 degrees would be used to fly the drone forward where a pitch
/// of 10 degrees would fly the drone backwards.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Attitude<T: Signed + Add + Mul + Add<Output = T> + Mul<Output = T>> {
    pub pitch: T,
    pub roll: T,
    pub yaw: T,
}

impl<T: Signed + Add + Mul + Add<Output = T> + Mul<Output = T>> Attitude<T> {
    /// Create a new attitude with specified pitch, roll and yaw of type T.  
    ///
    /// # Example:
    ///
    /// Create a level attitude:
    ///
    /// ```rust
    /// use spacial_controller::attitude::Attitude;
    ///
    /// let level_attitude = Attitude::new(0, 0, 0);
    ///
    /// assert_eq!(level_attitude.pitch, 0);
    /// assert_eq!(level_attitude.roll, 0);
    /// assert_eq!(level_attitude.yaw, 0);
    /// ```
    pub fn new(pitch: T, roll: T, yaw: T) -> Attitude<T> {
        Attitude { pitch, roll, yaw }
    }

    ///
    /// Change the current pitch to the specified value in degrees -90..90
    ///
    /// # Exammple:
    ///
    /// ```rust
    /// use spacial_controller::attitude::Attitude;
    ///
    /// let mut attitude = Attitude::new(0.0, 0.0, 0.0);
    /// attitude.pitch_to(-20.0);
    ///
    /// assert_eq!(attitude.pitch, -20.0);
    /// ```
    pub fn pitch_to(&mut self, value: T) {
        self.pitch = value;
    }

    ///
    /// Change the current roll position to the specified value in degrees -360..360
    ///
    /// # Exammple:
    ///
    /// ```rust
    /// use spacial_controller::attitude::Attitude;
    ///
    /// let mut attitude = Attitude::new(0.0, 0.0, 0.0);
    /// attitude.roll_to(-45.0);
    ///
    /// assert_eq!(attitude.roll, -45.0);
    /// ```
    pub fn roll_to(&mut self, value: T) {
        self.roll = value;
    }

    ///
    /// Change the current yaw position to the specified value in degrees -360..360
    ///
    /// # Exammple:
    ///
    /// ```rust
    /// use spacial_controller::attitude::Attitude;
    ///
    /// let mut attitude = Attitude::new(0.0, 0.0, 0.0);
    /// attitude.yaw_to(15.0);
    ///
    /// assert_eq!(attitude.yaw, 15.0);
    /// ```
    pub fn yaw_to(&mut self, value: T) {
        self.yaw = value;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_f64() {
        let a = Attitude::new(-20.0, 10.0, 3.0);

        assert_eq!(a.pitch, -20.0);
        assert_eq!(a.roll, 10.0);
        assert_eq!(a.yaw, 3.0);
    }

    #[test]
    fn new_i32() {
        let a = Attitude::new(30, 20, 10);

        assert_eq!(a.pitch, 30);
        assert_eq!(a.roll, 20);
        assert_eq!(a.yaw, 10);
    }

    #[test]
    fn pitch_to() {
        let mut a = Attitude::new(0.0, 0.0, 0.0);

        assert_eq!(a.pitch, 0.0);
        a.pitch_to(-45.0);
        assert_eq!(a.pitch, -45.0);
        assert_eq!(a.roll, 0.0);
        assert_eq!(a.yaw, 0.0);
    }

    #[test]
    fn roll_to() {
        let mut a = Attitude::new(0.0, 0.0, 0.0);

        assert_eq!(a.roll, 0.0);
        a.roll_to(-45.0);
        assert_eq!(a.pitch, 0.0);
        assert_eq!(a.roll, -45.0);
        assert_eq!(a.yaw, 0.0);
    }

    #[test]
    fn yaw_to() {
        let mut a = Attitude::new(0.0, 0.0, 0.0);

        assert_eq!(a.yaw, 0.0);
        a.yaw_to(40.0);

        assert_eq!(a.pitch, 0.0);
        assert_eq!(a.roll, 0.0);
        assert_eq!(a.yaw, 40.0);
    }
}
