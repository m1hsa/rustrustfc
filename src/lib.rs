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
