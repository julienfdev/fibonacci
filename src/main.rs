use std::cmp::Ordering;
use std::io;

fn main() {
    let (mut vm1, mut v0) = (0, 0);

    let mut bound = String::new();
    println!("Which fibonacci number ?");
    io::stdin()
        .read_line(&mut bound)
        .expect("Error reading line");

    let bound: i32 = bound.trim().parse().expect("Not a number, crashing...");

    for _ in 0..bound {
        fibo_add(&mut vm1, &mut v0);
    }

    println!("{bound} terme de fibonacci : {vm1}");
}

fn fibo_add(vm1: &mut u128, v0: &mut u128) -> u128 {
    match (*v0).cmp(&0) {
        Ordering::Less => 0,
        Ordering::Equal => {
            *v0 = 1;
            1
        }
        Ordering::Greater => {
            let return_val = *v0 + *vm1;
            *vm1 = *v0;
            *v0 = return_val;
            return_val
        }
    }
}
