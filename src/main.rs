//use std::sync::mpsc::channel; 
//use std::time::Duration; 
//use std::path::{PathBuf, Path}; 
use std::fs; 
//use std::fs::{File, OpenOptions}; 
//use std::ffi::OsStr; 
//use std::io::prelude::*; 
use serde::Deserialize; 
use toml; 
//use chrono::Utc; 
  
const CONFIG_FILE_NAME_PATH: &str = "/home/runner/joule-heat-rust/src/app_setting.toml"; 

#[derive(Deserialize)]
#[derive(Debug)]
struct Config { 
    resistance_tbl_path: String, 
    specific_heat_tbl_path: String,
    heat_transfer_tbl_path: String,
    current_tbl_path: String,
    surface_area: f64,
    weight: f64,
    start_sample_temperature: f64,
    enviroment_temperature: f64,
    pulse_duration: f64,
    num_of_iterations: u64,
}
  
impl Config { 
    fn build(file_content: &String) -> Config { 
        let cfg: Config = match toml::from_str(&file_content) { 
            Ok(cfg) => cfg, 
            Err(error) => panic!("Problem parsing config file: '{}'. {}", &CONFIG_FILE_NAME_PATH, error), 
        }; 
        cfg 
    } 
}

#[derive(Deserialize)]
#[derive(Debug)]
struct RowIndexValue {
    index: f64,
    value: f64,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct TblIndexValueData {
    index_value_data: Vec<RowIndexValue>,
}

impl TblIndexValueData { 
    fn fill_tbl_index_value(file_content: &String) -> TblIndexValueData {
        let tbl_data: TblIndexValueData = match toml::from_str(&file_content) { 
            Ok(tbl_data) => tbl_data, 
            Err(error) => panic!("Problem parsing config file: {}", error), 
        }; 
        tbl_data 
    }

    fn get_down_index_value(&self, index: f64) -> (f64, f64) {
        let (down_index, down_value) = match self.index_value_data.iter().find(|&x| x.index <= index) {
            Some(value) => (value.index, value.value),
            None => (self.index_value_data.first().unwrap().index, self.index_value_data.first().unwrap().value),
        };
    (down_index, down_value)
    }

    fn get_up_index_value(&self, index: f64) -> (f64, f64) {
        let (up_index, up_value) = match self.index_value_data.iter().find(|&x| x.index >= index) {
            Some(value) => (value.index, value.value),
            None => (self.index_value_data.last().unwrap().index, self.index_value_data.last().unwrap().value),
        };
    (up_index, up_value)
    }

    fn get_delta(down_number: f64, up_number: f64) -> f64 {
        let delta_number = if up_number == down_number {
            down_number
        } else {
            up_number - down_number
        };
        delta_number
    }

    fn calculate_value_by_index(&self, index: f64) -> f64 {
        let (down_index, down_value) = self.get_down_index_value(index);
        let (up_index, up_value) = self.get_up_index_value(index);
        let delta_index = TblIndexValueData::get_delta(down_index, up_index);
        let delta_value = TblIndexValueData::get_delta(down_value, up_value);

        ((index - down_index) / delta_index) * delta_value + down_value
    }
}

fn main() {
    let config = Config::build(&read_config_file(&CONFIG_FILE_NAME_PATH));

    let tbl_current = TblIndexValueData::fill_tbl_index_value(&read_config_file(&config.current_tbl_path));

    println!("calculate value: {:#?}", tbl_current.calculate_value_by_index(33.08));

    //println!("{:#?}", tbl_current.get_down_index_value(60.0).0);
    //println!("{:#?}", tbl_current.get_down_index_value(60.0).1);
    println!("{:#?}", tbl_current);
    //println!("{:#?}", config);

    get_calculated_data(&config);
}

fn read_config_file(config_path: &str) -> String { 
     let file_content = match fs::read_to_string(&config_path) { 
         Ok(file_content) => file_content, 
         Err(error) => panic!("Read config file error. Invalid configuration file: '{}'. {}", &config_path, error), 
     }; 
  
     file_content 
}

fn get_calculated_data(config: &Config) {
    let tbl_current = TblIndexValueData::fill_tbl_index_value(&read_config_file(&config.current_tbl_path));
    let tbl_resistance = TblIndexValueData::fill_tbl_index_value(&read_config_file(&config.resistance_tbl_path));
    let tbl_specific_heat = TblIndexValueData::fill_tbl_index_value(&read_config_file(&config.specific_heat_tbl_path));
    let tbl_heat_transfer = TblIndexValueData::fill_tbl_index_value(&read_config_file(&config.heat_transfer_tbl_path));

    let A = &config.surface_area / 1000000.0;    //mm^2 to m^2
    let m = &config.weight / 1000.0;             //g to kg
    let Tp = &config.enviroment_temperature;
    let t = &config.pulse_duration / (config.num_of_iterations as f64);
    
    let mut temperature = config.start_sample_temperature;
    let mut dT: f64;
    let mut heating: f64;
    let mut cooling: f64;
    let mut mc: f64;        //= m * c
    
    for i in 0..config.num_of_iterations {
        mc = m * tbl_specific_heat.calculate_value_by_index(temperature);
        heating = ((f64::powf(tbl_current.calculate_value_by_index(temperature), 2.0) * tbl_resistance.calculate_value_by_index(temperature)) / mc) * t;
        cooling = ((A * tbl_heat_transfer.calculate_value_by_index(temperature) * (temperature - Tp)) / mc) * t;
        dT = heating - cooling;
        temperature += dT;
            
        println!("time: {}; temperature: {}; heating: {}; cooling: {}", (t * i as f64), temperature, heating, cooling);
    }
}