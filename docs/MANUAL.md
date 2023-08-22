# Joule Heat Calculator v0.1.0

## How to run it? 
Download the latest Release and extract it to your local computer. 
  
### Requirements 
To compile the source code on your system, you must have Rust installed to run the application. See [Install Rust](https://rust-lang.org/tools/install) 
  
### Instructions 
Run the application with a parameter that contains the path to the settings file (by default it is ***app_setting.toml***). 

*Windows:*
```bash
joule_heat_calculator.exe app_setting.toml 
```

*Linux:*
```bash
joule_heat_calculator app_setting.toml 
```
*or:*
```bash
./joule_heat_calculator app_setting.toml 
```
*or, if you don't have permission:*
```bash
sudo chmod +x ./joule_heat_calculator
joule_heat_calculator ./app_setting.toml 
``` 
  
Or run the application without the parameter and the application prompts you for the path to the settings file. 

The calculated data are saved to the file defined in the ***export_path*** parameter in the configuration. The data is saved in ***CSV*** format.

### Note 
Prior to execution, please review the ***app_setting.toml*** file or your designated settings file, and ensure that all file paths are configured in alignment with the current directory. Additionally, adjust all parameters within the file to suit your specific requirements. It's important not to overlook the customization of values within all related tables according to your specific needs.

## Create configuration files 
Firstly, it is necessary to consider the configuration file, from which the application will read all the essential parameters required for the calculation. I have chosen the TOML format for this purpose. Within the file, initial values for the calculation will be provided, along with references to tables containing relevant data such as current, resistance, heat transfer coefficients, and mass thermal capacity. These tables will be stored in either **TOML** or **CSV** format. 
  
The main setting file looks like this: 
  
*app_setting.toml* 
```toml 
# Set resistance table [toml] file path 'resistance_tbl = [{index = 0.0, value=0.0}]' or as CSV file 
resistance_tbl_path = "/home/runner/joule-heat-rust/src/setting/resistance_tbl.csv" 
  
# Set specific heat table [toml] file path 'specific_heat_tbl = [{index = 0.0, value=0.0}]' or as CSV file 
specific_heat_tbl_path = "/home/runner/joule-heat-rust/src/setting/specific_heat_tbl.csv" 
  
# Set heat transfer table [toml] file path 'heat_transfer_tbl = [{index = 0.0, value=0.0}]' or as CSV file 
heat_transfer_tbl_path = "/home/runner/joule-heat-rust/src/setting/heat_transfer_tbl.csv" 
  
# Set current table [toml] file path 'current_tbl = [{index = 0.0, value=0.0}]' or as CSV file 
current_tbl_path = "/home/runner/joule-heat-rust/src/setting/current_tbl.csv" 
  
# Set sample surface area [mm^2] 
surface_area = 70.591586 
  
# Set sample weight [g] 
weight = 0.17037 
  
# Set start sample temperature [K] 
start_sample_temperature = 77.0 
  
# Set enviroment temperature [K] 
enviroment_temperature = 77.0 
  
# Set pulse duration [ms] 
pulse_duration = 1000 
  
# Set number of iterations 
num_of_iterations = 1000 
  
# Set export file path [*.csv]
export_path = "/home/runner/joule-heat-rust/src/tets.csv" 
``` 
  
And tables with data like current, resistance, heat transfer and mass thermal capacity looks like this: 
  
*current_tbl.toml* 
```toml 
index_value_data = [  
{index=1,value=10}, 
{index=10,value=20}, 
{index=20,value=50}, 
{index=30,value=100}, 
{index=50,value=200}, 
{index=1000,value=200} 
] 
``` 
  
or as csv like this: 
  
*current_tbl.csv* 
```csv 
index, value 
1, 10 
10, 20 
20, 50 
30, 100 
50, 200 
1000, 200 
``` 
  
- **current_tbl:** 
Contains the DC electric current data dependet on time. The ***'index'*** reprezents the time data **[ms]** and ***'value'*** represents the current value **[A]**. 
  
- **resistance_tbl:** 
Contains the resistance of sample dependet on temperature. The ***'index'*** reprezents the temperature data **[K]** and ***'value'*** represents the resisance value **[ohm]**. 
  
- **specific_heat_tbl:** 
Contains the mass thermal capacity dependet on temperature. Heat capacity is a property that describle how much heat energy is required to raise the temperature if a given sample. The ***'index'*** reprezents the temperature data **[K]** and ***'value'*** represents the heath capacity. 
  
- **heat_transfer_tbl:** 
Contains the heat transfer coefficient dependet on temperature. Is the proportionality constant between the heat flux and the thermodynamic driving force for flow of heat. The ***'index'*** reprezents the temperature data **[K]** and ***'value'*** represents the heat transfer coefficient.
