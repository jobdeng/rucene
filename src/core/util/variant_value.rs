// Copyright 2019 Zhizhesihai (Beijing) Technology Limited.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use serde;
use serde::ser::{SerializeMap, SerializeSeq};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use core::util::numeric::Numeric;

#[derive(Debug, Clone, Deserialize)]
pub enum VariantValue {
    Bool(bool),
    Char(char),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    VString(String),
    Binary(Vec<u8>),
    Vec(Vec<VariantValue>),
    Map(HashMap<String, VariantValue>),
}

impl VariantValue {
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            VariantValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn get_char(&self) -> Option<char> {
        match self {
            VariantValue::Char(c) => Some(*c),
            _ => None,
        }
    }
    pub fn get_short(&self) -> Option<i16> {
        match self {
            VariantValue::Short(s) => Some(*s),
            _ => None,
        }
    }
    pub fn get_int(&self) -> Option<i32> {
        match self {
            VariantValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn get_long(&self) -> Option<i64> {
        match self {
            VariantValue::Long(l) => Some(*l),
            _ => None,
        }
    }
    pub fn get_numeric(&self) -> Option<Numeric> {
        match *self {
            VariantValue::Short(s) => Some(Numeric::Short(s)),
            VariantValue::Int(i) => Some(Numeric::Int(i)),
            VariantValue::Long(l) => Some(Numeric::Long(l)),
            VariantValue::Float(f) => Some(Numeric::Float(f)),
            VariantValue::Double(d) => Some(Numeric::Double(d)),
            _ => None,
        }
    }
    pub fn get_float(&self) -> Option<f32> {
        match self {
            VariantValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn get_double(&self) -> Option<f64> {
        match self {
            VariantValue::Double(d) => Some(*d),
            _ => None,
        }
    }
    pub fn get_string(&self) -> Option<&str> {
        match self {
            VariantValue::VString(s) => Some(s.as_str()),
            _ => None,
        }
    }
    pub fn get_binary(&self) -> Option<&[u8]> {
        match self {
            VariantValue::Binary(b) => Some(b.as_slice()),
            _ => None,
        }
    }

    pub fn get_utf8_string(&self) -> Option<String> {
        match self {
            VariantValue::VString(s) => Some(s.clone()),
            VariantValue::Binary(b) => {
                if let Ok(s) = String::from_utf8(b.clone()) {
                    Some(s)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // used for index sort check
    pub fn is_zero(&self) -> bool {
        match self {
            VariantValue::Int(i) => *i == 0,
            VariantValue::Long(i) => *i == 0,
            VariantValue::Float(i) => *i == 0.0,
            VariantValue::Double(i) => *i == 0.0,
            _ => {
                unreachable!();
            }
        }
    }

    pub fn get_vec(&self) -> Option<&Vec<VariantValue>> {
        match self {
            VariantValue::Vec(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_map(&self) -> Option<&HashMap<String, VariantValue>> {
        match self {
            VariantValue::Map(m) => Some(m),
            _ => None,
        }
    }
}

impl Eq for VariantValue {}

impl fmt::Display for VariantValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VariantValue::Bool(b) => write!(f, "{}", b),
            VariantValue::Char(c) => write!(f, "{}", c),
            VariantValue::Short(s) => write!(f, "{}s", s),
            VariantValue::Int(ival) => write!(f, "{}", ival),
            VariantValue::Long(lval) => write!(f, "{}", lval),
            VariantValue::Float(fval) => write!(f, "{:.3}", fval),
            VariantValue::Double(d) => write!(f, "{:.6}", d),
            VariantValue::VString(ref s) => write!(f, "{}", s),
            VariantValue::Binary(ref _b) => write!(f, "Binary(unprintable)"),
            VariantValue::Vec(ref v) => write!(f, "{:?}", v),
            VariantValue::Map(ref m) => write!(f, "{:?}", m),
        }
    }
}

impl serde::Serialize for VariantValue {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            VariantValue::Bool(b) => serializer.serialize_bool(b),
            VariantValue::Char(c) => serializer.serialize_char(c),
            VariantValue::Short(s) => serializer.serialize_i16(s),
            VariantValue::Int(ival) => serializer.serialize_i32(ival),
            VariantValue::Long(lval) => serializer.serialize_i64(lval),
            VariantValue::Float(fval) => serializer.serialize_f32(fval),
            VariantValue::Double(d) => serializer.serialize_f64(d),
            VariantValue::VString(ref s) => serializer.serialize_str(s.as_str()),
            VariantValue::Binary(ref b) => serializer.serialize_bytes(b),
            VariantValue::Vec(ref vec) => {
                let mut seq = serializer.serialize_seq(Some(vec.len())).unwrap();
                for v in vec {
                    seq.serialize_element(v)?;
                }

                seq.end()
            }
            VariantValue::Map(ref m) => {
                let mut map = serializer.serialize_map(Some(m.len())).unwrap();
                for (k, v) in m {
                    map.serialize_entry(&k.to_string(), &v)?;
                }
                map.end()
            }
        }
    }
}

impl Hash for VariantValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            VariantValue::Bool(ref b) => b.hash(state),
            VariantValue::Char(ref c) => c.hash(state),
            VariantValue::Short(ref s) => s.hash(state),
            VariantValue::Int(ref i) => i.hash(state),
            VariantValue::Long(ref l) => l.hash(state),
            VariantValue::Float(ref f) => f.to_bits().hash(state),
            VariantValue::Double(ref d) => d.to_bits().hash(state),
            VariantValue::VString(ref s) => s.hash(state),
            VariantValue::Binary(ref v) => v.hash(state),
            _ => (),
        }
    }
}

impl PartialEq for VariantValue {
    fn eq(&self, other: &VariantValue) -> bool {
        match *self {
            VariantValue::Bool(ref b) => {
                if let VariantValue::Bool(ref o) = *other {
                    b.eq(o)
                } else {
                    false
                }
            }
            VariantValue::Char(ref c) => {
                if let VariantValue::Char(ref o) = *other {
                    c.eq(o)
                } else {
                    false
                }
            }
            VariantValue::Short(ref s) => {
                if let VariantValue::Short(ref o) = *other {
                    s.eq(o)
                } else {
                    false
                }
            }
            VariantValue::Int(ref i) => {
                if let VariantValue::Int(ref o) = *other {
                    i.eq(o)
                } else {
                    false
                }
            }
            VariantValue::Long(ref l) => {
                if let VariantValue::Long(ref o) = *other {
                    l.eq(o)
                } else {
                    false
                }
            }
            VariantValue::Float(ref f) => {
                if let VariantValue::Float(ref o) = *other {
                    f.eq(o)
                } else {
                    false
                }
            }
            VariantValue::Double(ref d) => {
                if let VariantValue::Double(ref o) = *other {
                    d.eq(o)
                } else {
                    false
                }
            }
            VariantValue::VString(ref s) => {
                if let VariantValue::VString(ref o) = *other {
                    s.eq(o)
                } else {
                    false
                }
            }
            VariantValue::Binary(ref v) => {
                if let VariantValue::Binary(ref o) = *other {
                    v.eq(o)
                } else {
                    false
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Ord for VariantValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&VariantValue::Bool(b1), &VariantValue::Bool(b2)) => b1.cmp(&b2),
            (&VariantValue::Char(c1), &VariantValue::Char(c2)) => c1.cmp(&c2),
            (&VariantValue::Short(v1), &VariantValue::Short(v2)) => v1.cmp(&v2),
            (&VariantValue::Int(v1), &VariantValue::Int(v2)) => v1.cmp(&v2),
            (&VariantValue::Long(v1), &VariantValue::Long(v2)) => v1.cmp(&v2),
            (&VariantValue::Float(v1), &VariantValue::Float(v2)) => v1.partial_cmp(&v2).unwrap(),
            (&VariantValue::Double(v1), &VariantValue::Double(v2)) => v1.partial_cmp(&v2).unwrap(),
            (&VariantValue::VString(ref s1), &VariantValue::VString(ref s2)) => s1.cmp(&s2),
            (&VariantValue::Binary(ref b1), &VariantValue::Binary(ref b2)) => b1.cmp(&b2),
            (_, _) => panic!("Non-comparable"),
        }
    }
}

impl PartialOrd for VariantValue {
    fn partial_cmp(&self, other: &VariantValue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<bool> for VariantValue {
    fn from(val: bool) -> Self {
        VariantValue::Bool(val)
    }
}

/// Implement the From<char> trait for VariantValue
impl From<char> for VariantValue {
    fn from(val: char) -> Self {
        VariantValue::Char(val)
    }
}

/// Implement the From<i16> trait for VariantValue
impl From<i16> for VariantValue {
    fn from(val: i16) -> Self {
        VariantValue::Short(val)
    }
}

impl From<i32> for VariantValue {
    fn from(val: i32) -> Self {
        VariantValue::Int(val)
    }
}

impl From<i64> for VariantValue {
    fn from(val: i64) -> Self {
        VariantValue::Long(val)
    }
}

impl From<f32> for VariantValue {
    fn from(val: f32) -> Self {
        VariantValue::Float(val)
    }
}

impl From<f64> for VariantValue {
    fn from(val: f64) -> Self {
        VariantValue::Double(val)
    }
}

impl<'a> From<&'a str> for VariantValue {
    fn from(val: &'a str) -> Self {
        VariantValue::VString(String::from(val))
    }
}

impl<'a> From<&'a [u8]> for VariantValue {
    fn from(val: &'a [u8]) -> Self {
        VariantValue::Binary(val.to_vec())
    }
}

impl From<Numeric> for VariantValue {
    fn from(val: Numeric) -> Self {
        debug_assert!(!val.is_null());
        match val {
            Numeric::Byte(b) => VariantValue::Char(b as u8 as char),
            Numeric::Short(s) => VariantValue::Short(s),
            Numeric::Int(i) => VariantValue::Int(i),
            Numeric::Long(v) => VariantValue::Long(v),
            Numeric::Float(v) => VariantValue::Float(v),
            Numeric::Double(v) => VariantValue::Double(v),
            Numeric::Null => unreachable!(),
        }
    }
}

use serde_json::{Value,Number};
use std::convert::{TryFrom,TryInto};
impl TryFrom<&Value> for VariantValue {
    type Error = &'static str;
    /// TODO error with json path
    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        if val.is_boolean() {
            match val.as_bool() {
                None => Err("not a bool"),
                Some(val) => Ok(VariantValue::Bool(val))
            }
        } else if val.is_f64() {
            match val.as_f64() {
                None => Err("not a double"),
                Some(val) => Ok(VariantValue::Double(val))
            }
        } else if val.is_i64() {
            match val.as_i64() {
                None => Err("not a signed long"),
                Some(val) => Ok(VariantValue::Long(val))
            }
        } else if val.is_u64() {
            match val.as_u64() {
                None => Err("not an unsigned long"),
                Some(val) => Ok(VariantValue::Long(val as i64))
            }
        // } else if val.is_number() {}//char, short, int, float
        } else if val.is_string() {
            match val.as_str() {
                None => Err("not a string"),
                Some(val) => Ok(VariantValue::VString(val.into()))//TODO binary?
            }
        } else if val.is_array() {
            match val.as_array() {
                None => Err("not an array"),
                Some(val) => {
                    let mut itms = Vec::<VariantValue>::new();
                    for itm in val {
                        itms.push(VariantValue::try_from(itm)?);
                    }
                    Ok(VariantValue::Vec(itms))
                }
            }
        } else if val.is_object() {
            match val.as_object() {
                None => Err("not an object"),
                Some(val) => {
                    let mut itms = HashMap::<String,VariantValue>::new();
                    for (key,val) in val {
                        itms.insert(key.into(), VariantValue::try_from(val)?);
                    }
                    Ok(VariantValue::Map(itms))
                }
            }
        } else {//null     
            Err("invald value")      
        }
    }
}

impl TryInto<Value> for VariantValue {
    type Error = &'static str;
    fn try_into(self) -> Result<Value, Self::Error> {
        Ok(match self {
            VariantValue::Bool(val) => Value::Bool(val),
            VariantValue::Char(val) => Value::Number(Number::from(val as u8)),
            VariantValue::Short(val) => Value::Number(Number::from(val)),
            VariantValue::Int(val) => Value::Number(Number::from(val)),
            VariantValue::Long(val) => Value::Number(Number::from(val)),
            VariantValue::Float(val) => match Number::from_f64(val as f64) {
                None => return Err("not a json number"),
                Some(val) => Value::Number(val)
            },
            VariantValue::Double(val) => match Number::from_f64(val) {
                None => return Err("not a json number"),
                Some(val) => Value::Number(val)
            },
            VariantValue::VString(val) => Value::String(val),
            VariantValue::Binary(val) => match String::from_utf8(val) {
                Err(err) => return Err("the binary array is not utf-8 string"),
                Ok(val) =>  Value::String(val)
            },
            VariantValue::Vec(vals) => {
                let mut itms = Vec::<Value>::new();
                for val in vals {
                    itms.push(val.try_into()?);
                }
                Value::Array(itms)
            },
            VariantValue::Map(vals) => {
                let mut itms = serde_json::Map::<String,Value>::new();
                for (key,val) in vals {
                    itms.insert(key, val.try_into()?);
                }
                Value::Object(itms)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_bool_test() {
        let b = VariantValue::Bool(true);
        let expr = format!("{}", b);
        assert_eq!(expr, "true");
    }

    #[test]
    fn variant_char_test() {
        let c = VariantValue::Char('Z');
        let expr = format!("{}", c);
        assert_eq!(expr, "Z");
    }

    #[test]
    fn variant_short_test() {
        let s = VariantValue::Short(30);
        let expr = format!("{}", s);
        assert_eq!(expr, "30s");
    }

    #[test]
    fn variant_int_test() {
        let ival = VariantValue::Int(287);
        let expr = format!("{}", ival);
        assert_eq!(expr, "287");
    }

    #[test]
    fn variant_long_test() {
        let ival = VariantValue::Long(28_754_383);
        let expr = format!("{}", ival);
        assert_eq!(expr, "28754383");
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn variant_float_test() {
        let fval = VariantValue::Float(3.141_593);
        let expr = format!("{}f", fval);
        assert_eq!(expr, "3.142f");

        {
            let fval2 = VariantValue::Float(3.141_593);
            assert_eq!(fval.cmp(&fval2), Ordering::Equal);
        }

        {
            let fval2 = VariantValue::Float(2.141_593);
            assert_eq!(fval.cmp(&fval2), Ordering::Greater);
        }

        {
            let fval2 = VariantValue::Float(4.141_593);
            assert_eq!(fval.cmp(&fval2), Ordering::Less);
        }

        {
            let fval2 = VariantValue::Float(4.141_593);
            assert_eq!(fval.partial_cmp(&fval2), Some(Ordering::Less));
        }
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn variant_double_test() {
        let dval = VariantValue::Double(3.141_592_653_5);
        let expr = format!("{}", dval);
        assert_eq!(expr, "3.141593");
    }

    #[test]
    fn variant_string_test() {
        let strval = VariantValue::VString(String::from("hello world"));
        let expr = format!("{}", strval);
        assert_eq!(expr, "hello world");
    }

    #[test]
    fn variant_binary_test() {
        let bval = VariantValue::Binary(vec![65u8, 66u8, 67u8]);
        let expr = format!("{}", bval);
        assert_eq!(expr, "Binary(unprintable)");

        if let VariantValue::Binary(ref bvec) = bval {
            for (i, val) in bvec.iter().enumerate() {
                assert_eq!(*val, b'A' + i as u8);
            }
        }
    }

    #[test]
    fn test_from_json_value() {
        let jval = /*json!()*/ serde_json::from_str::<Value>(r#"{
            "fld_bool": true,
            "fld_double": 3.907,
            "fld_long": -78203,
            "fld_ulong": 45678062,
            "fld_string": "Hi Runcene!",
            "fld_array": [{
                "ary_fld_string": "val1",
                "ary_fld_double": 0.123
            },{
                "ary_fld_string": "val2",
                "ary_fld_double": 9.8765
            }],
            "fld_object": {
                "obj_fld_ulong": 23456789,
                "obj_fld_bool": false,
                "obj_fld_array": ["val1","val2"]
            }
        }"#).unwrap();
        let vval = VariantValue::try_from(&jval).unwrap();
        //TODO serialize vval then commpare ...
        if let VariantValue::Map(map) = vval && !map.is_empty() {
            assert!(map.get("fld_bool").is_some_and(|val|val.get_bool().unwrap()==true));
            assert!(map.get("fld_double").is_some_and(|val|val.get_double().unwrap()==3.907f64));
            assert!(map.get("fld_long").is_some_and(|val|val.get_long().unwrap()==-78203i64));
            assert!(map.get("fld_ulong").is_some_and(|val|val.get_long().unwrap()==45678062i64));
            assert!(map.get("fld_string").is_some_and(|val|val.get_string().unwrap().eq("Hi Runcene!")));
            assert!(map.get("fld_noexists").is_none());
            let fld_ary = map.get("fld_array").unwrap();
            let fld_obj = map.get("fld_object").unwrap();
            if let (VariantValue::Vec(ary),VariantValue::Map(obj)) = (fld_ary,fld_obj) && ary.len()==2 && obj.len()==3 {
                assert!(ary.get(0).unwrap().get_map().unwrap().get("ary_fld_double").is_some_and(|val|val.get_double().unwrap()==0.123f64));
                assert!(obj.get("obj_fld_ulong").is_some_and(|val|val.get_long().unwrap()==23456789));
                assert!(obj.get("obj_fld_bool").is_some_and(|val|val.get_bool().unwrap()==false));
                assert!(obj.get("obj_fld_array").is_some_and(|val|val.get_vec().unwrap().len()==2));
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_into_json_value() {
        let var_val = VariantValue::Bool(true);
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_boolean()&&jsn_val.as_bool().unwrap());
        let var_val = VariantValue::Char('^');
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_number()&&jsn_val.is_u64()&&jsn_val.as_i64().unwrap()==('^' as i64));
        let var_val = VariantValue::Short(82i16);
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_number()&&jsn_val.is_i64()&&jsn_val.as_i64().unwrap()==82i64);
        let var_val = VariantValue::Int(6321i32);
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_number()&&jsn_val.is_i64()&&jsn_val.as_i64().unwrap()==6321i64);
        let var_val = VariantValue::Long(-56783974i64);
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_number()&&jsn_val.is_i64()&&jsn_val.as_i64().unwrap()==-56783974i64);
        let var_val = VariantValue::Float(-32.7458932f32);
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_number()&&jsn_val.is_f64()&&(jsn_val.as_f64().unwrap()+32.745f64).abs()<=0.001);//==-32.7458932f64
        let var_val = VariantValue::Double(345679084.78213f64);
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_number()&&jsn_val.is_f64()&&(jsn_val.as_f64().unwrap()-345679084.782f64).abs()<=0.001);//==345679084.78213f64
        let var_val = VariantValue::VString(String::from("auxhfcmzk_ dsjf udf3432120+(#QR98"));
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_string()&&jsn_val.as_str().unwrap().eq("auxhfcmzk_ dsjf udf3432120+(#QR98"));
        let var_val = VariantValue::Binary("您好！👋Hello Rucene!頑張って".as_bytes().to_vec());
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_string()&&jsn_val.as_str().unwrap().eq("您好！👋Hello Rucene!頑張って"));
        let var_val = VariantValue::Vec(vec![
            VariantValue::VString("成都大运会欢迎您！".into()),
            VariantValue::VString("女子足球世界杯-中国队1:0海地".into()),
            VariantValue::VString("がんばって".into()),
            VariantValue::VString("Let's go hunting!".into())
        ]);
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_array());
        let jsn_ary = jsn_val.as_array().unwrap();
        assert!(jsn_ary.len()==4 &&
            jsn_ary[0].is_string()&&jsn_ary[0].as_str().unwrap().eq("成都大运会欢迎您！") &&
            jsn_ary[3].is_string()&&jsn_ary[3].as_str().unwrap().eq("Let's go hunting!")
        );
        let var_val = VariantValue::Map(vec![
            (String::from("seq_no"),VariantValue::Long(901_245)),
            (String::from("len_km"),VariantValue::Long(45_678_018))
        ].into_iter().collect());
        let jsn_val: Value = var_val.try_into().unwrap();
        assert!(jsn_val.is_object());
        let jsn_obj = jsn_val.as_object().unwrap();
        assert!(jsn_obj.len()==2 && jsn_obj.contains_key("seq_no") && jsn_obj.contains_key("len_km"));
        let len_km = jsn_obj.get("len_km").unwrap();
        assert!(len_km.is_number()&&len_km.is_i64()&&len_km.as_i64().unwrap()==45_678_018i64);
    }
}
