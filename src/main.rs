use rust_cidrange::build_net;

fn main() {
    let ip_checker = build_net!{
        "1.0.1.0/24",
        "1.0.2.0/23",
        "1.0.8.0/21"
    };
    // let ip_checker = build_net!("1.0.2.1/31");

    println!("{}", ip_checker.check(1,0,2,1));
}
