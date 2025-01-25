use std::collections::HashMap;
use std::env;
use std::process;

use rustrustfc::Config;

fn main() {
    println!();

    // dicks
    let materials: HashMap<&str, f32> = [
        ("sulfur", 1.0 / 3.0),
        ("metal", 2.0 / 3.0),
        ("hqm", 4.0 / 3.0),
    ]
    .into_iter()
    .collect();

    let furnaces: HashMap<&str, f32> = [
        ("furnace", 1.0 / 5.0),
        ("large", 1.0),
        ("electric", 1.0 / 3.0),
    ]
    .into_iter()
    .collect();

    // args
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("While parsing config error ocured: {err}");
        process::exit(1);
    });

    // error handeling that i dont want to do in the Config::new
    if !materials.contains_key(config.material.as_str())
        || !furnaces.contains_key(config.furnace.as_str())
    {
        rustrustfc::usage("Invalid material or furnace");
        return;
    }
    // error handeling end

    let time = config.amount * materials[config.material.as_str()]
        / furnaces[config.furnace.as_str()]
        / config.furnaces_divisor;

    // string making
    let furna = rustrustfc::make_furna(config, time);
    // string making end

    if time < 60.0 {
        println!("{} seconds {furna}", time.round());
    } else if time < 3600.0 {
        println!(
            "{} minutes {} seconds {furna}",
            (time / 60.0).floor(),
            (time % 60.0).round()
        );
    } else {
        println!(
            "{} hours {} minutes {} seconds {furna}",
            (time / 3600.0).floor(),
            (time % 3600.0 / 60.0).floor(),
            (time % 3600.0 % 60.0).round()
        );
    }
}
