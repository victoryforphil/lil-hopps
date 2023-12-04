#[derive(Debug, Clone, PartialEq)]

pub enum TelemtryType{
    String(String),
    Float(f64),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Telemtry {
    pub name: String,
    pub value: TelemtryType,
}