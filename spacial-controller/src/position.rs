use core::ops::{Add, AddAssign, Mul, Sub};
use num::traits::Signed;

///
/// The generic position values are related to the drone's world grid and can be in any units.  A good
/// starting point would be feet or meters.
///
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Position<T: Signed + Add + Sub + Mul + Add<Output = T> + Mul<Output = T>> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Signed + Copy + Add + Sub + AddAssign + Mul + Add<Output = T> + Mul<Output = T>> Position<T> {
    /// Create a new position point at the specified x, y, z values, in f64, f32, i64, i32, etc.;
    ///
    /// # Example:
    ///
    /// This example creates a typical position struct with f64 types (default).
    ///
    /// ```rust
    /// use spacial_controller::position::Position;
    ///
    /// let mut position = Position::new(7.3, -4.3, 15.9);
    ///
    /// assert_eq!(position.x, 7.3);
    /// assert_eq!(position.y, -4.3);
    /// assert_eq!(position.z, 15.9);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Position<T> {
        Position { x, y, z }
    }

    /// Set values of x, y, and z to the specified value.  This is a convenient way to zero a
    /// position.
    ///
    /// # Example:
    ///
    /// Zero a position.
    ///
    /// ```rust
    /// use spacial_controller::position::Position;
    ///
    /// let mut position = Position::new(7, -4, 15);
    ///
    /// position.set_all(0);
    ///
    /// assert_eq!(position.x, 0);
    /// assert_eq!(position.y, 0);
    /// assert_eq!(position.z, 0);
    /// ```
    pub fn set_all(&mut self, value: T) {
        self.x = value;
        self.y = value;
        self.z = value;
    }

    /// Move the current position to the new x, y, z;
    ///
    /// # Example:
    ///
    /// ```rust
    /// use spacial_controller::position::Position;
    ///
    /// let mut position = Position::new(0.0, 0.0, 0.0);
    /// position.move_to(5.0, -6.5, 25.0);
    ///
    /// assert_eq!(position.x, 5.0);
    /// assert_eq!(position.y, -6.5);
    /// assert_eq!(position.z, 25.0);
    /// ```
    pub fn move_to(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Move the position by adding the x, y, and z values.
    ///
    /// # Esample:
    ///
    /// ```rust
    /// use spacial_controller::position::Position;
    ///
    /// let mut position = Position{ x: 5.0, y: 3.0, z: 22.5 };
    /// position.move_by(7.0, -9.5, 2.0);
    ///
    /// assert_eq!(position.x, 12.0);
    /// assert_eq!(position.y, -6.5);
    /// assert_eq!(position.z, 24.5);
    /// ```
    pub fn move_by(&mut self, x: T, y: T, z: T) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let p = Position::new(3.0, 4.0, 5.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
        assert_eq!(p.z, 5.0);
    }

    #[test]
    fn new_i32() {
        let mut p = Position::new(0, 0, 0);

        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
        assert_eq!(p.z, 0);

        p.move_by(5, 7, 9);

        assert_eq!(p.x, 5);
        assert_eq!(p.y, 7);
        assert_eq!(p.z, 9);
    }

    #[test]
    fn new_i8() {
        let p = Position::new(0_i8, 0_i8, 0_i8);

        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
        assert_eq!(p.z, 0);
    }

    #[test]
    fn move_to() {
        let mut p = Position::new(0.0, 0.0, 0.0);

        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);

        p.move_to(10.0, 50.0, 25.5);

        assert_eq!(p.x, 10.0);
        assert_eq!(p.y, 50.0);
        assert_eq!(p.z, 25.5);
    }

    #[test]
    fn move_by() {
        let mut p = Position::new(0.0, 0.0, 0.0);

        p.move_by(10.0, 50.0, 25.5);

        assert_eq!(p.x, 10.0);
        assert_eq!(p.y, 50.0);
        assert_eq!(p.z, 25.5);
    }

    #[test]
    fn set_all() {
        let mut p = Position::new(0, 0, 0);

        p.set_all(25);

        assert_eq!(p.x, 25);
        assert_eq!(p.y, 25);
        assert_eq!(p.z, 25);
    }
}
