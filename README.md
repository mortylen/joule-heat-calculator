# Joule Heat Calculator
********************
image
********************

## Joule Heating Calculation and Cooling via Heat Transfer in Wire.

[Article about this](https://mortylen...)
|
[Donate](https://mortylen...)

* [Title](#Joule-Heat-Calculator)
* [Introduction](#Introduction)
* [Project Layout](#Project-Layout)
* [Mathematics](#Mathematics,-I-have-to-explain-this-a-little-bit.)
[Congiguration files](#Create-congiguration-files)

## Project Layout
```bash
joule-heat/
|--src                     # All source code
|  |--setting                # Necessary data for calculation
|  |  |--current_tbl.csv       # Sample of the current table
|  |  |--heat_transfer_tbl.csv # Sample of the heat transfer table
|  |  |--resistance_tbl.csv    # Sample of the resistance table
|  |  |--specific_heat_tbl.csv # Sample of the spicific heat table
|  |--app_setting.toml     # Sample of the main setting file
|  |--main.rs              # Source code
```

## Introduction
For starters, let me explain the purpose of this project. I aim to develop software that calculates wire heating. As the electric current flows through the wire, it will generate heat, but the surrounding environment will also contribute to temperature dissipation. Therefore, it is essential to calculate the wire's temperature over time, taking into account both the heating from the current and the cooling effects from the environment.

The joule heating calculation application is a console application developed in Rust. The application starts by reading a prepared configuration file, which contains the necessary input parameters for the calculation. Once the configuration file is loaded, the application initiates the calculation process. After the calculation completes, the application stores the calculated results in a file.

The application assumes that the user has pre-calculated the characteristics for a current table, a resistance table, a specific heat table, and a heat transfer table.

The path to this configuration file is either sent as an argument when the application is launched, or it can be directly set within the application.

## Mathematics, I have to explain this a little bit.

I have this mathematical formula:
*********************
img formula
*********************

If I want to track the heating process over time, it is necessary to break down the calculation into numerous small steps or iterations. The more iterations there are, the more accurate and smoother the calculation becomes. In each iteration, the formula will be computed, and the result (deltaT) will be added to the current temperature of the sample.

In the calculation, the following variables are involved:

- I - DC electric current [A] dependet on time
- R - sample resistance [ohm] dependet on temperature
- A - surface area of the sample [m^2]
- h - heat transfer coefficient dependet on temperature
- Ts - initial surface temperature of sample [K]
- Tp - temperature of the environment [K]
- m - weight of the sample [kg]
- c - mass thermal capacity dependet on temperature
- t - iteration time interval [s]
- e - Euler's number (2.718281828459...)

The value of I (DC electric current) will be selected from a table that represents the current values at different points in time. This table will have two columns, one for time and the other for the corresponding current value. Essentially, it represents the waveform of the current pulse.

Similarly, the values for R (resistance), h (heat transfer coefficient), and c (mass thermal capacity) will be selected using a similar approach, but the waveform will depend on temperature instead of time.

The other variables, such as A (surface area) and m (weight), remain constant for a specific sample. Additionally, Ts (initial surface temperature) and Tp (environmental temperature) are initial constants required for the calculation. The time interval, t, is calculated by dividing the pulse duration by the number of iterations. And e (Euler's number) is used for the exponential calculation.

## Create congiguration files
Firstly, it is necessary to consider the configuration file, from which the application will read all the essential parameters required for the calculation. I have chosen the TOML format for this purpose. Within the file, initial values for the calculation will be provided, along with references to tables containing relevant data such as current, resistance, heat transfer coefficients, and mass thermal capacity. These tables will be stored in either TOML or CSV format.

The main setting file looks like this:

app_setting.toml
```bash
# Set resistance table [toml] file path 'resistance_tbl = [{index = 0.0, value=0.0}]' or csv file
resistance_tbl_path = "/home/runner/joule-heat-rust/src/setting/resistance_tbl.toml"
  
# Set specific heat table [toml] file path 'specific_heat_tbl = [{index = 0.0, value=0.0}]' or csv file
specific_heat_tbl_path = "/home/runner/joule-heat-rust/src/setting/specific_heat_tbl.toml"
  
# Set heat transfer table [toml] file path 'heat_transfer_tbl = [{index = 0.0, value=0.0}]' or csv file
heat_transfer_tbl_path = "/home/runner/joule-heat-rust/src/setting/heat_transfer_tbl.toml"

# Set current table [toml] file path 'current_tbl = [{index = 0.0, value=0.0}]' or csv file
current_tbl_path = "/home/runner/joule-heat-rust/src/setting/current_tbl.toml"

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

# Set export directory path
export_path = "/home/runner/joule-heat-rust/src/tets.csv"
```

And tables with data like current, resistance, heat transfer and mass thermal capacity looks like this:

current_tbl.toml
```bash
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
current_tbl.csv
```bash
index, value
1, 10
10, 20
20, 50
30, 100
50, 200
1000, 200
```

- current_tbl
Contains the DC electric current data dependet on time. The 'index' reprezents the time data [ms] and 'value' represents the current value [A].

- resistance_tbl
Contains the resistance of sample dependet on temperature. The 'index' reprezents the temperature data [K] and 'value' represents the resisance value [ohm].

- specific_heat_tbl
Contains the mass thermal capacity dependet on temperature. Heat capacity is a property that describle how much heat energy is required to raise the temperature if a given sample. The 'index' reprezents the temperature data [K] and 'value' represents the heath capacity.

- heat_transfer_tbl
Contains the heat transfer coefficient dependet on temperature. Is the proportionality constant between the heat flux and the thermodynamic driving force for flow of heat. The 'index' reprezents the temperature data [K] and 'value' represents the heat transfer coefficient.
