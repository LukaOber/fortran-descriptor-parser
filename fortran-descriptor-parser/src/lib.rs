pub use fortran_descriptor_parser_macro::descriptor_parser;
use lexical::{format::STANDARD, parse, parse_with_options};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum DescriptorParserError {
    #[error("Can't convert '{0}' into f32")]
    Invalidf32(String),
    #[error("Can't convert '{0}' into f64")]
    Invalidf64(String),
    #[error("Can't convert '{0}' into i32")]
    Invalidi32(String),
    #[error("Found {0} bytes, expected at least {1}")]
    NotEnoughBytes(usize, usize),
}

pub trait FromSlice<T> {
    fn to_type(&self) -> Result<T, DescriptorParserError>;
}

impl FromSlice<f32> for [u8] {
    fn to_type(&self) -> Result<f32, DescriptorParserError> {
        let f_res = parse(self.trim_ascii()).map_err(|_| {
            DescriptorParserError::Invalidf32(String::from_utf8_lossy(self).to_string())
        });
        match f_res {
            Ok(_) => f_res,
            Err(_) => {
                let options = lexical::ParseFloatOptions::builder()
                    .exponent(b'D')
                    .build()
                    .unwrap();
                parse_with_options::<f32, _, STANDARD>(self.trim_ascii(), &options).map_err(|_| {
                    DescriptorParserError::Invalidf32(String::from_utf8_lossy(self).to_string())
                })
            }
        }
    }
}

impl FromSlice<f64> for [u8] {
    fn to_type(&self) -> Result<f64, DescriptorParserError> {
        let options = lexical::ParseFloatOptions::builder()
            .exponent(b'D')
            .build()
            .unwrap();
        let f_res =
            parse_with_options::<f64, _, STANDARD>(self.trim_ascii(), &options).map_err(|_| {
                DescriptorParserError::Invalidf64(String::from_utf8_lossy(self).to_string())
            });
        match f_res {
            Ok(_) => f_res,
            Err(_) => parse(self.trim_ascii()).map_err(|_| {
                DescriptorParserError::Invalidf64(String::from_utf8_lossy(self).to_string())
            }),
        }
    }
}

impl FromSlice<i32> for [u8] {
    fn to_type(&self) -> Result<i32, DescriptorParserError> {
        parse(self.trim_ascii()).map_err(|_| {
            DescriptorParserError::Invalidi32(String::from_utf8_lossy(self).to_string())
        })
    }
}

impl FromSlice<String> for [u8] {
    fn to_type(&self) -> Result<String, DescriptorParserError> {
        Ok(String::from_utf8_lossy(self.trim_ascii()).to_string())
    }
}

pub fn get_sub_slice<'a>(start_byte: &mut usize, width: usize, slice: &'a [u8]) -> &'a [u8] {
    let end_byte = *start_byte + width;
    let s = &slice[*start_byte..end_byte];
    *start_byte += width;
    s
}
