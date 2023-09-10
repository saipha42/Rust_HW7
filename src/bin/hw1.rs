use clap::{Arg, App};

fn main() {

    let _matches = App::new("Sort")
                                .about("Sort Program")
                                .author("Sai Marn Pha")
                                .version("0.1.0")
                                .arg(
                                    Arg::with_name("list")
                                    .value_name("LIST")
                                    .required(true)
                                    .multiple(true)
                                ).get_matches();
    
    let mut v  = _matches.values_of_lossy("list").unwrap();
    
    let mut res:Vec<i32> = Vec::new();

    for i in v {
        res.push( i.parse::<i32>().unwrap());
    }

    res.sort_by(|a, b| a.cmp(b));
    println!("Ascending order : {:?}", res);
    res.sort_by(|a, b| b.cmp(a));
    println!("Descending order : {:?}", res);
}



