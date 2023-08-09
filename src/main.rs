use std::env;
use std::error::Error;
use serde::Deserialize;
use dotenv;
use clap::Parser;
const LAT: f32 = -41.2;
const LON: f32 = 174.7;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal",long_about = None)]

struct Args {
    // Number of days for weather forecast
    #[arg(short, default_value_t = 0)]
    days: u8,
}

#[derive(Debug, Deserialize)]
struct Coord {
    lat: f32,
    lon: f32,
}

#[derive(Deserialize, Debug)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct CurrentWeatherMain {
    temp: f32,
    feels_like: f32,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    coord: Coord,
    weather: Vec<Weather>,
    // base: String,
    main: CurrentWeatherMain,
}

fn main() -> Result<(), Box<dyn Error>>{
    dotenv::dotenv().unwrap();
    let mut api_key = None;
    for (key, value) in env::vars() {
        if key != "APIKEY" {
            continue;
        }
        api_key = Some(value);
        // println!("{key} : {value}");
    }
    if api_key.is_none() {
        panic!("Need API KEY");
    }
    let api_key = api_key.unwrap();
    let args = Args::parse();
    let method = match args.days {
        0 => "weather",
        _ => "forecast",
    };

    let cnt = args.days * 8;
    let url = format!("https://api.openweathermap.org/data/2.5/{method}?lat={LAT}&lon={LON}&appid={api_key}&units=metric&cnt={cnt}");
    let weather: CurrentWeather = reqwest::blocking::get(url)?
                    .json()?;
    println!("Weather description is: = {:?}", weather.weather[0].description);
    println!("Weather details: = {:?}", weather.weather[0]);
    Ok(())
}