// 'mod' declares a submodule. It looks for a file named 'models.rs' or 'models/mod.rs'.
// This is similar to having a class in the same package in Java.
mod models;

// 'use' brings items into scope, similar to 'import' in Java.
use dotenv::dotenv;
use std::env;
use std::error::Error;
use models::WeatherResponse;

// Constants are declared with 'const'. Type annotation is mandatory.
// 'static' lifetime is inferred for string literals.
const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

// #[tokio::main] is a macro that transforms the async main function into a synchronous one
// that initializes the Tokio runtime and executes the async code.
// Java doesn't have a direct equivalent, but it's like setting up a main thread that joins on a CompletableFuture.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from a .env file, if present.
    dotenv().ok();

    // Fetch the API key from environment variables.
    // env::var returns a Result<String, VarError>.
    // expect() unwraps the Result, panicking with the message if it's an Err.
    // In Java, this would be like System.getenv("OPENWEATHER_API_KEY") and throwing a RuntimeException if null.
    let api_key = env::var("OPENWEATHER_API_KEY")
        .expect("OPENWEATHER_API_KEY must be set in .env file");

    // Define the city we want to look up.
    // 'let' binds a variable. Variables are immutable by default.
    let city = "London";

    println!("Fetching weather for {}...", city);

    // Call the async function. In Rust, calling an async function returns a Future (like CompletableFuture).
    // .await suspends the current function until the Future completes.
    // The '?' operator checks the Result. If Ok, it unwraps the value. If Err, it returns the error from the function immediately.
    // This is a concise way to do error propagation, replacing try-catch blocks for checked exceptions.
    let weather = get_weather(city, &api_key).await?;

    // Print the result using the Debug implementation (derived in models.rs).
    // {:?} is the debug formatter. {} is the display formatter (like toString()).
    println!("Full Weather Data: {:?}", weather);

    println!("---------------------------------");
    println!("Weather in {}: {}", weather.name, weather.weather[0].description);
    println!("Temperature: {:.2}°C", weather.main.temp);
    println!("Humidity: {}%", weather.main.humidity);
    println!("Wind Speed: {} m/s", weather.wind.speed);

    // Return Ok(()) to indicate success. () is the unit type, similar to void in Java, but it's an actual value.
    Ok(())
}

// An async function definition.
// Arguments are passed by reference (&str) to avoid copying strings (borrowing).
// Returns a Result<WeatherResponse, Box<dyn Error>>.
// Box<dyn Error> is a type-erased error object, similar to throwing 'Exception' in Java.
async fn get_weather(city: &str, api_key: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    // Construct the URL. format! is a macro for string interpolation.
    let url = format!("{}?q={}&appid={}&units=metric", BASE_URL, city, api_key);

    // reqwest::get is an async HTTP call.
    // .await waits for the response.
    // ? propagates any network errors.
    let response = reqwest::get(&url).await?;

    // Check if the status is success (200-299).
    if response.status().is_success() {
        // Parse the JSON body into our WeatherResponse struct.
        // This uses Serde under the hood.
        let weather_data = response.json::<WeatherResponse>().await?;
        Ok(weather_data)
    } else {
        // If the API returns an error, we can construct a custom error message.
        // .into() converts the String into a Box<dyn Error>.
        let error_msg = format!("Request failed with status: {}", response.status());
        Err(error_msg.into())
    }
}
