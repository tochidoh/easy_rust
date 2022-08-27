const OKAY_CHARACTERS: &str = "1234567890+- ";

#[derive(Clone)]
struct Calculator {
    results: Vec<String>,
    current_input: String,
    total: i32,
    adds: bool,
}

impl Calculator {
    fn new() -> Self {
        Self {
            results: vec![],
            current_input: String::new(),
            total: 0,
            adds: true,
        }
    }

    fn clear(&mut self) {
        self.current_input.clear();
    }

    fn push_char(&mut self, character: char) {
        self.current_input.push(character);
    }
}

fn math(input: &str) -> i32 {
    // check only valid chars
    if !input
        .chars()
        .all(|character| OKAY_CHARACTERS.contains(character)) ||
        !input.chars().take(2).any(|character| character.is_numeric())
    {
        panic!("please only input numbers, +-, or spaces");
    }

    // remove trailing +/-, remove whitespace, collect into string
    let input = input
        .trim_end_matches(|x| "+- ".contains(x))
        .chars()
        .filter(|x| *x != ' ')
        .collect::<String>(); // string bc mutable


    let mut calculator = Calculator::new();

    for character in input.chars() {
        match character {
            '+' => {
                if !calculator.current_input.is_empty() {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear();
                }
            },
            '-' => {
                if calculator.current_input.contains('-') || calculator.current_input.is_empty() {
                    calculator.push_char(character);
                } else {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear();
                    calculator.push_char(character);
                }
            },
            number => {
                if calculator.current_input.contains('-') {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear();
                    calculator.push_char(number);
                } else {
                    calculator.push_char(number);
                }
            },
        }
    }
    calculator.results.push(calculator.current_input);

    let math_iter = calculator.results.into_iter();

    for entry in math_iter {
        if entry.contains('-') {
            if entry.chars().count() % 2 == 1 {
                calculator.adds = match calculator.adds {
                    true => false,
                    false => true,
                };
                continue;
            } else {
                continue;
            }
        }

        if calculator.adds {
            calculator.total += entry.parse::<i32>().unwrap();
        } else {
            calculator.total -= entry.parse::<i32>().unwrap();
        }
    }
    
    calculator.total
}

fn main() {
    math("7 + 198931");
    math("1 + 1----+-+-+----+");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }

    #[test]
    fn one_minus_two_is_minus_one() {
        assert_eq!(math("1 - 2"), -1);
    }

    #[test]
    fn one_minus_minus_one_is_two() {
        assert_eq!(math("1 --1"), 2);
    }

    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        math("7 + seven");
    }

    #[test]
    #[should_panic]
    fn panics_when_bunch_of_symbols() {
        math("--+5");
    }
}
