#[aoc(day4, part1)]
pub fn passwords_1(input: &str) -> i32 {
    number_of_passwords(input, &is_valid_password_1)
}

#[aoc(day4, part2)]
pub fn passwords_2(input: &str) -> i32 {
    number_of_passwords(input, &is_valid_password_2)
}

pub fn number_of_passwords(input: &str, password_validator: &dyn Fn(i32) -> bool) -> i32 {
    let mut parts = input.split('-');
    let start: i32 = parts.next().unwrap().parse().unwrap();
    let end: i32 = parts.next().unwrap().parse().unwrap();
    let mut result = 0;
    for i in start..=end {
        if password_validator(i) {
            result += 1;
        }
    }
    result
}

fn is_valid_password_1(password: i32) -> bool {
    let digits = split_digits(password);
    doublet_exists(&digits) && is_increasing(&digits)
}

fn is_valid_password_2(password: i32) -> bool {
    let digits = split_digits(password);
    exact_doublet_exists(&digits) && is_increasing(&digits)
}

fn split_digits(number: i32) -> Vec<i32> {
    let mut digits = Vec::with_capacity(6);
    let mut n = number;
    while n > 0 {
        let lsd = n % 10;
        digits.push(lsd);
        n = n / 10;
    }
    digits.reverse();
    digits
}

fn is_increasing(digits: &[i32]) -> bool {
    for i in 1..digits.len() {
        if digits[i] < digits[i - 1] {
            return false;
        }
    }
    true
}

fn doublet_exists(digits: &[i32]) -> bool {
    if digits.len() < 2 {
        return false;
    }
    for i in 1..digits.len() {
        if digits[i] == digits[i - 1] {
            return true;
        }
    }
    false
}

fn exact_doublet_exists(digits: &[i32]) -> bool {
    if digits.len() < 2 {
        return false;
    }
    for i in 1..digits.len() {
        let mut exact_doublet = false;
        if digits[i] == digits[i - 1] {
            exact_doublet = true;
        }
        if i >= 2 && digits[i] == digits[i - 2] {
            exact_doublet = false;
        }
        if i + 1 < digits.len() && digits[i] == digits[i + 1] {
            exact_doublet = false;
        }
        if exact_doublet {
            return exact_doublet;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_password() {
        assert_eq!(super::is_valid_password_1(111111), true);
        assert_eq!(super::is_valid_password_1(122345), true);
        assert_eq!(super::is_valid_password_1(223450), false);
        assert_eq!(super::is_valid_password_1(123789), false);
    }

    #[test]
    fn test_valid_password_2() {
        assert_eq!(super::is_valid_password_2(112233), true);
        assert_eq!(super::is_valid_password_2(778888), true);
        assert_eq!(super::is_valid_password_2(123444), false);
        assert_eq!(super::is_valid_password_2(124445), false);
        assert_eq!(super::is_valid_password_2(111111), false);
        assert_eq!(super::is_valid_password_2(111122), true);
        assert_eq!(super::is_valid_password_2(223450), false);
        assert_eq!(super::is_valid_password_2(123789), false);
        assert_eq!(super::is_valid_password_2(555789), false);
    }
}
