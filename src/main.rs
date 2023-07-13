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
    
    //get_down(&tbl_spec_heat);
    //get_up(&tbl_spec_heat);

    println!("{:#?}", get_spec_heat_from_vec(&tbl_spec_heat, 0.9));
    //get_spec_heat_from_vec(&tbl_spec_heat, 6.0);
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
 