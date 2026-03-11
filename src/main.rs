use std::io;

fn read_positive_number(prompt: &str, allow_zero: bool) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Nie udalo sie odczytac danych");

        // Accept both comma and dot decimals
        let cleaned = input.trim().replace(',', ".");

        match cleaned.parse::<f64>() {
            Ok(num) => {
                if num < 0.0 {
                    println!("Wartość nie może być ujemna. Spróbuj ponownie.\n");
                } else if !allow_zero && num == 0.0 {
                    println!("Wartość musi być większa niż zero. Spróbuj ponownie.\n");
                } else {
                    return num;
                }
            }
            Err(_) => {
                println!("Nieprawidłowa liczba. Spróbuj ponownie.\n");
            }
        }
    }
}

fn main() {
    // Radius must be > 0
    let radius = read_positive_number("Wprowadź promień:", false);

    // Chord must be >= 0 and <= diameter
    let chord = loop {
        let c = read_positive_number("Wprowadź cięciwę:", true);

        if c <= 2.0 * radius {
            break c;
        } else {
            println!("Cięciwa nie może być większa niż średnica (2 * promień). Spróbuj ponownie.\n");
        }
    };

    // Alpha in degrees
    let alpha = 2.0 * (chord / (2.0 * radius)).asin();

    // Arc length
    let length = radius * alpha;

    println!("Szukana długość łuku to: {:.4}", length);
    println!("\nWciśnij Enter aby wyjść...");

    let mut exit = String::new();
    io::stdin().read_line(&mut exit).unwrap();
}