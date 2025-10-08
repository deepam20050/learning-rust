use proconio::input;

fn main() {
    input! {
        x: String,
        y: String,
    }

    let ocelot = "Ocelot";
    let serval = "Serval";
    let lynx = "Lynx";

    let result = {
        if x == lynx {
            "Yes"
        } else if x == serval {
            if y == lynx {
                "No"
            } else {
                "Yes"
            }
        } else {
            if y == ocelot {
                "Yes"
            } else {
                "No"
            }
        }
    };

    println!("{}", result);
}
