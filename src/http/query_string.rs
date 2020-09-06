use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

// a=123&b=abc&c&d=name&a=456
impl <'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&'){
            let mut key =  sub_str;
            let mut value = "";

            if let Some(i) = sub_str.find('='){
                key = &sub_str[..i]; 
                value = &sub_str[i+ 1..]
            };

            data.entry(key)
            .and_modify(|v| match v {
                Value::Single(single_value) => *v = Value::Multiple(vec![single_value, value]),
                Value::Multiple(vec) => vec.push(value)
            })
            .or_insert(Value::Single(value));
        }

        QueryString{data}
    }
}

#[derive(Debug)]
pub enum Value <'a>{
    Single(&'a str),
    Multiple(Vec<&'a str>)
}