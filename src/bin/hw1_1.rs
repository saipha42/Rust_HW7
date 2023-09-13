use std::env;

fn main() {


    let args : Vec<_> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments!");
    }
    
 
    let v = &args[1..];
    let mut res : Vec<i32> = Vec::new();
    for i in v {
        res.push( i.parse::<i32>().unwrap());
    }

    res.sort_by(|a, b| a.cmp(b));
    let asc = res.clone();
    res.sort_by(|a, b| b.cmp(a));
    let desc = res.clone();


    println!("Ascending : {:?}", asc);
    println!("Descending : {:?}", desc);
}



