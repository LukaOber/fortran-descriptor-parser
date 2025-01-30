use quote::quote;

#[derive(Debug, Clone)]
struct Nested {
    repetitions: usize,
    elements: Vec<DataElement>,
}

#[derive(Debug, Clone)]
struct ElementInfo {
    repetitions: usize,
    bytes_to_take: usize,
}

impl ElementInfo {
    fn new(repetitions: usize, bytes_to_take: usize) -> Self {
        Self {
            repetitions,
            bytes_to_take,
        }
    }
}

#[derive(Debug, Clone)]
enum DataElement {
    Integer(ElementInfo),
    Float(ElementInfo),
    Double(ElementInfo),
    String(ElementInfo),
    Nested(Nested),
}

impl DataElement {
    fn bytes_to_take(&self) -> usize {
        match self {
            DataElement::Integer(element_info) => element_info.bytes_to_take,
            DataElement::Float(element_info) => element_info.bytes_to_take,
            DataElement::Double(element_info) => element_info.bytes_to_take,
            DataElement::String(element_info) => element_info.bytes_to_take,
            DataElement::Nested(_nested) => unreachable!(),
        }
    }

    fn repetitions(&self) -> usize {
        match self {
            DataElement::Integer(element_info) => element_info.repetitions,
            DataElement::Float(element_info) => element_info.repetitions,
            DataElement::Double(element_info) => element_info.repetitions,
            DataElement::String(element_info) => element_info.repetitions,
            DataElement::Nested(_nested) => unreachable!(),
        }
    }
}

fn split_elements(s: &str) -> Vec<String> {
    let mut complete_list: Vec<String> = vec![];
    let mut current_element: Vec<char> = vec![];
    let mut bracket_count = 0;
    let chars = s.chars();
    for c in chars {
        match c {
            ' ' => (),
            ',' => {
                if bracket_count == 0 {
                    complete_list.push(current_element.iter().collect());
                    current_element.clear();
                } else {
                    current_element.push(c);
                }
            }
            '(' => {
                bracket_count += 1;
                current_element.push(c);
            }
            ')' => {
                bracket_count -= 1;
                current_element.push(c);
            }
            _ => {
                current_element.push(c);
            }
        }
    }
    if current_element.is_empty() {
        complete_list
    } else {
        complete_list.push(current_element.iter().collect());
        complete_list
    }
}

impl DataElement {
    fn parse_element(s: &str) -> DataElement {
        if s.contains("(") {
            let (repetitions_str, rest) = s.split_at(s.find('(').unwrap());
            let repetitions = if repetitions_str.is_empty() {
                1
            } else {
                repetitions_str.parse::<usize>().unwrap_or_else(|_| {
                    panic!("Can't convert {} to repetitions (usize)", repetitions_str)
                })
            };
            let mut bracket_count = 0;
            let mut inner: Vec<char> = Vec::new();

            for c in rest.chars() {
                match c {
                    '(' => {
                        if bracket_count > 0 {
                            inner.push(c);
                        }
                        bracket_count += 1;
                    }
                    ')' => {
                        bracket_count -= 1;
                        if bracket_count < 1 {
                            let inner: String = inner.iter().collect();
                            let split = split_elements(&inner);
                            let mut nested = Nested {
                                repetitions,
                                elements: Vec::new(),
                            };
                            parse_elements(&split, &mut nested.elements);
                            return DataElement::Nested(nested);
                        } else {
                            inner.push(c);
                        }
                    }
                    _ => {
                        inner.push(c);
                    }
                }
            }
            panic!("Opening bracket was not closed: {}", repetitions_str);
        } else {
            let (i, c) =
                if let Some((i, c)) = s.char_indices().find(|(_i, c)| c.is_ascii_alphabetic()) {
                    (i, c)
                } else {
                    panic!("Element does not have a type identifier");
                };
            let (repetitions_str, bytes_to_take_str) = s.split_at(i);
            let bytes_to_take_str: String = bytes_to_take_str.chars().skip(1).collect();
            let repetitions = if repetitions_str.is_empty() {
                1
            } else {
                repetitions_str.parse::<usize>().unwrap_or_else(|_| {
                    panic!("Can't convert {} to repetitions (usize)", repetitions_str)
                })
            };
            let bytes_to_take = bytes_to_take_str.parse::<usize>().unwrap_or_else(|_| {
                panic!(
                    "Can't convert {} to bytes_to_take (usize)",
                    bytes_to_take_str
                )
            });
            match c.to_ascii_uppercase() {
                'I' => DataElement::Integer(ElementInfo::new(repetitions, bytes_to_take)),
                'F' => DataElement::Float(ElementInfo::new(repetitions, bytes_to_take)),
                'D' => DataElement::Double(ElementInfo::new(repetitions, bytes_to_take)),
                'S' => DataElement::String(ElementInfo::new(repetitions, bytes_to_take)),
                _ => panic!("Unsupported type {}", c),
            }
        }
    }
}

fn parse_elements(split: &Vec<String>, list: &mut Vec<DataElement>) {
    for s in split {
        list.push(DataElement::parse_element(s));
    }
}

fn expand_elements(elements: &[DataElement]) -> Vec<DataElement> {
    let mut expanded_elements: Vec<DataElement> = Vec::new();
    for element in elements.iter() {
        match element {
            DataElement::Nested(nested) => {
                for _ in 0..nested.repetitions {
                    let mut nested_elements = expand_elements(&nested.elements);
                    expanded_elements.append(&mut nested_elements);
                }
            }
            _ => {
                for _ in 0..element.repetitions() {
                    expanded_elements.push(element.clone());
                }
            }
        }
    }
    expanded_elements
}

#[proc_macro]
pub fn descriptor_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input_iter = input.into_iter();
    let input_string = if let Some(tree) = input_iter.next() {
        match tree {
            proc_macro::TokenTree::Literal(literal) => literal.to_string(),
            _ => panic!("Please provide a string, found {:?}", tree),
        }
    } else {
        panic!("Please provide a string literal, found empty input");
    };
    let input_string: String = input_string.chars().filter(|c| *c != '"').collect();
    let split = split_elements(&input_string);
    let mut elements = Vec::new();
    parse_elements(&split, &mut elements);

    let expanded_elements = expand_elements(&elements);
    let bytes_to_take = expanded_elements
        .iter()
        .fold(0, |acc, e| acc + e.bytes_to_take());

    let mut parse_tokens = Vec::new();
    for e in expanded_elements.iter() {
        match e {
            DataElement::Integer(element_info) => {
                let bytes_to_take = element_info.bytes_to_take;
                parse_tokens.push(quote! {
                    <[u8] as ::fortran_descriptor_parser::FromSlice<i32>>::to_type(::fortran_descriptor_parser::get_sub_slice(&mut start_byte, #bytes_to_take, i))?
                 });
            }
            DataElement::Float(element_info) => {
                let bytes_to_take = element_info.bytes_to_take;
                parse_tokens.push(quote! {
                    <[u8] as ::fortran_descriptor_parser::FromSlice<f32>>::to_type(::fortran_descriptor_parser::get_sub_slice(&mut start_byte, #bytes_to_take, i))?
                 });
            }
            DataElement::Double(element_info) => {
                let bytes_to_take = element_info.bytes_to_take;
                parse_tokens.push(quote! {
                    <[u8] as ::fortran_descriptor_parser::FromSlice<f64>>::to_type(::fortran_descriptor_parser::get_sub_slice(&mut start_byte, #bytes_to_take, i))?
                 });
            }
            DataElement::String(element_info) => {
                let bytes_to_take = element_info.bytes_to_take;
                parse_tokens.push(quote! {
                    <[u8] as ::fortran_descriptor_parser::FromSlice<String>>::to_type(::fortran_descriptor_parser::get_sub_slice(&mut start_byte, #bytes_to_take, i))?
                 });
            }
            DataElement::Nested(_nested) => unreachable!(),
        }
    }
    quote! {
        |i: &[u8]| {
            if i.len() < #bytes_to_take {
                return Err(::fortran_descriptor_parser::DescriptorParserError::NotEnoughBytes(i.len(), #bytes_to_take));
            }
            let mut start_byte: usize = 0;
            let parsers = (#(#parse_tokens),*);
            Ok(parsers)
        }
    }
    .into()
}
