//! Module contains Structs that describe all possible
//! responses of the [Prices](crate::api::price::PriceApi)
//! and [Station](crate::api::station::StationApi) api.

pub mod price;
pub mod station;

pub use price::PriceResponse;
pub use station::{AreaFuelResponse, AreaNearResponse, DetailsResponse};

/// Enum of all supported fuel types
#[derive(EnumString, Display, PartialEq, Debug, Eq, Clone, PartialOrd, Ord)]
pub enum Fuel {
    /// Super95 or E5 and serialized to `e5`
    #[strum(serialize = "e5")]
    E5,
    /// Super90 or E10 and serialized to `e10`
    #[strum(serialize = "e10")]
    E10,
    /// Diesel and serialized to `diesel`
    #[strum(serialize = "diesel")]
    Diesel,
}

/// Enum of supported sorting logic
#[derive(EnumString, Display, PartialEq, Debug, Eq, Clone, PartialOrd, Ord)]
pub enum Sort {
    /// Sort by price from lowest to highest price and
    /// will serialize to `price`
    #[strum(serialize = "price")]
    Price,
    /// Sort by distance from near to far and
    /// will serialize to `dist`
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
