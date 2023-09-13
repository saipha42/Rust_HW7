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

    let mut list : Vec<(f32, f32)> = Vec::new();
    for i in (0..v.len()).step_by(2) {
        let x = v[i].parse::<f32>().unwrap();
        let y = v[i+1].parse::<f32>().unwrap();

        list.push( (x, y) );
    }

    let list = BSort {list: list};

    let asc = list.asc();
    let desc = list.desc();

    println!("Ascending : {:?}", asc);
    println!("Descending : {:?}", desc);
    
}




struct BSort {
    list : Vec<(f32, f32)>
}

impl BSort {
    fn asc (&self) -> Vec<(f32, f32)> {

        let mut list = self.list.clone();
        self.sort(&mut list, 1)
    }

    fn desc (&self) -> Vec<(f32, f32)> {

        let mut list = self.list.clone();
        self.sort(&mut list, -1)
    }

    fn sort(&self, list : &mut Vec<(f32, f32)>, direction : i32) -> Vec<(f32, f32)> {

        let status = direction > 0; // true for ascending and false for descending
        for _ in 0..list.len() {

            for k in 0..(list.len() - 1) {
    
                let ax = list[k].0;
                let bx = list[k+1].0;
                if (ax > bx) == status {
                    list.swap(k, k+1);
                }
            }
        }
        
        for _ in 0..list.len() {
    
            for k in 0..(list.len() - 1) {
                
                let ax = list[k].0;
                let bx = list[k+1].0;
                let ay = list[k].1;
                let by = list[k+1].1;
                if (ay > by) == status && ax == bx {
                    list.swap(k, k+1);
                }
            }
        }

        list.to_owned()

        
    }
}