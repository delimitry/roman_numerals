use std::env;
use std::path::Path;
use std::string::String;


static ROMAN_NUMERALS: &'static [&'static str] = &[
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", // 1..10
    "XL", "L", "XC", "C", "CD", "D", "CM", "M", // 40, 50, 90, 100, 400, 500, 900, 1000
];


static NUMBERS: &[u16] = &[
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    40,
    50,
    90,
    100,
    400,
    500,
    900,
    1000,
];


// Convert integer to Roman numerals
fn int_to_roman(number: u16) -> String {
    // largest number that can be represented is 4999
    let mut out = String::with_capacity(4);
    let mut val = number;
    for (i, &n) in NUMBERS.iter().rev().enumerate() {
        if val >= n {
            let q = val / n;
            val = val % n;
            if q > 0 {
                let v = ROMAN_NUMERALS[NUMBERS.len() - 1 - i];
                out.push_str(&v.repeat(q as usize));
            }
        }
    }
    out
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_to_roman_test() {
        assert_eq!(int_to_roman(1), "I");
        assert_eq!(int_to_roman(26), "XXVI");
        assert_eq!(int_to_roman(344), "CCCXLIV");
        assert_eq!(int_to_roman(4999), "MMMMCMXCIX");
        assert_ne!(int_to_roman(123), "IVX"); // negative
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
            if !(0 < i && i <= 4999) {
                println!("Please enter positive number from 1 to 4999");
                return;
            }
            println!("{}", int_to_roman(i))
        }
        Err(..) => println!("Please enter positive number from 1 to 4999"),
    };
}
