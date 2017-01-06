struct Generator {
    secret: String,
}

impl Generator {
    pub fn new(secret: String) -> Self {
        Generator { secret: secret }
    }

    fn CreateToken(self) -> Result<String, &'static str> {
        Ok("hello".to_owned())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let gen = super::Generator::new("hello".to_owned());
        match gen.CreateToken() {
            Ok(s) => println!("{}", s),
            _ => println!("wrong"),
        }
    }

}
