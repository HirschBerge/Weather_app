use clap::{App, Arg};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct WeatherData {
    name: String,
    main: Main,
    weather: Vec<Weather>,
    sys: Sys,
    coord: Coord,
}

const API_KEY: &str = "4b0a11494a50bcaf28b0f5aa8099fec4";

#[derive(Debug, Serialize, Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    main: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sys {
    country: String,
    state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Coord {
    lat: f64,
    lon: f64,
}

fn get_emoji(weather_main: &str) -> &str {
    match weather_main {
        "Clear" => "☀️",
        "Clouds" => "☁️",
        "Rain" => "🌧️",
        "Drizzle" => "🌦️",
        "Thunderstorm" => "⛈️",
        "Snow" => "❄️",
        "Mist" | "Smoke" | "Haze" | "Dust" | "Fog" | "Sand" | "Ash" | "Squall" | "Tornado" => "🌫️",
        _ => "",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Weather App")
        .version("1.0")
        .author("Hirschy Kirkwood")
        .about("Displays the current weather of a given city")
        .arg(
            Arg::with_name("location")
                .short("l")
                .long("location")
                .takes_value(true)
                .help("The name of the city to get weather for"),
        )
        .get_matches();

    let default_location = "Pittsburgh".to_string();
    let location = matches.value_of("location").unwrap_or(&default_location);

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
        location, API_KEY
    );

    let resp = reqwest::get(&url).await?.json::<WeatherData>().await?;
    let emoji = get_emoji(&resp.weather[0].main);
    // let _state = resp.sys.state.unwrap_or_else(|| "N/A".to_string());
    println!(
        "Current weather in {}, {} - {} {}ԅ(‾⌣‾ԅ) - {:.1}°F\n",
        resp.name, resp.sys.country, resp.weather[0].main, emoji, resp.main.temp
    );

    Ok(())
}