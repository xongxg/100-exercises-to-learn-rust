pub fn sum(n: u32) -> u32 {
    let mut sum = 0;
    let mut i = 1;
    while i <= n {
        sum += i;
        i += 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn first() {
        assert_eq!(sum(1), 1);
    }

    #[test]
    fn second() {
        assert_eq!(sum(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(sum(2), 3);
    }

    #[test]
    fn fifth() {
        assert_eq!(sum(5),15);
    }
}
