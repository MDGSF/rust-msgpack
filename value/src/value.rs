use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.format_value(self, 0))
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Null => Value::Null,
            Value::Bool(b) => Value::Bool(*b),
            Value::Number(n) => Value::Number(n.clone()),
            Value::String(s) => Value::String(s.clone()),
            Value::Array(arr) => {
                let mut result: Vec<Value> = Vec::new();
                for x in arr {
                    result.push(x.clone());
                }
                Value::Array(result)
            }
            Value::Object(hm) => {
                let mut result: HashMap<String, Value> = HashMap::new();
                for key in hm.keys() {
                    let value = hm.get(key).unwrap();
                    result.insert(key.clone(), value.clone());
                }
                Value::Object(result)
            }
        }
    }
}

impl Value {
    pub fn get_type(&self) -> String {
        match self {
            Value::Null => "Null".to_string(),
            Value::Bool(_) => "Bool".to_string(),
            Value::Number(_) => "Number".to_string(),
            Value::String(_) => "String".to_string(),
            Value::Array(_) => "Array".to_string(),
            Value::Object(_) => "Object".to_string(),
        }
    }

    pub fn get_string(&self) -> std::string::String {
        match self {
            Value::String(s) => s.clone(),
            _ => panic!("value is not string"),
        }
    }
}

impl Value {
    fn format_value(&self, input: &Value, level: i32) -> String {
        match input {
            Value::Null => self.format_null(),
            Value::Bool(b) => self.format_bool(*b),
            Value::Number(n) => self.format_number(n.to_string()),
            Value::String(s) => self.format_string(s.to_string()),
            Value::Array(a) => self.format_array(a, level),
            Value::Object(o) => self.format_object(o, level),
        }
    }

    fn format_null(&self) -> String {
        "null".to_string()
    }

    fn format_bool(&self, input: bool) -> String {
        if input {
            return "true".to_string();
        }
        "false".to_string()
    }

    fn format_number(&self, input: String) -> String {
        input
    }

    fn format_string(&self, input: String) -> String {
        "\"".to_string() + &input + "\""
    }

    fn format_array(&self, input: &Vec<Value>, level: i32) -> String {
        let mut result = "[\n".to_string();

        let mut idx = 0;
        let length = input.len();
        for item in input.iter() {
            result += " ".repeat(((level + 1) * 4) as usize).as_str();
            result += self.format_value(item, level + 1).as_str();

            if idx == length - 1 {
                result += "\n";
            } else {
                result += ",\n";
            }

            idx += 1;
        }

        result += " ".repeat((level * 4) as usize).as_str();
        result += "]";
        result
    }

    fn format_object(&self, input: &HashMap<String, Value>, level: i32) -> String {
        let mut result = "{\n".to_string();

        let mut idx = 0;
        let length = input.len();
        for (k, v) in input {
            result += " ".repeat(((level + 1) * 4) as usize).as_str();
            result = result + "\"" + &k + "\": ";
            result += self.format_value(&v, level + 1).as_str();

            if idx == length - 1 {
                result += "\n";
            } else {
                result += ",\n";
            }

            idx += 1;
        }

        result += " ".repeat((level * 4) as usize).as_str();
        result += "}";
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
