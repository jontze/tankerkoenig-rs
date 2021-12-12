pub mod price;
pub mod station;

#[derive(EnumString, Display, PartialEq, Debug)]
pub enum Fuel {
    #[strum(serialize = "e5")]
    E5,
    #[strum(serialize = "e10")]
    E10,
    #[strum(serialize = "diesel")]
    Diesel,
}

#[derive(EnumString, Display, PartialEq, Debug)]
pub enum Sort {
    #[strum(serialize = "price")]
    Price,
    #[strum(serialize = "dist")]
    Distance,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_fuel_enum_e5() {
        assert_eq!(Fuel::E5.to_string(), "e5");
    }

    #[test]
    fn serialize_fuel_enum_e10() {
        assert_eq!(Fuel::E10.to_string(), "e10");
    }

    #[test]
    fn serialize_fuel_enum_diesel() {
        assert_eq!(Fuel::Diesel.to_string(), "diesel");
    }

    #[test]
    fn serialize_sort_enum_price() {
        assert_eq!(Sort::Price.to_string(), "price");
    }

    #[test]
    fn serialize_sort_enum_distance() {
        assert_eq!(Sort::Distance.to_string(), "dist");
    }
}
