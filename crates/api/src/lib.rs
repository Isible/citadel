//! Greetings traveler, this is the nonexistent api wrapper for the citadel toolchain


macro_rules! test {
    ($ty:ty) => {{
        let backend = <$ty>::default();
    }};
}

#[cfg(test)]
mod tests {
    use citadel_backend::experimental::asm::AsmBackend;

    use super::*;

    #[test]
    fn test() {
        test!(AsmBackend);
    }
}