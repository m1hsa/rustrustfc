use std::collections::HashMap;
use std::env;

fn main() {
    println!();

    // dicks
    let mut materials: HashMap<&str, f32> = HashMap::new();
    materials.insert("sulfur", 1.0 / 3.0);
    materials.insert("metal", 2.0 / 3.0);
    materials.insert("hqm", 4.0 / 3.0);

    let mut furnances: HashMap<&str, f32> = HashMap::new();
    furnances.insert("furnance", 1.0 / 5.0);
    furnances.insert("large", 1.0);
    furnances.insert("electric", 1.0 / 3.0);

    let mut furnances_divisor: f32 = 1.0;

    // args
    let args: Vec<String> = env::args().collect();

    // test args
    // for (arg, index) in args.iter().enumerate() {
    //     println!("{}: {}", index, arg);
    // }

    // error handling
    if args.len() < 4 {
        usage("Not enough arguments");
        return;
    } else if args.len() == 5 {
        furnances_divisor = match args[4].parse() {
            Ok(num) => num,
            Err(_) => {
                usage("Invalid furnance_divisor");
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
    let furnance = args[3].as_str();

    if !materials.contains_key(material) || !furnances.contains_key(furnance) {
        usage("Invalid material or furnance");
        return;
    }
    // error handeling end

    let time = ammount * materials[material] / furnances[furnance] / furnances_divisor;

    // string making
    let remove_furnance_furnance =
        format!("{}", if furnance != "furnance" { furnance } else { "" });

    let mut furna = format!(
        "for {ammount} {material} in {furnances_divisor}{remove_furnance_furnance} furnance ",
    );

    if furnances_divisor != 1.0 {
        furna = format!(
            "{furna}s by {am_fr} {material} ",
            am_fr = (ammount / furnances_divisor).round()
        );

        if furnance == "electric" {
            furna = format!(
                "{furna}each and {t_fd} power total",
                t_fd = (time / furnances_divisor).round()
            );
        } else {
            furna = format!(
                "{furna}and {t_2} wood each or {t_df_2} wood total",
                t_2 = (time / 2.0).round(),
                t_df_2 = (time * furnances_divisor / 2.0).round()
            );
        }
    } else {
        furna = format!("{furna}filled with ");
        if furnance == "electric" {
            furna = format!(
                "{furna}{t_fd} power total",
                t_fd = (time * furnances_divisor).round()
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

    // println!("Time: {} seconds", time);
    // colored for colors
}

fn usage(error: &str) {
    let usage_string = format!(
        "{}, Usage ->
        rustfc [ammount] [material] [furnance] [optional furnances]
        [ammount]: Number
        [material]: sulfur or metal or hqm
        [furnance]: furnance or large or electric
        [furnances]: Number of furnances",
        error
    );

    println!("{}", usage_string);
}
