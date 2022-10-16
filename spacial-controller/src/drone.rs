// use core::ops::{Add, Mul};

// use super::position::Position;
// use super::attitude::Attitude;
// use super::velocity::Velocity;

#[derive(Debug, PartialEq, Eq)]
pub enum FlightState {
    Ground, // this is where everything starts
    Takeoff,
    Hover,
    InFlight, // when in the air, you are either hovering or in flight
    Land,     // the end of the flight
    Critical, // battery failures?
}

impl Default for FlightState {
    fn default() -> Self {
        Self::Ground
    }
}

///
/// Drone stucts represent the state of a drone--it's position, velocity, attituce, state, etc.  FlightPlans use a
/// vector of drone structs to control the drones condition.
///
#[derive(Debug, Default)]
pub struct Drone {
    pub state: FlightState,
    // pub position: Position,
    // pub attitude: Attitude,
    // pub velocity: Velocity,
}

/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_drone() {
        let drone = Drone::new();
        let velocity = Velocity::new();
        let position = Position::new(0.0, 0.0, 0.0);
        let attitude = Attitude::new();

        assert_eq!(drone.position, position);
        assert_eq!(drone.velocity, velocity);
        assert_eq!(drone.attitude, attitude);
        assert_eq!(drone.state, FlightState::Ground);
    }

    #[test]
    fn lift_off() {
        let d0 = Drone::new();
        let p = Position {
            x: 10.0,
            y: 5.0,
            z: 25.0,
        };
        let v = Velocity {
            vx: 50.0,
            vy: 50.0,
            vz: 50.0,
        };
        let d1 = d0.fly_to(p, v);

        assert_eq!(d1.position.x, 10.0);
        assert_eq!(d1.position.y, 5.0);
        assert_eq!(d1.position.z, 25.0);
        assert_eq!(d1.velocity.vx, 50.0);
        assert_eq!(d1.velocity.vy, 50.0);
        assert_eq!(d1.velocity.vz, 50.0);
    }
}
 */
