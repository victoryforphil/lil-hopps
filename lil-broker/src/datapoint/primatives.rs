use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Primatives {
    Number(f64),
    String(String),
    Boolean(bool),
    StringArray(Vec<String>),
    NumberArray(Vec<f64>),
    BooleanArray(Vec<bool>),
    ArrayRef(Vec<String>),
}
impl Primatives {
    pub fn from_value(val: Value) -> Option<Primatives> {
        match val {
            Value::String(s) => Some(Primatives::String(s)),
            Value::Number(n) => n.as_f64().map(|f| Primatives::Number(f as f64)),
            Value::Bool(b) => Some(Primatives::Boolean(b)),
            Value::Array(arr) => {
                // Here we check the first element to guess the array type
                if let Some(first) = arr.first() {
                    // Check if first can be a float

                    if first.is_string() {
                        let maybe_string_array: Option<Vec<String>> = arr
                            .into_iter()
                            .map(|v| v.as_str().map(String::from))
                            .collect();
                        maybe_string_array.map(Primatives::StringArray)
                    } else if first.is_number() {
                        let maybe_number_array: Option<Vec<f64>> = arr
                            .into_iter()
                            .map(|v| v.as_f64().map(|n| n as f64))
                            .collect();
                        maybe_number_array.map(Primatives::NumberArray)
                    } else if first.is_boolean() {
                        let maybe_boolean_array: Option<Vec<bool>> =
                            arr.into_iter().map(|v| v.as_bool()).collect();
                        maybe_boolean_array.map(Primatives::BooleanArray)
                    } else {
                        None
                    }
                } else {
                    None // Empty array case or mixed types case
                }
            }
            Value::Null => None,
            _ => None, // Covers other cases such as Object, which you might want to handle differently
        }
    }

    pub fn to_value(&self) -> Value {
        match self {
            Primatives::Number(n) => json!(*n),
            Primatives::String(s) => Value::String(s.clone()),
            Primatives::Boolean(b) => Value::Bool(*b),
            Primatives::StringArray(arr) => {
                Value::Array(arr.iter().map(|s| Value::String(s.clone())).collect())
            }
            Primatives::NumberArray(arr) => Value::Array(arr.iter().map(|n| json!(n)).collect()),
            Primatives::BooleanArray(arr) => {
                Value::Array(arr.iter().map(|b| Value::Bool(*b)).collect())
            }
            Primatives::ArrayRef(arr) => {
                Value::Array(arr.iter().map(|s| Value::String(s.clone())).collect())
            }
        }
    }
}
///Concversion Function from Primative to f64
impl From<Primatives> for f64 {
    fn from(p: Primatives) -> f64 {
        match p {
            Primatives::Number(n) => n,
            _ => panic!("Not a Number"),
        }
    }
}
/// Conversion Function from Primative to String
impl From<Primatives> for String {
    fn from(p: Primatives) -> String {
        match p {
            Primatives::String(s) => s,
            _ => panic!("Not a String"),
        }
    }
}

/// Conversion Function from Primative to bool
impl From<Primatives> for bool {
    fn from(p: Primatives) -> bool {
        match p {
            Primatives::Boolean(b) => b,
            _ => panic!("Not a Boolean"),
        }
    }
}

/// Conversion Function from Primative to Vec<String>
impl From<Primatives> for Vec<String> {
    fn from(p: Primatives) -> Vec<String> {
        match p {
            Primatives::StringArray(s) => s,
            _ => panic!("Not a StringArray"),
        }
    }
}

/// Conversion Function from Primative to Vec<f64>
impl From<Primatives> for Vec<f64> {
    fn from(p: Primatives) -> Vec<f64> {
        match p {
            Primatives::NumberArray(n) => n,
            _ => panic!("Not a NumberArray"),
        }
    }
}

/// Conversion Function from Primative to Vec<bool>
impl From<Primatives> for Vec<bool> {
    fn from(p: Primatives) -> Vec<bool> {
        match p {
            Primatives::BooleanArray(b) => b,
            _ => panic!("Not a BooleanArray"),
        }
    }
}

/// Conversion Function from f64 to Primative
impl From<f64> for Primatives {
    fn from(n: f64) -> Primatives {
        Primatives::Number(n)
    }
}

/// Conversion Function from String to Primative
impl From<String> for Primatives {
    fn from(s: String) -> Primatives {
        Primatives::String(s)
    }
}

/// Conversion Function from bool to Primative
impl From<bool> for Primatives {
    fn from(b: bool) -> Primatives {
        Primatives::Boolean(b)
    }
}

/// Conversion Function from Vec<String> to Primative
impl From<Vec<String>> for Primatives {
    fn from(s: Vec<String>) -> Primatives {
        Primatives::StringArray(s)
    }
}

/// Conversion Function from Vec<f64> to Primative
impl From<Vec<f64>> for Primatives {
    fn from(n: Vec<f64>) -> Primatives {
        Primatives::NumberArray(n)
    }
}

/// Conversion Function from Vec<bool> to Primative
impl From<Vec<bool>> for Primatives {
    fn from(b: Vec<bool>) -> Primatives {
        Primatives::BooleanArray(b)
    }
}
