use serde::Deserialize;

// In Rust, we use structs to define data structures, similar to Java classes with public fields.
// The #[derive(Deserialize)] attribute is a macro that automatically generates code
// to create this struct from JSON (like Jackson's ObjectMapper would use reflection).
// Debug allows us to print the struct using {:?} format specifier (like toString()).
#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    // 'pub' makes the field accessible from other modules (like public in Java).
    // If omitted, fields are private to the module by default.
    pub coord: Coord,
    pub weather: Vec<Weather>, // Vec<T> is a growable array, similar to ArrayList<T>
    pub main: Main,
    pub wind: Wind,
    pub name: String, // String is an owned, heap-allocated string (like Java's String)
}

#[derive(Deserialize, Debug)]
pub struct Coord {
    pub lon: f64, // f64 is a 64-bit floating point number (like double in Java)
    pub lat: f64,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub id: u32, // u32 is an unsigned 32-bit integer. Java doesn't have unsigned primitives by default.
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: u32,
}