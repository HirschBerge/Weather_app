use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(deprecated)]
#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    pub(crate) coord: Coord,
    pub main: Main,
    pub name: String,
    pub(crate) sys: Sys,
    pub(crate) weather: Vec<Weather>,
}

/// I don't like the name of Main, but it's taken from the data gathered from the API, so suck it lol
#[derive(Debug, Deserialize, Serialize)]
pub struct Main {
    pub temp: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Weather {
    pub main: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Sys {
    pub country: String,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Coord {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastData {
    pub list: Vec<Forecast>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
    pub dt_txt: String,
    pub main: Main,
    pub weather: Vec<Weather>,
}

impl fmt::Display for Forecast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let emoji = get_emoji(&self.weather[0].main);
        write!(f, "{} {} : {:.1}°F", self.dt_txt, emoji, self.main.temp)
    }
}

pub fn get_emoji(main: &str) -> &str {
    match main {
        "Thunderstorm" => "⛈️",
        "Drizzle" => "🌦️ ",
        "Rain" => "🌧️ ",
        "Snow" => "❄️ ",
        "Clear" => "☀️ ",
        "Clouds" => "☁️",
        "Mist" | "Smoke" | "Haze" | "Dust" | "Fog" | "Sand" | "Ash" | "Squall" | "Tornado" => "🌫️ ",
        _ => "Fuck Me",
    }
}
