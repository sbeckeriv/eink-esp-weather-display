use std::fs::File;
use std::io::prelude::*;

use std::path::PathBuf;

use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

mod tasks;
use tasks::*;

mod util;
use util::*;

mod env_data;
pub use env_data::*;

mod weather;
pub use weather::*;

pub mod draw;
pub use draw::*;

mod render;
pub use render::*;

pub fn get_env_data() -> EnvData {
    let file_path = get_envdata_filepath();
    EnvData::from_file(&file_path)
}

pub fn get_envdata_filepath() -> PathBuf {
    let path_str = std::env::args().nth(1)
        .expect("1st cli argument did not exist");
    PathBuf::from(path_str)
}

pub fn get_output_path() -> PathBuf {
    let path_str = std::env::args().nth(2)
        .expect("2nd cli argument did not exist");
    PathBuf::from(path_str) 
}

pub struct DisplayData {
    current_weather: CurrentWeather,
    avg_forecast: AvgForecast,
    todoist_tasks: Vec<Task>,
}

fn gather_data(env_data: &EnvData, tx: SyncSender<(String, String, String)>) {
    let client = create_weather_client(&env_data);
    //let daily_forecast = get_daily_forecast(&env_data, &client);
    //println!("{daily_forecast:#?}");

    let current_weather_json = get_current_weather(&env_data, &client);
    let hourly_forecast_json = get_hourly_forecast(&env_data, &client);

    tx.send((current_weather_json, hourly_forecast_json, "[]".to_string())).unwrap();
}

fn parse_data(tx: SyncSender<DisplayData>, current_weather_json: String, hourly_forecast_json: String, tasks_json: String) {
    // start a new context for parsing the json
    
    let current_weather = parse_current_weather(&current_weather_json);
    let forecast = parse_hourly_forecast(&hourly_forecast_json);
    let avg_forecast = gather_5day_forecast(&forecast);

    let data = DisplayData {
        current_weather,
        avg_forecast,
        todoist_tasks: Vec::new(),
    };
    tx.send(data).unwrap();
}

fn main() {
    if std::env::args().len() != 3 {
        eprintln!("usage: halldisplay <env json filename> <output filename>");
        return;
    }
    let env_data = get_env_data();
    let output_filepath = get_output_path();

    let mut output_data_file = File::create(&output_filepath).expect("failed to create file");
    let mut output_image_file = File::create(&output_filepath.with_extension("png")).expect("failed to create file");

    let (json_sender, json_receiver) = sync_channel(1);
    let (data_sender, data_receiver) = sync_channel(1);

    let use_debug_data = false;
    let display_data = if !use_debug_data {
        let env_data = env_data.clone();
        thread::spawn(move || gather_data(&env_data, json_sender));
        let (current_weather_json, hourly_weather_json,a) = json_receiver.recv()
            .expect("failed to get json");


        thread::spawn(move || parse_data(data_sender, current_weather_json, hourly_weather_json, "[]".into()));

        data_receiver.recv()
            .expect("failed to get data")
    }
    else {
        get_test_data()
    };



    let (buffer, image) = render(env_data.local_timezone, display_data);

    println!("image file {:?}", image.write_to(&mut output_image_file, image::ImageOutputFormat::Png));

}
