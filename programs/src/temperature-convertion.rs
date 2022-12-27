use std::io;
use std::io::Write;

fn flush() {
    io::stdout().flush().unwrap();
}

fn main() {
    println!("1. Fahrenheit -> Celcius");
    println!("2. Celcius -> Fahrenheit");

    print!("Choose [1, 2]: ");
    flush();
    loop {
        let mut selected_option = String::new();
        io::stdin()
            .read_line(&mut selected_option)
            .expect("Failed to read line");
        let selected_option: u8 = match selected_option.trim().parse() {
            Ok(num) if num == 1 || num == 2 => num,
            Ok(_) | Err(_) => {
                handle_invalide_option();
                continue;
            }
        };
        if selected_option == 1 {
            println!("");
            println!("Fahrenheit -> Celcius");
            print!("Enter temperature °F: ");
            flush();
            let fahrenheit = read_temperature();
            let celcius = get_celcius(fahrenheit);
            println!("");
            println!("{fahrenheit}°F = {celcius}°C");
            break;
        }
        if selected_option == 2 {
            println!("");
            println!("Celcius -> Fahrenheit");
            print!("Enter temperature °C: ");
            flush();
            let celcius = read_temperature();
            let fahrenheit = get_fahrenheit(celcius);
            println!("");
            println!("{celcius}°C = {fahrenheit}°F");
            break;
        }
    }
}

fn handle_invalide_option() {
    println!("Invalid option");
    print!("Try again: ");
    flush();
}

fn get_celcius(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.) * (5. / 9.);
}

fn get_fahrenheit(celcius: f32) -> f32 {
    return (celcius * (9. / 5.)) + 32.;
}

fn read_temperature() -> f32 {
    loop {
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please enter a valid number: ");
                flush();
                continue;
            }
        };
        break temperature;
    }
}
