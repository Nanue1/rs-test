pub trait Validate {
    fn validate(&self, val: &str) ->Result<(),String>;
}
pub struct Validator {}

impl Validator {
    pub fn new()-> Self {
        Self { }
    }
}
impl Validate for Validator {
    fn validate(&self, val: &str) -> Result<(),String> {
        println!("{}", val);
        Ok(())
    }
}