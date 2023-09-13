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

    for _ in 0..res.len() {

        for k in 0..(res.len() - 1) {

            let x = res[k];
            let y = res[k+1];
            if x > y {
                res.swap(k, k+1);
            }
        }
    }

    let asc = res.clone();

    for _ in 0..res.len() {

        for k in 0..(res.len() - 1) {

            let x = res[k];
            let y = res[k+1];
            if y > x {
                res.swap(k, k+1);
            }
        }
    }

    let desc = res.clone();
    println!("Ascending : {:?}", asc);
    println!("Descending : {:?}", desc);
}



