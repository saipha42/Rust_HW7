pub mod tem_table {

    pub fn convert_temparature(t1: i32, t2: i32, step: i32)-> Vec<(f32, f32)> {

        let fahr_start : f32 = t1.to_string().parse().unwrap();
        let fahr_end : f32 = t2.to_string().parse().unwrap();
        let step : f32= step.to_string().parse().unwrap();
        let mut fahr = *&fahr_start;

        let mut temps :Vec<(f32, f32)>= Vec::new();

        loop {
            
            let celcius = (5.0/9.0)*(  fahr  - 32.0);
            let celcius = format!("{:.1}", celcius).parse().unwrap();

            temps.push((fahr, celcius));


            //logic of increasing or decresing the index for the loop
            if fahr_start < fahr_end {

                fahr += step;
                if fahr > fahr_end {
                    break;
                }

            }else if fahr_start > fahr_end {

                fahr -= step;
                if fahr < fahr_end {
                    break;
                }
            }
        }

        return temps;
    }
}

