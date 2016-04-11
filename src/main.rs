use std::env::args;

fn main() {
    println!(r"                  _  /)");
    println!(r"                 mo / )");
    println!(r"                 |/)\)");
    println!(r"                  /\_");
    println!(r"                  \__|=");
    println!(r"                 (    )");
    println!(r"                 __)(__");
    println!(r"           _____/      \\_____");
    println!(r"          |  _     ___   _   ||");
    println!(r"          | | \     |   | \  ||");
    println!(r"          | |  |    |   |  | ||");
    println!(r"          | |_/     |   |_/  ||");
    println!(r"          | | \     |   |    ||");
    println!(r"          | |  \    |   |    ||");
    println!(r"          | |   \. _|_. | .  ||");
    println!(r"          |                  ||");
    println!(r"          |  {            }  ||", tombstone_label());
    println!(r"          |                  ||");
    println!(r"  *       | *   **    * **   |**      **");
    println!(r"   \))ejm97/.,(//,,..,,\||(,,.,\\,.((//");
}

fn tombstone_label() -> String {
    let args = make_string(args().skip(1).collect());
    if args.len() <= 14 && args.len() > 0 {
        center_pad(&args, 14)
    } else if args.len() == 0 {
        center_pad("", 14)
    } else {
        center_pad("tl;dw", 14)
    }
}

fn make_string(strs: Vec<String>) -> String {
    let mut res = String::new();
    for s in strs {
        res.push_str(&s);
        res.push(' ');
    }
    if res.len() > 0 {
        let len = res.len() - 1;
        res.truncate(len);
    }
    res
}

fn center_pad(s: &str, len: usize) -> String {
    let mut res = s.to_owned();
    while res.len() < len {
        res.insert(0, ' ');
        res.push(' ');
    }
    res.truncate(len);
    res
}
