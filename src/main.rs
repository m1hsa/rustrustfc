use std::collections::HashMap;
use std::env;

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

    let mut furnaces_divisor: f32 = 1.0;

    // args
    let args: Vec<String> = env::args().collect();

    // error handling
    if args.len() < 4 {
        usage("Not enough arguments");
        return;
    } else if args.len() == 5 {
        furnaces_divisor = match args[4].parse() {
            Ok(num) => num,
            Err(_) => {
                usage("Invalid furnace_divisor");
                return;
            }
        };
    }

    let ammount: f32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            usage("Invalid ammount");
            return;
        }
    };

    let material = args[2].as_str();
    let furnace = args[3].as_str();

    if !materials.contains_key(material) || !furnaces.contains_key(furnace) {
        usage("Invalid material or furnace");
        return;
    }
    // error handeling end

    let time = ammount * materials[material] / furnaces[furnace] / furnaces_divisor;

    // string making
    let remove_furnace_furnace = String::from(if furnace != "furnace" { furnace } else { "" });

    let mut furna =
        format!("for {ammount} {material} in {furnaces_divisor}{remove_furnace_furnace} furnace ",);

    if furnaces_divisor != 1.0 {
        furna = format!(
            "{furna}s by {am_fr} {material} ",
            am_fr = (ammount / furnaces_divisor).round()
        );

        if furnace == "electric" {
            furna = format!(
                "{furna}each and {t_fd} power total",
                t_fd = (time / furnaces_divisor).round()
            );
        } else {
            furna = format!(
                "{furna}and {t_2} wood each or {t_df_2} wood total",
                t_2 = (time / 2.0).round(),
                t_df_2 = (time * furnaces_divisor / 2.0).round()
            );
        }
    } else {
        furna = format!("{furna}filled with ");
        if furnace == "electric" {
            furna = format!(
                "{furna}{t_fd} power total",
                t_fd = (time * furnaces_divisor).round()
            );
        } else {
            furna = format!("{furna}{t_2} wood", t_2 = (time / 2.0).round());
        }
    }

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

fn usage(error: &str) {
    let usage_string = format!(
        "{}, Usage ->
        rustfc [ammount] [material] [furnace] [optional furnaces]
        [ammount]: Number
        [material]: sulfur or metal or hqm
        [furnace]: furnace or large or electric
        [furnaces]: Number of furnaces",
        error
    );

    println!("{}", usage_string);
}
