fn main() {}

fn fibonacci_1(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    fibonacci_1(n - 1) + fibonacci_1(n - 2)
}

fn fibonacci_2(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fibonacci_2(n - 1) + fibonacci_2(n - 2),
    }
}

fn fibonacci_3(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => (0..n).fold((0, 1), |(a, b), _| (b, a + b)).1,
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci_3;

    #[test]
    fn first() {
        assert_eq!(fibonacci_3(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci_3(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci_3(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(fibonacci_3(5), 8);
    }
}
