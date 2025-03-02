pub fn factorial(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 3);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 15);
    }
}
