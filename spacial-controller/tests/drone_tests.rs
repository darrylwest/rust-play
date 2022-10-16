/*
use spacial_controller::attitude::Attitude;
use spacial_controller::drone::{Drone, FlightState};
use spacial_controller::position::Position;
use spacial_controller::velocity::Velocity;

#[test]
fn new_drone() {
    let drone = Drone::new();
    let v = Velocity::new();
    let p = Position::new();
    let a = Attitude::new();

    assert_eq!(drone.velocity, v);
    assert_eq!(drone.position, p);
    assert_eq!(drone.attitude, a);
    assert_eq!(drone.state, FlightState::Ground);
}

#[test]
fn test_mut() {
    let mut drone = Drone::new();

    let mut p = Position::new();
    p.move_to(22.0, 33.0, 50.0);

    drone.position.x = p.x;
    drone.position.y = p.y;
    drone.position.z = p.z;
    drone.state = FlightState::Hover;

    assert_eq!(drone.position, p);
    assert_eq!(drone.state, FlightState::Hover);
}

#[test]
fn take_off() {
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
    let d1 = d0.fly_to(p.clone(), v.clone());

    let a = Attitude::new();

    assert_eq!(d1.position, p);
    assert_eq!(d1.velocity, v);
    assert_eq!(d1.attitude, a)
}

*/
