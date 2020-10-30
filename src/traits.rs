/// Describes anything that has a integer code
pub trait Coded {
    fn get_code(&self) -> u8;
}
