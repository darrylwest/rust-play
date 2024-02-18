// from https://www.youtube.com/watch?v=CHRNj5oubwc
//
// dynamic dispatch: pointer to vtable (dyn) when you have lots of types
// static displatch: with generics (impl) fewer types

struct Sedan; 

impl LandCapable for Sedan { }

struct SUV;

impl LandCapable for SUV {
    fn drive(&self) -> String {
        String::from("SUV is driving")
    }
}

fn road_trip(vehicle: &impl LandCapable) {
    println!("{}", vehicle.drive());
}

trait LandCapable {
    fn drive(&self) -> String {
        String::from("LandCapable vehicle is driving")
    }
}

fn main() {
    let car = Sedan;
    road_trip(&car);

    let suv = SUV;
    road_trip(&suv);
}
