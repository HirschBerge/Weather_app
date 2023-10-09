use chrono::{NaiveDateTime, DateTime, Utc, FixedOffset};
use clap::{App, Arg};
use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::env;
use std::path::Path;

#[allow(deprecated)]

#[derive(Debug, Serialize, Deserialize)]
struct WeatherData {
    name: String,
    main: Main,
    weather: Vec<Weather>,
    sys: Sys,
    coord: Coord,
}

// const api_key: String = env::var("api_key").expect("api_key not set in .env file");//= "4b0a11494a50bcaf28b0f5aa8099fec4";

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

#[derive(Debug, Serialize, Deserialize)]
struct ForecastData {
    list: Vec<Forecast>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Forecast {
    dt_txt: String,
    main: Main,
    weather: Vec<Weather>,
}

impl fmt::Display for Forecast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let emoji = get_emoji(&self.weather[0].main);
        write!(
            f,
            "{} {}: {:.1}°F",
            self.dt_txt, emoji, self.main.temp
        )
    }
}
fn convert_date(date_str: &str) -> String {
    let datetime = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").unwrap();
    let utc_datetime = DateTime::<Utc>::from_utc(datetime, Utc);

    // Create a fixed offset of -4 hours (for Eastern Standard Time)
    let est_offset = FixedOffset::east_opt(-4 * 3600).unwrap();  // 4 hours in seconds

    let est_datetime = utc_datetime.with_timezone(&est_offset);
    est_datetime.format("%m-%d %H:%M").to_string()
}

async fn get_current_weather(url: &str) -> Result<WeatherData, reqwest::Error> {
    reqwest::get(url).await?.json::<WeatherData>().await
}

fn get_emoji(main: &str) -> &str {
    match main {
        "Thunderstorm" => "⛈️",
        "Drizzle" => "🌦️",
        "Rain" => "🌧️",
        "Snow" => "❄️",
        "Clear" => "☀️",
        "Clouds" => "☁️",
        "Mist" | "Smoke" | "Haze" | "Dust" | "Fog" | "Sand" | "Ash" | "Squall" | "Tornado" => "🌫️",
        _ => "",
    }
}

fn print_current_weather(resp: &WeatherData, emoji: &str) {
    println!(
        "Current weather in \x1b[1;31m{}, {}\x1b[0m - \x1b[1;32m{}\x1b[0m {} - \x1b[1;33m{:.1}\x1b[0m°F",
        resp.name, resp.sys.country, resp.weather[0].main, emoji, resp.main.temp
    );
}

async fn get_forecast_weather(url: &str) -> Result<ForecastData, reqwest::Error> {
    reqwest::get(url).await?.json::<ForecastData>().await
}

fn print_forecast_weather(forecast: &ForecastData, location: &str) {
    println!("Forecast for the next 18 hours in {}:", location);

    for i in 0..6 {
        let temp = forecast.list[i].main.temp;
        let emoji = get_emoji(&forecast.list[i].weather[0].main);
        let description = &forecast.list[i].weather[0].description;
        let time = &forecast.list[i].dt_txt.trim();
        let time = convert_date(&time);

        print!("\x1b[1;31m{}\x1b[0m{}\x1b[1;32m{:.1}°F\x1b[0m", time, emoji, temp);
        print!("  \x1b[1;33m{}\x1b[0m | ", description);
    }
}

fn print_polybar(resp: &WeatherData, emoji: &str){
    println!(
        "{} {} {:.1}°F",
        resp.weather[0].main, emoji, resp.main.temp
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let env_path = Path::new(manifest_dir).join(".env");
    dotenv::from_path(&env_path).ok();
    let api_key = env::var("API_KEY").expect("api_key not set in .env file");
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
        .arg(
            Arg::with_name("forecast")
                .short("f")
                .long("forecast")
                .takes_value(false)
                .help("Do you want the forecast?"),
        )
        .arg(
            Arg::with_name("polybar")
                .short("p")
                .long("polybar")
                .takes_value(false)
                .help("For use w/ polybar"),
        )
        .get_matches();

    let default_location = "Pittsburgh".to_string();
    let location = matches.value_of("location").unwrap_or(&default_location);

    let current_url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
        location, api_key
    );
    let forecast_url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=imperial",
        location, api_key
    );

    let resp = get_current_weather(&current_url).await?;
    let emoji = get_emoji(&resp.weather[0].main);
    if !matches.is_present("polybar"){
        print_current_weather(&resp, &emoji);
    }
    if matches.is_present("polybar"){
        print_polybar(&resp, &emoji);
    }
    if matches.is_present("forecast") {
        let resp_forecast = get_forecast_weather(&forecast_url).await?;
        print_forecast_weather(&resp_forecast, &resp.name);
        println!("\n")
    }

    Ok(())
}