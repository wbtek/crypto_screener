// MIT License
// 
// Copyright (c) 2024 - WBTek: Greg Slocum
// Division of WhiteBear Family, Inc.
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, Visitor};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub enum StringNumberFloat {
    String(String),
    Number(i64),
    Float(f64),
}

impl fmt::Display for StringNumberFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StringNumberFloat::String(s) => write!(f, "{}", s),
            StringNumberFloat::Number(n) => write!(f, "{}", n),
            StringNumberFloat::Float(fl) => write!(f, "{:}", fl), // Use default float formatting
        }
    }
}

pub fn string_number_float<'de, D>(deserializer: D) -> Result<StringNumberFloat, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringNumberFloatVisitor;

    impl<'de> Visitor<'de> for StringNumberFloatVisitor {
        type Value = StringNumberFloat;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string, a number, or a float")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.is_empty() {
                Ok(StringNumberFloat::String("".to_string()))
            } else {
                value.parse::<f64>()
                    .map(StringNumberFloat::Float)
                    .or_else(|_| Ok(StringNumberFloat::String(value.to_owned())))
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(StringNumberFloat::Number(value))
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(StringNumberFloat::Float(value))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(StringNumberFloat::Float(0.0))
        }
    }

    deserializer.deserialize_any(StringNumberFloatVisitor)
}

