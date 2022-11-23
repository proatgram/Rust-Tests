use std::io::Write;

fn main() {

    let mut temp = String::new();

    print!("What tempature do you want to convert?: ");

    std::io::stdout().flush().expect("Unable to flush stdout.");

    std::io::stdin().read_line(&mut temp).expect("Error: Unable to read from stdin.");

    let temp: f32 = match temp.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => return,
    };

    let temp = convert_celcius(temp);

    println!("Your number is: {temp}");
}

fn convert_celcius(f_number: f32) -> f32 {
    (f_number - 32.0) * (5.0/9.0)
}