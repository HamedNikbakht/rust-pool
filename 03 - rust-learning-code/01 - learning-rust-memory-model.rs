enum Light {
    Bright,
    Dull,
}

fn light_info(light: Light) {
    match light {
        Light :: Bright => println!("bright"),
        Light :: Dull => println!("dull"),
    }
}

fn main() {
    let dull = Light :: Dull;
    light_info(dull);
    light_info(dull);
}
