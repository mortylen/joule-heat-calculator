use std::io;
use std::path::Path;
use std::fs; 
use std::error::Error;
use std::env;
use serde::Deserialize; 
use toml; 
use csv::Writer;
use chrono::Utc; 
  
//const CONFIG_FILE_NAME_PATH: &str = "/home/runner/joule-heat-rust/src/app_setting.toml"; 
//static mut CONFIG_FILE_NAME_PATH: &str = "app_setting.toml"; 
const CONFIG_FILE_NAME_PATH: &str = "app_setting.toml"; 

#[derive(Deserialize)]
#[derive(Debug)]
struct Config { 
    resistance_tbl_path: String, 
    specific_heat_tbl_path: String,
    heat_transfer_tbl_path: String,
    current_tbl_path: String,
    export_path: String,
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
        let mut tbl_data = TblIndexValueData {
                index_value_data: Vec::new(),
        };
        if file_content.starts_with("index_value_data") {                //toml file
            tbl_data = match toml::from_str(&file_content) { 
                Ok(tbl_data) => tbl_data, 
                Err(error) => panic!("Problem parsing config file: {}", error), 
            }; 
        }
        else if file_content.starts_with("index") {                      //csv file
            let mut reader = csv::Reader::from_reader(file_content.as_bytes());
            for result in reader.deserialize::<RowIndexValue>() {
                match result {
                    Ok(record) => tbl_data.index_value_data.push(record),
                    Err(err) => {
                        eprintln!("Error while deserializing row: {}", err);
                    }
                }
            }
        }
        tbl_data
    }

    fn get_down_index_value(&self, index: f64) -> (f64, f64) {
        let (down_index, down_value) = match self.index_value_data.iter().find(|&x| x.index <= index) {
            Some(value) => (value.index, value.value),
            None => (self.index_value_data.first().unwrap().index, self.index_value_data.first().unwrap().value),
        };
    (down_index, down_value)
    }

    //fn get_down_index_value(&self, index: f64) -> Option<(f64, f64)> {
    //    match self.index_value_data.iter().find(|&x| x.index <= index) {
    //        Some(value) => Some((value.index, value.value)),
    //        None => {
    //            if let Some(first_value) = self.index_value_data.first() {
    //                Some((first_value.index, first_value.value))
    //            } else {
    //                None
    //            }
    //        }
    //    }
    //}

    fn get_up_index_value(&self, index: f64) -> (f64, f64) {
        let (up_index, up_value) = match self.index_value_data.iter().find(|&x| x.index >= index) {
            Some(value) => (value.index, value.value),
            None => (self.index_value_data.last().unwrap().index, self.index_value_data.last().unwrap().value),
        };
    (up_index, up_value)
    }

    //fn get_up_index_value(&self, index: f64) -> Option<(f64, f64)> {
    //    match self.index_value_data.iter().find(|&x| x.index >= index) {
    //        Some(value) => Some((value.index, value.value)),
    //        None => {
    //            if let Some(last_value) = self.index_value_data.last() {
    //                Some((last_value.index, last_value.value))
    //            } else {
    //                None
    //            }
    //        }
    //    }
    //}

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

struct ExportData {
    time: f64,
    temperature: f64,
    heating: f64,
    cooling: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config_file_path: String = if args.len() > 1 {
        args[1].to_string()
    } else {
        CONFIG_FILE_NAME_PATH.to_string()
    };
    
    //println!("{}", config_file_path);
    
    //Set config
    let config = match read_config_file(&set_application(&config_file_path)) {
        Ok(file_content) => Config::build(&file_content),
        Err(error) => {
            println!("Error reading config file: {}. {}", &config_file_path, error);
            io::stdin().read_line(&mut String::new()).unwrap();
            panic!("Application terminate.");
        } 
    };
    //let config = Config::build(&read_config_file(&set_application(&config_file_path)));

    //Run calculation
    let calculated_data: Vec<ExportData> = match get_calculated_data(&config) {
        Ok(data) => {
            data
        }
        Err(error) => {
            println!("Calculation error: {}", error);
            io::stdin().read_line(&mut String::new()).unwrap();
            panic!("Application terminate.");
            //vec![ExportData {
            //    time: 0.0,
            //    temperature: 0.0,
            //    heating: 0.0,
            //    cooling: 0.0,
            //}]
        }
    };
    //let calculated_data = get_calculated_data(&config)?;

    //Export data to CSV file
    if let Err(error) = export_data_to_csv(&calculated_data, &config) {
        println!("Error exporting data to CSV, check 'export_path' value in setting.toml file. {}", error);
    } else {
        println!("Data exported successfully!");
    }
    
    //export_data_to_csv(&calculated_data, &config)?;

    let mut user_input = String::new();
    println!("Complete... Press any key to close.");
    io::stdin().read_line(&mut user_input).unwrap();
    
    Ok(())
}

//fn read_config_file(config_path: &str) -> String { 
//     let file_content = match fs::read_to_string(&config_path) { 
//         Ok(file_content) => file_content, 
//         Err(error) => panic!("Read config file error. Invalid configuration file: '{}'. {}", &config_path, error), 
//     }; 
  
//     file_content 
//}

fn read_config_file(config_path: &str) -> Result<String, io::Error> {
    match fs::read_to_string(&config_path) {
        Ok(file_content) => Ok(file_content),
        Err(error) => Err(error),
    }
}

//fn set_application() -> String {
//    if Path::new(&CONFIG_FILE_NAME_PATH).exists() {
//        let mut user_input = String::new();
//        println!("Load data settings from: {} [Y/N]", &CONFIG_FILE_NAME_PATH);
//        io::stdin().read_line(&mut user_input).unwrap();
//        if user_input.trim().to_lowercase() == "y" {
//            CONFIG_FILE_NAME_PATH.to_string()
//        } else {
//            get_user_input_path()
//        }
//    } else {
//        get_user_input_path()
//    }
//}

fn set_application(config_file_path: &String) -> String {
    if Path::new(&config_file_path).exists() {
        let mut user_input = String::new();
        println!("Load data settings from: {} [Y/N]", &config_file_path);
        io::stdin().read_line(&mut user_input).unwrap();
        if user_input.trim().to_lowercase() == "y" {
            config_file_path.to_string()
        } else {
            get_user_input_path()
        }
    } else {
        get_user_input_path()
    }
}

fn get_user_input_path() -> String {
    let mut user_input = String::new();
    println!("Enter setting file path: ");
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

fn get_calculated_data(config: &Config) -> Result<Vec<ExportData>, Box<dyn Error>> {
    let tbl_current = TblIndexValueData::fill_tbl_index_value(&read_config_file(&config.current_tbl_path)?);
    let tbl_resistance = TblIndexValueData::fill_tbl_index_value(&read_config_file(&config.resistance_tbl_path)?);
    let tbl_specific_heat = TblIndexValueData::fill_tbl_index_value(&read_config_file(&config.specific_heat_tbl_path)?);
    let tbl_heat_transfer = TblIndexValueData::fill_tbl_index_value(&read_config_file(&config.heat_transfer_tbl_path)?);

    let e = 2.71828182845904523536;                           //Euler's number
    let A = &config.surface_area / 1000000.0;                 //Surface area [m^2]
    let m = &config.weight / 1000.0;                          //Weight [kg]
    let Tp = &config.enviroment_temperature;                  //Temperature of environment
    let dTime = (&config.pulse_duration / 1000.0) / (config.num_of_iterations as f64);    //Delta time [s]
    
    let mut temperature = config.start_sample_temperature;    //Temperature of sample
    let mut dT: f64;                                          //Delta temperature
    let mut heating: f64;                                     //= ((current^2 * resistance) / mc) * dTime
    let mut cooling: f64;                                     //= ((Ah * (temperature - Tp)) / mc) * dTime
    let mut tau_euler_coef: f64;                              //= 1-e^(-tAh / mc)
    let mut mc: f64;                                          //= m * specific_heat
    let mut Ah: f64;                                          //= A * heat_transfer
    let mut time: f64;                                        //= dTime * i

    let mut export_data: Vec<ExportData> = Vec::new();
   
    for i in 0..config.num_of_iterations {
        time = dTime * i as f64;
        mc = m * tbl_specific_heat.calculate_value_by_index(temperature);
        Ah = A * tbl_heat_transfer.calculate_value_by_index(temperature);
        heating = ((f64::powf(tbl_current.calculate_value_by_index(temperature), 2.0) * tbl_resistance.calculate_value_by_index(temperature)) / mc) * dTime;
        cooling = ((Ah * (temperature - Tp)) / mc) * dTime;
        tau_euler_coef = 1.0 - f64::powf(e, -((Ah * time) / mc));
        
        dT = heating - (cooling * tau_euler_coef);
        temperature += dT;
            
        //println!("time: {}; temperature: {}; heating: {}; cooling: {}", (time), temperature, heating, cooling);
        
        export_data.push(ExportData {
            time,
            temperature,
            heating,
            cooling
        });
    }

    Ok(export_data)
}

fn export_data_to_csv(data: &[ExportData], config: &Config)  -> Result<(), io::Error> {
    let datetime_now: String = Utc::now().format("_%Y%m%d-%H%M%S").to_string();
    let writer_result = Writer::from_path(&config.export_path.replace(".csv",  &(datetime_now + ".csv")));
    let mut writer = writer_result?;

    writer.write_record(&["Time", "Temperature", "Heating", "Cooling"])?;

    for item in data {
        writer.serialize((
            item.time,
            item.temperature,
            item.heating,
            item.cooling,
        ))?;
    }

    writer.flush()?;
    Ok(())
}