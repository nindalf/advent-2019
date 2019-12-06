pub fn number_of_passwords(start: i32, end: i32, password_validator: &dyn Fn(i32) -> bool) -> i32 {
    let mut result = 0;
    for i in start..=end {
        if password_validator(i) {
            result += 1;
        }
    }
    result
}

pub fn is_valid_password(password: i32) -> bool {
    let digits = split_digits(password);
    doublet_exists(&digits) && is_increasing(&digits)
}

pub fn is_valid_password_2(password: i32) -> bool {
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
        assert_eq!(super::is_valid_password(111111), true);
        assert_eq!(super::is_valid_password(122345), true);
        assert_eq!(super::is_valid_password(223450), false);
        assert_eq!(super::is_valid_password(123789), false);
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

    #[test]
    fn test_num_passwords() {
        assert_eq!(
            super::number_of_passwords(278384, 824795, &super::is_valid_password),
            921
        );
        assert_eq!(
            super::number_of_passwords(278384, 824795, &super::is_valid_password_2),
            603
        );
    }
}
