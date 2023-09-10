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
    for _ in 0..res.len() {

        for k in 0..(res.len() - 1) {

            let x = res[k].0;
            let y = res[k+1].0;
            if x > y {
                res.swap(k, k+1);
            }
        }
    }
    
    for i in 0..res.len() {

        let mut v = vec![res[i].0 , res[i].1];
        if v[0] > v[1] {
            v.swap(0, 1);
        }
        res[i] = (v[0], v[1]);
    }

    println!("Ascending Order {:?}", res);


    //Descending sort
    for _ in 0..res.len() {

        for k in 0..(res.len() - 1) {

            let x = res[k].0;
            let y = res[k+1].0;
            if y > x {
                res.swap(k, k+1);
            }
        }
    }
    
    for i in 0..res.len() {

        let mut v = vec![res[i].0 , res[i].1];
        if v[0] < v[1] {
            v.swap(0, 1);
        }
        res[i] = (v[0], v[1]);
    }
    println!("Descending Sort  {:?}", res);
    
}
