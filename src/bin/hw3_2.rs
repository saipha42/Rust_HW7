use clap::{App, Arg};
use hw8::tem_table::convert_temparature; //import the function from lib.rs

fn main () {

    let _matches = App::new("temparature")
            .author("Sai Marn Pha")
            .version("0.1.0")
            .about("This program converts a range of temperature in fahr to celcius")
            .arg(
                Arg::with_name("tem_1")
                .value_name("TEMPATURE_1")
                .required(true)
                .index(1)
            )
            .arg(
                Arg::with_name("tem_2")
                .value_name("TEMPATURE_2")
                .required(true)
                .index(2)
            )
            .arg(
                Arg::with_name("step")
                .value_name("STEP")
                .required(true)
                .index(3)
                .validator(step_validator)
            )
            .setting(clap::AppSettings::AllowNegativeNumbers)
            .get_matches();

    let tem_1 = _matches.value_of("tem_1").unwrap().parse::<i32>().unwrap();
    let tem_2 = _matches.value_of("tem_2").unwrap().parse::<i32>().unwrap();
    let step = _matches.value_of("step").unwrap().parse::<i32>().unwrap();

    let temps = convert_temparature(tem_1, tem_2, step);

    let mut table_content = String::new();
    table_content.push_str("<style>table, td, th {border: 1px solid #000000;\
    border-collapse : collapse;\
    padding : 6px;\
    }</style>");
    table_content.push_str("<table>\
    <tr>\
        <th>x</th>\
        <th>x^2</th>\
        <th>x^3</th>\
    </tr>\
    ");

    for i in 0..temps.len() {
        let x = temps[i].1;
        let x2 = x*x;
        let x3 = x*x*x;
        let row = format!("<tr> <td>{}</td>  <td>{}</td>  <td>{}</td> </tr>", x, x2, x3);
        table_content.push_str(&row);
    }

    table_content.push_str("</table>");

    println!("{}", table_content);

}

//check if the step input valid for preventing infinite loop
fn step_validator(step : String) -> Result<(), String> {

    let step = step.parse::<i32>().unwrap();
    if step <= 0 {
        return  Err(String::from("Invalid step argument"));
    }
    return  Ok(());
}

