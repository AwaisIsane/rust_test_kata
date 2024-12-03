use std::fmt;

#[derive(Debug)]
struct NegativeNumberError {
    negative_numbers: Vec<i32>,
}

impl fmt::Display for NegativeNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negative_nums_str: Vec<String> = self
            .negative_numbers
            .iter()
            .map(|n| n.to_string())
            .collect();
        write!(
            f,
            "negative numbers not allowed: {}",
            negative_nums_str.join(", ")
        )
    }
}

impl std::error::Error for NegativeNumberError {}

fn add(numbers: &str) -> Result<i32, NegativeNumberError> {
    if numbers.is_empty() {
        return Ok(0);
    }

    // Handle custom delimiter
    let (delimiter, numbers) = if numbers.starts_with("//") {
        let delimiter_end = numbers.find('\n').unwrap_or(numbers.len());
        let delimiter = &numbers[2..delimiter_end];
        let numbers = &numbers[delimiter_end + 1..];
        (delimiter, numbers)
    } else {
        (",", numbers)
    };

    // Replace newlines with the current delimiter
    let numbers = numbers.replace(delimiter, ",");

    // Split and parse numbers
    let parsed_numbers: Result<Vec<i32>, _> = numbers
        .split(&[',', '\n'])
        .map(|n| n.trim().parse())
        .collect();

    let parsed_numbers = match parsed_numbers {
        Ok(nums) => nums,
        Err(_) => return Ok(0), // Handle parsing errors by returning 0
    };

    // Check for negative numbers
    let negative_numbers: Vec<i32> = parsed_numbers.iter().filter(|&&n| n < 0).cloned().collect();

    if !negative_numbers.is_empty() {
        return Err(NegativeNumberError { negative_numbers });
    }

    // Sum numbers, ignoring numbers > 1000
    let sum = parsed_numbers.iter().filter(|&&n| n <= 1000).sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(add("").unwrap(), 0);
    }

    #[test]
    fn test_single_number() {
        assert_eq!(add("1").unwrap(), 1);
    }

    #[test]
    fn test_two_numbers() {
        assert_eq!(add("1,5").unwrap(), 6);
    }

    #[test]
    fn test_multiple_numbers() {
        assert_eq!(add("1,2,3").unwrap(), 6);
    }

    #[test]
    fn test_new_lines_between_numbers() {
        assert_eq!(add("1\n2,3").unwrap(), 6);
    }

    #[test]
    fn test_custom_delimiter() {
        assert_eq!(add("//;\n1;2").unwrap(), 3);
    }

    #[test]
    fn test_custom_delimiter_and_orignalcondition() {
        assert_eq!(add("//;\n1;2,3,4\n5,1").unwrap(), 16);
    }

    #[test]
    fn test_negative_numbers() {
        let result = add("-1,2,-3");
        assert!(result.is_err());

        match result {
            Err(e) => {
                assert_eq!(e.to_string(), "negative numbers not allowed: -1, -3");
            }
            _ => panic!("Expected an error"),
        }
    }

    #[test]
    fn test_numbers_over_1000_ignored() {
        assert_eq!(add("1001,2").unwrap(), 2);
        assert_eq!(add("2,1001").unwrap(), 2);
    }
}
fn main() {
    let a = add("1,2").unwrap();
    println!("Hello, world!,{}", a);
}
