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
    
    let v  = _matches.values_of_lossy("list").unwrap();
    
    let mut res:Vec<i32> = Vec::new();

    for i in v {
        res.push( i.parse::<i32>().unwrap());
    }

    for _ in 0..res.len() {

        for k in 0..(res.len() - 1) {

            let x = res[k];
            let y = res[k+1];
            if x > y {
                res.swap(k, k+1);
            }
        }
    }

    println!("Ascending order : {:?}", res);

    for _ in 0..res.len() {

        for k in 0..(res.len() - 1) {

            let x = res[k];
            let y = res[k+1];
            if y > x {
                res.swap(k, k+1);
            }
        }
    }

    println!("Descending order : {:?}", res);
}



