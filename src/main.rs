use std::env;
use std::path::Path;
use std::string::String;


static ROMAN_NUMERALS: &'static [&'static str] = &[
    "I", "IV", "V", "IX", "X", // 1, 4, 5, 9, 10
    "XL", "L", "XC", "C", "CD", "D", "CM", "M", // 40, 50, 90, 100, 400, 500, 900, 1000
];


static NUMBERS: &[u16] = &[1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000];


// Convert integer to Roman numerals
fn int_to_roman(number: u16) -> Result<String, String> {
    // largest number that can be represented is 4999
    if !(0 < number && number <= 4999) {
        Err("Please enter positive number from 1 to 4999".to_owned())
    } else {
        let mut out = String::with_capacity(4);
        let mut val = number;
        for (i, &num) in NUMBERS.iter().rev().enumerate() {
            if val >= num {
                let quotient = val / num;
                val %= num; // set to remainder
                if quotient > 0 {
                    let v = ROMAN_NUMERALS[NUMBERS.len() - 1 - i];
                    // repeat numeral `quotient` times (e.g. 2 = 2 * I = II, 30 = 3 * X = XXX)
                    out.push_str(&v.repeat(quotient as usize));
                }
            }
        }
        Ok(out)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_to_roman_test() {
        assert_eq!(int_to_roman(1).unwrap(), "I");
        assert_eq!(int_to_roman(3).unwrap(), "III");
        assert_eq!(int_to_roman(7).unwrap(), "VII");
        assert_eq!(int_to_roman(26).unwrap(), "XXVI");
        assert_eq!(int_to_roman(344).unwrap(), "CCCXLIV");
        assert_eq!(int_to_roman(4999).unwrap(), "MMMMCMXCIX");
        assert_ne!(int_to_roman(123).unwrap(), "IVX"); // negative
        assert!(int_to_roman(0).is_err(), "Error must be returned"); // err
        assert!(int_to_roman(5555).is_err(), "Error must be returned"); // err
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Usage: {} <number>",
            // get current executable file name
            Path::new(&args[0]).file_name().unwrap().to_str().unwrap()
        );
        return;
    }

    let number = args[1].parse::<u16>();
    match number {
        Ok(i) => {
            match int_to_roman(i) {
                Ok(res) => println!("{}", res),
                Err(err) => println!("Error: {}", err),
            }
        }
        Err(_) => println!("Error: Please enter positive number from 1 to 4999"),
    };
}
