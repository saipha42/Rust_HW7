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
    if v.len()%2 != 0 {
         v = v[0..v.len()-1].to_vec();
    }

    let mut res : Vec<(i32, i32)> = Vec::new();
    for i in (0..v.len()-1).step_by(2) {
        let x = v[i].parse::<i32>().unwrap();
        let y = v[i+1].parse::<i32>().unwrap();

        res.push( (x, y) );

    }

    //Ascending sort
    res.sort_by(|a, b| a.0.cmp(&b.0));
    
    for i in 0..res.len() {
        let mut v = vec![res[i].0, res[i].1];
        v.sort_by(|a, b| a.cmp(b));
        res[i] = (v[0], v[1]);
    }

    println!("Ascending Order {:?}", res);


    //Descending sort
    res.sort_by(|a, b| b.0.cmp(&a.0));
    for i in 0..res.len() {
        let mut v = vec![res[i].0, res[i].1];
        v.sort_by(|a, b| a.cmp(b));
        res[i] = (v[1], v[0]);
    }
    println!("Descending Sort  {:?}", res);
    
}
