fn podnies_do_kwadratu(n: i32) -> i32 {
    n * n
}

fn main() {
    //     for i in 0.. {
    //         println!("{:?}", i);
    //     }
    let x = 0..;

    let x = (1..=10).map(podnies_do_kwadratu);

    let v: Vec<_> = (1..=10).map(podnies_do_kwadratu).collect();
    println!("{:?}", v);

    let v: Vec<_> = (1..=10).map(|n| n * n).collect();
    println!("{:?}", v);
}