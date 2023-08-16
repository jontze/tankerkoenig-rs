/// Macro to transform a vector into a vector of arrays with a fixed size of 10.
/// The last array will be filled with None if the vector is not divisible by 10.
///
/// # Example
/// ```
/// use tankerkoenig::chunk_into_option_arrays;
///
/// let input_vec = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
/// let output_vec = chunk_into_option_arrays!(input_vec);
///
/// assert_eq!(output_vec.len(), 1);
/// assert_eq!(output_vec[0], [
///     Some("1".to_string()),
///     Some("2".to_string()),
///     Some("3".to_string()),
///     Some("4".to_string()),
///     Some("5".to_string()),
///     Some("6".to_string()),
///     Some("7".to_string()),
///     Some("8".to_string()),
///     Some("9".to_string()),
///     None]);
/// ```
///
#[macro_export]
macro_rules! chunk_into_option_arrays {
    ($input_vec:expr) => {{
        $input_vec
            .chunks(10)
            .map(|chunk| {
                let mut current: [Option<String>; 10] = Default::default();
                for (index, value) in chunk.iter().enumerate() {
                    current[index] = Some((value.as_ref() as &str).to_string());
                }
                current
            })
            .collect::<Vec<[Option<String>; 10]>>()
    }};
}

pub(crate) mod price {
    use std::{cell::RefCell, rc::Rc};

    pub fn format_ids_string<S>(ids: &[Option<S>]) -> String
    where
        S: AsRef<str> + std::fmt::Display,
    {
        ids.iter()
            .fold(
                Rc::new(RefCell::new(String::from(""))),
                |ids_string, potential_id| match potential_id {
                    Some(id) => {
                        ids_string.replace_with(|old_ids_string| {
                            if old_ids_string.is_empty() {
                                id.to_string()
                            } else {
                                format!("{old_ids_string},{id}")
                            }
                        });
                        ids_string
                    }
                    None => ids_string,
                },
            )
            .take()
    }
}

#[cfg(test)]
mod macro_test {

    #[test]
    fn transform_vector_to_vector_of_single_array() {
        let input = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        let output = chunk_into_option_arrays![input];
        assert_eq!(
            output,
            vec![[
                Some("a".to_string()),
                Some("b".to_string()),
                Some("c".to_string()),
                Some("d".to_string()),
                Some("e".to_string()),
                Some("f".to_string()),
                Some("g".to_string()),
                Some("h".to_string()),
                Some("i".to_string()),
                Some("j".to_string())
            ]]
        );
    }

    #[test]
    fn transform_vector_to_vector_of_multiple_arrays_filled_with_none() {
        let input = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "f"];
        let output = chunk_into_option_arrays![input];
        assert_eq!(
            output,
            vec![
                [
                    Some("a".to_string()),
                    Some("b".to_string()),
                    Some("c".to_string()),
                    Some("d".to_string()),
                    Some("e".to_string()),
                    Some("f".to_string()),
                    Some("g".to_string()),
                    Some("h".to_string()),
                    Some("i".to_string()),
                    Some("j".to_string())
                ],
                [
                    Some("f".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                ]
            ]
        );
    }

    #[test]
    fn transform_empty_vector_to_empty_vector() {
        let input: Vec<&str> = vec![];
        let output = chunk_into_option_arrays![input];
        let expected: Vec<[Option<String>; 10]> = vec![];
        assert_eq!(output, expected);
    }
}

#[cfg(test)]
mod price_test {
    use super::price::*;

    #[test]
    fn should_create_string_of_ids() {
        // Construct string of ids out of array with potential ids
        let ids = vec![Some("123"), Some("456"), None, None, Some("789")];
        let ids_string = format_ids_string(&ids);
        assert_eq!(ids_string, "123,456,789");

        // Empty if array only None
        let ids: Vec<Option<String>> = vec![None, None];
        let ids_string = format_ids_string(&ids);
        assert_eq!(ids_string, "");

        // Should have first available id as first part of the id string
        let ids = vec![None, Some("123"), None, Some("456")];
        let ids_string = format_ids_string(&ids);
        assert_eq!(ids_string, "123,456");
    }
}
