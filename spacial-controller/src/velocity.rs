use core::ops::{Add, AddAssign, Mul};
use num::traits::Signed;

/// Velocity is in units, possibly Knots, feet/second, miles/hour, Kilometer/hour, etc.  The
/// types are constrained to Signed numbers that implement Add, Mul, etc.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Velocity<T: Signed + Add + Mul + Add<Output = T> + Mul<Output = T>> {
    pub vx: T,
    pub vy: T,
    pub vz: T,
}

/// Types are constrained to Signed numbers that implement Add, Mul, etc.
impl<T: Signed + Copy + Add + AddAssign + Mul + Add<Output = T> + Mul<Output = T>> Velocity<T> {
    /// Create a new Velocity instance with vx, vy and vt of type T.
    ///
    /// # Example
    ///
    /// This example creates a new Velocity instace of type f64.
    ///
    /// ```rust
    /// use spacial_controller::velocity::Velocity;
    ///
    /// let velocity = Velocity::new(8.0, 3.0, 5.0);
    ///
    /// assert_eq!(velocity.vx, 8.0);
    /// assert_eq!(velocity.vy, 3.0);
    /// assert_eq!(velocity.vz, 5.0);
    /// ```
    pub fn new(vx: T, vy: T, vz: T) -> Velocity<T> {
        Velocity { vx, vy, vz }
    }

    /// Change velocity settings to new specified values of vx, vy and vz.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use spacial_controller::velocity::Velocity;
    ///
    /// let mut velocity = Velocity::new(20, 0, 5);
    ///
    /// velocity.change_to(0, 0, 0);
    ///
    /// assert_eq!(velocity.vx, 0);
    /// assert_eq!(velocity.vy, 0);
    /// assert_eq!(velocity.vz, 0);
    /// ```
    pub fn change_to(&mut self, vx: T, vy: T, vz: T) {
        self.vx = vx;
        self.vy = vy;
        self.vz = vz;
    }

    /// Change the velocity by adding a new vx, vy, vz values to the current values.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use spacial_controller::velocity::Velocity;
    ///
    /// let mut velocity = Velocity::new(0.0, 0.0, 0.0);
    ///
    /// velocity.change_by(5.0, 2.0, 1.5);
    /// assert_eq!(velocity.vx, 5.0);
    /// assert_eq!(velocity.vy, 2.0);
    /// assert_eq!(velocity.vz, 1.5);
    ///
    /// velocity.change_by(3.0, 1.0, -1.0);
    /// assert_eq!(velocity.vx, 8.0);
    /// assert_eq!(velocity.vy, 3.0);
    /// assert_eq!(velocity.vz, 0.5);
    /// ```
    pub fn change_by(&mut self, vx: T, vy: T, vz: T) {
        self.vx += vx;
        self.vy += vy;
        self.vz += vz;
    }

    /// Change all velocity settings to a single specified value.
    ///
    /// # Example
    ///
    /// A typical way of stoping a drone, say to hover in place.
    ///
    /// ```rust
    /// use spacial_controller::velocity::Velocity;
    ///
    /// let mut velocity = Velocity::new(55, 0, -10);
    ///
    /// velocity.change_all(0);
    ///
    /// assert_eq!(velocity.vx, 0);
    /// assert_eq!(velocity.vy, 0);
    /// assert_eq!(velocity.vz, 0);
    /// ```
    pub fn change_all(&mut self, value: T) {
        self.vx = value;
        self.vy = value;
        self.vz = value;
    }
}

#[cfg(test)]
mod test {
    use super::Velocity;

    #[test]
    fn new_f64() {
        let v = Velocity::new(0.5, 0.2, 0.25);

        assert_eq!(v.vx, 0.5);
        assert_eq!(v.vy, 0.2);
        assert_eq!(v.vz, 0.25);
    }

    #[test]
    fn new_i32() {
        let v = Velocity::new(5, 2, 25);

        assert_eq!(v.vx, 5);
        assert_eq!(v.vy, 2);
        assert_eq!(v.vz, 25);
    }

    #[test]
    fn change_to() {
        let mut v = Velocity::new(0, 0, 0);
        v.change_to(5, 10, 20);

        assert_eq!(v.vx, 5);
        assert_eq!(v.vy, 10);
        assert_eq!(v.vz, 20);
    }

    #[test]
    fn change_by() {
        let mut v = Velocity::new(0.0, 0.0, 0.0);
        v.change_by(0.5, 0.2, 0.25);

        assert_eq!(v.vx, 0.5);
        assert_eq!(v.vy, 0.2);
        assert_eq!(v.vz, 0.25);
    }
}
