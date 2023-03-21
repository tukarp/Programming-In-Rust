fn f1(x:f64) -> f64 {
    x*x-2.0*x+7.0
}

fn f2(x:f64) -> f64 {
    x-27.0
}

fn tablicuj(f:fn(f64)->f64, mut a:f64, b:f64, h:f64) {
    println!("==================================");
    while a <=b {
        println!("f({}) = {}", a, f(a));
        a += h;
    }
    println!("==================================");
}

fn main() {
    tablicuj(f1, -0.1, 0.1, 0.01);
    tablicuj(f2, 0.0, 10.0, 2.5);
}
