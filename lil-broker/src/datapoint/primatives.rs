#[derive(Debug, PartialEq,Clone)]
pub enum Primatives{
    Number(f64),
    String(String),
    Boolean(bool),
    StringArray(Vec<String>),
    NumberArray(Vec<f64>),
    BooleanArray(Vec<bool>),
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