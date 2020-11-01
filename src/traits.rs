/// Describes anything that has a integer code
pub trait Coded {
    fn code(&self) -> u8;
}
