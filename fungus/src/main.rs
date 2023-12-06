use api::add;

fn main() {
    'start: {
        println!("{}", add(1, 2));
        break 'start;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
