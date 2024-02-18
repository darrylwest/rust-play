// from https://www.youtube.com/watch?v=CHRNj5oubwc

struct Sedan;

impl Sedan {
    fn drive(&self) -> String {
        String::from("Sedan is driving")
    }
}

struct SUV;

impl SUV {
    fn drive(&self) -> String {
        String::from("SUV is driving")
    }
}

fn road_trip(vehicle: &Sedan) {
    println!("{}", vehicle.drive());
}

fn suv_trip(vehicle: &SUV) {
    println!("{}", vehicle.drive());
}

fn main() {
    let car = Sedan;
    road_trip(&car);

    let suv = SUV;
    suv_trip(&suv);
}
