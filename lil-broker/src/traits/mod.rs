use std::collections::BTreeMap;

use crate::{utils::BrokerKey, Primatives};

pub trait BrokerType{
    fn get_type() -> String;
    fn from_broker(primitives: BTreeMap<String, Primatives>) -> Result<Self, anyhow::Error> where Self: Sized;
    fn to_broker(&self) -> BTreeMap<String, Primatives>;
}

/// Mock implementation of BrokerType
pub struct MockType{
    pub value_number: f64,
    pub value_string: String,
    pub child : MockTypeChild,
}

pub struct MockTypeChild{
    pub value_array : Vec<f64>,
}

impl BrokerType for MockType{
    fn get_type() -> String{
        "MockType".to_string()
    }

    fn from_broker(primitives: BTreeMap<String, Primatives>) -> Result<Self, anyhow::Error>{
        match primitives.get("_type"){
            Some(Primatives::String(s)) => {
                if s != Self::get_type().as_ref(){
                    return Err(anyhow::anyhow!("Incorrect type"));
                }
            },
            _ => {
                return Err(anyhow::anyhow!("Missing _type"));
            }
        };
        //TODO: make this into a utility function that just returns the type in a result. 
        // Such as value_number: f64 = get_value("value_number", primitives)?;
        // This will dynamically check the type of the value and return it in the correct type.
        let value_number = match primitives.get("value_number"){
            Some(Primatives::Number(n)) => *n,
            _ => {
                return Err(anyhow::anyhow!("Missing valid value for value_number"));
            }
        };
        let value_string = match primitives.get("value_string"){
            Some(Primatives::String(s)) => s.clone(),
            _ => {
                return Err(anyhow::anyhow!("Missing valid value for value_string"));
            }
        };
       
        let child = match MockTypeChild::from_broker(primitives){
            Ok(c) => c,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Self{
            value_number,
            value_string,
            child,
        })
    }

    fn to_broker(&self) -> BTreeMap<String, Primatives>{
        let mut map = BTreeMap::new();
        map.insert("value_number".to_string(), Primatives::Number(self.value_number));
        map.insert("value_string".to_string(), Primatives::String(self.value_string.clone()));
        

        let child_map = self.child.to_broker();
        let child_map = BrokerKey::prefix_batch(child_map, "child");
        map.extend(child_map.into_iter());
        map
    }
}

impl BrokerType for MockTypeChild{
    fn get_type() -> String{
        "MockTypeChild".to_string()
    }

    fn from_broker(primitives: BTreeMap<String, Primatives>) -> Result<Self, anyhow::Error>{
        match primitives.get("_type"){
            Some(Primatives::String(s)) => {
                if s != Self::get_type().as_str(){
                    return Err(anyhow::anyhow!("Incorrect type"));
                }
            },
            _ => {
                return Err(anyhow::anyhow!("Missing _type"));
            }
        };

        let value_array = match primitives.get("value_array"){
            Some(Primatives::NumberArray(a)) => a.clone(),
            _ => {
                return Err(anyhow::anyhow!("Missing valid value for value_array"));
            }
        };

        Ok(Self{
            value_array,
        })

    }

    fn to_broker(&self) -> BTreeMap<String, Primatives>{
        let mut map = BTreeMap::new();
        map.insert("value_array".to_string(), Primatives::NumberArray(self.value_array.clone()));
        map
    }

}