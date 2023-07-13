#[derive(Debug)]
struct SpecHeat {
    temperature: f64,
    specific_heat: f64,
}

fn main() {
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
    
    get_down(&tbl_spec_heat);
    get_up(&tbl_spec_heat);
}

fn get_down(tbl_data: &Vec<SpecHeat>) {
    let down_number: Vec<&SpecHeat> = tbl_data
        .into_iter()
        .filter(|n| n.temperature <= 19.0)
        .collect();
    
    println!("{:#?}", down_number.last());
}

fn get_up(tbl_data: &Vec<SpecHeat>) {
    let up_number: Vec<&SpecHeat> = tbl_data
        .into_iter()
        .filter(|n| n.temperature >= 2.0)
        .collect();
    
    println!("{:#?}", up_number.first());
}
 