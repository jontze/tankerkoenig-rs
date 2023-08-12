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
