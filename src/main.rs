fn greeting() -> String {
    String::from("Hello from tpe-frontend-webapp!")
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greets() {
        assert!(greeting().contains("tpe-frontend-webapp"));
    }
}
