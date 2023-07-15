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

//TODO is not table, but ROW
#[derive(Deserialize)]
#[derive(Debug)]
struct TblIndexValue {
    index: f64,
    value: f64,
}

//TODO TblCurrent prepisat na vseobecne + pridat get_down a get_up
#[derive(Deserialize)]
#[derive(Debug)]
struct TblCurrent {
    current_tbl: Vec<TblIndexValue>,
}

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
    tbl_test: Vec<SpecHeat>,
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

fn fill_tbl_index_value(file_content: &String) -> TblCurrent {
    let tbl: TblCurrent = match toml::from_str(&file_content) { 
            Ok(tbl) => tbl, 
            Err(error) => panic!("Problem parsing config file: {}", error), 
        }; 
        tbl 
}

//TODO: delete struct SpecHeat
#[derive(Deserialize)]
#[derive(Debug)]
struct SpecHeat {
    temperature: f64,
    specific_heat: f64,
}

fn main() {
    let config = Config::build(&read_config_file(&CONFIG_FILE_NAME_PATH));


    
   let tbl_current = fill_tbl_index_value(&read_config_file(&config.current_tbl_path));

    println!("{:#?}", tbl_current);
    println!("{:#?}", config);
    
    let mut tbl_spec_heat: Vec<SpecHeat> = Vec::new();
    
    let spec_heat_row01 = SpecHeat {
        temperature: 1.0,
        specific_heat: 10.11,
    };
    
    let spec_heat_row02 = SpecHeat {
        temperature: 5.0,
        specific_heat: 15.55,
    };
    
    let spec_heat_row03 = SpecHeat {
        temperature: 10.0,
        specific_heat: 20.22,
    };
    
    let spec_heat_row04 = SpecHeat {
        temperature: 20.0,
        specific_heat: 33.33,
    };
    
    tbl_spec_heat.push(spec_heat_row01);
    tbl_spec_heat.push(spec_heat_row02);
    tbl_spec_heat.push(spec_heat_row03);
    tbl_spec_heat.push(spec_heat_row04);
    
    println!("{:#?}", tbl_spec_heat);
    
    //get_down(&tbl_spec_heat);
    //get_up(&tbl_spec_heat);

    println!("{:#?}", get_spec_heat_from_vec(&tbl_spec_heat, 0.9));
    //get_spec_heat_from_vec(&tbl_spec_heat, 6.0);
}

fn read_config_file(config_path: &str) -> String { 
     let file_content = match fs::read_to_string(&config_path) { 
         Ok(file_content) => file_content, 
         Err(error) => panic!("Read config file error. Invalid configuration file: '{}'. {}", &config_path, error), 
     }; 
  
     file_content 
}

fn get_down_index_value(tbl_data: &Vec<TblIndexValue>, index: f64) -> (f64, f64) {
    let (down_index, down_value) = match tbl_data.iter().find(|&x| x.index <= index) {
        Some(value) => (value.index, value.value),
        None => (tbl_data.first().unwrap().index, tbl_data.first().unwrap().value),
    };
    (down_index, down_value)
}

fn get_up_index_value(tbl_data: &Vec<TblIndexValue>, index: f64) -> (f64, f64) {
    let (up_index, up_value) = match tbl_data.iter().find(|&x| x.index >= index) {
        Some(value) => (value.index, value.value),
        None => (tbl_data.last().unwrap().index, tbl_data.last().unwrap().value),
    };
    (up_index, up_value)
}


fn get_spec_heat_from_vec(tbl_data: &Vec<SpecHeat>, temperature: f64) -> Option<f64> {
    let (down_temp, down_heat) = match tbl_data.iter().find(|&x| x.temperature <= temperature) {
        Some(spec_heat) => (spec_heat.temperature, spec_heat.specific_heat),
        None => (tbl_data.first()?.temperature, tbl_data.first()?.specific_heat),
        //None => return None,
    };

    let (up_temp, up_heat) = match tbl_data.iter().find(|&x| x.temperature >= temperature) {
        Some(spec_heat) => (spec_heat.temperature, spec_heat.specific_heat),
        None => (tbl_data.last()?.temperature, tbl_data.last()?.specific_heat),
        //None => return None,
    };

    
    let delta_temperature = if up_temp == down_temp {
        down_temp
    } else {
        up_temp - down_temp
    };

    let delta_specifi_heat = if up_heat == down_heat {
        down_heat
    } else {
        up_heat - down_heat
    };

    let res = ((temperature - down_temp) / delta_temperature) * delta_specifi_heat + down_heat;

    Some(res)
}



//fn get_down(tbl_data: &Vec<SpecHeat>) {
//    let down_number: Vec<&SpecHeat> = tbl_data
//        .into_iter()
//        .filter(|n| n.temperature <= 19.0)
//        .collect();   
//    println!("{:#?}", down_number.last());
//}

//fn get_up(tbl_data: &Vec<SpecHeat>) {
//    let up_number: Vec<&SpecHeat> = tbl_data
//        .into_iter()
//        .filter(|n| n.temperature >= 2.0)
//        .collect();   
//    println!("{:#?}", up_number.first());
//}
 