// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    } else if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    } else {
        let mut counter = 2;
        let mut tmp = 2;
        while counter != n {
            counter += 1;
            tmp = tmp * counter;
        }
        tmp
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
