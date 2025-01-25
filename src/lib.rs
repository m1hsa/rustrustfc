pub fn usage(error: &str) -> String {
    let usage_string = format!(
        "{}, Usage ->
      rustfc [amount] [material] [furnace] [optional furnaces]
      [amount]: Number
      [material]: sulfur or metal or hqm
      [furnace]: furnace or large or electric
      [furnaces]: Number of furnaces",
        error
    );

    // println!("{}", usage_string);
    usage_string
}

pub struct Config {
    pub amount: f32,
    pub material: String,
    pub furnace: String,
    pub furnaces_divisor: f32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let mut furnaces_divisor = 1.0;

        if args.len() < 4 {
            return Err(usage("Not enough arguments"));
        } else if args.len() == 5 {
            furnaces_divisor = match args[4].parse() {
                Ok(num) => num,
                Err(_) => {
                    return Err(usage("Invalid furnace_divisor"));
                }
            };
        }

        let amount: f32 = match args[1].clone().parse() {
            Ok(num) => num,
            Err(_) => {
                return Err(usage("Invalid amount"));
            }
        };

        let material = args[2].clone();
        let furnace = args[3].clone();

        Ok(Config {
            amount,
            material,
            furnace,
            furnaces_divisor,
        })
    }
}

pub fn make_furna(config: Config, time: f32) -> String {
    let remove_furnace_furnace = String::from(if config.furnace != "furnace" {
        config.furnace.as_str()
    } else {
        ""
    });

    let mut furna = format!(
        "for {} {} in {} {remove_furnace_furnace} furnace ",
        config.amount, config.material, config.furnaces_divisor
    );

    if config.furnaces_divisor != 1.0 {
        furna = format!(
            "{furna}s by {am_fr} {} ",
            config.material,
            am_fr = (config.amount / config.furnaces_divisor).round()
        );

        if config.furnace == "electric" {
            furna = format!(
                "{furna}each and {t_fd} power total",
                t_fd = (time / config.furnaces_divisor).round()
            );
        } else {
            furna = format!(
                "{furna}and {t_2} wood each or {t_df_2} wood total",
                t_2 = (time / 2.0).round(),
                t_df_2 = (time * config.furnaces_divisor / 2.0).round()
            );
        }
    } else {
        furna = format!("{furna}filled with ");
        if config.furnace == "electric" {
            furna = format!(
                "{furna}{t_fd} power total",
                t_fd = (time * config.furnaces_divisor).round()
            );
        } else {
            furna = format!("{furna}{t_2} wood", t_2 = (time / 2.0).round());
        }
    };
    furna
}
