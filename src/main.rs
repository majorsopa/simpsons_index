fn main() {
    let (n_total, ni_total): (u64, u64) = {
        let (mut n_total, mut ni_total) = (0_u64, 0_u64);
        let mut i = 0;
        println!("Enter e to exit");
        loop {
            println!("Number of species {}:", i);
            i += 1;
            let mut n = String::new();
            std::io::stdin().read_line(&mut n).unwrap();

            let n = n.trim();
            if n == "e" {
                break;
            }

            let n = n.parse::<u64>().unwrap();
            n_total += n;
            ni_total += n * (n - 1);
        }
        (n_total * (n_total - 1), ni_total)
    };


    let p = ni_total as f64 / n_total as f64;

    println!("Simpson's Index: {}", 1.0 - p);
}
