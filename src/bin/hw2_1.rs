use std::env;

fn main() {
    let args : Vec<_> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments!");
    }
    
    let mut v: Vec<String> = args[1..].to_vec();

    if v.len()%2 != 0 {
         v = v[0..v.len()-1].to_vec();
    }

    let mut res : Vec<(f32, f32)> = Vec::new();
    for i in (0..v.len()).step_by(2) {
        let x = v[i].parse::<f32>().unwrap();
        let y = v[i+1].parse::<f32>().unwrap();

        res.push( (x, y) );

    }

    //Ascending sort
    res.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.partial_cmp(&b.0).unwrap()
        }else {
            a.1.partial_cmp(&b.1).unwrap()
        }
    });
    let asc = res.clone();

    //Descending sort
    res.sort_by(|a, b| {
        if a.0 != b.0 {
            b.0.partial_cmp(&a.0).unwrap()
        }else {
            b.1.partial_cmp(&a.1).unwrap()
        }
    });
    let desc = res.clone();

    println!("Ascending : {:?}", asc);
    println!("Descending : {:?}", desc);
    
}
