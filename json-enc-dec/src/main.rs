
use serde::{Serialize, Deserialize};
use chrono::naive::NaiveDateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
enum SerializedColor {
    Red,
    Blue,
    Green,
}

type Color = SerializedColor;

#[derive(Serialize, Deserialize, Debug)]
struct Blob {
    name: String,
    when: NaiveDateTime,
    maybe: Option<NaiveDateTime>,
    color: Color,
    x: i32,
    y: i32,
}

fn main() {
    let dt: NaiveDateTime = Utc::now().naive_utc();
    println!("it's now: {}", dt);

    let mycolor: Color = Color::Red;
    let blob = Blob{
        name: "Flurber".to_string(),
        when: dt,
        // maybe: Some(dt),
        maybe: None,
        color: mycolor,
        x: 1, 
        y: 2,
    };
    
    let serialized = serde_json::to_string_pretty(&blob)
        .expect("This should encode to json");
    
    println!("serialized = {}", serialized);
    
    let deserialized: Blob = serde_json::from_str(&serialized)
        .expect("This should decode from json string");
    
    println!("and back {:?}", deserialized);
}
