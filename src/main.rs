fn main() {
    add();
    println!("Hello, world!");
}

fn add() -> i32 {
    32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let add = add();
        assert_eq!(32, add);
    }
}
