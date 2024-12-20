// This function returns how much icecream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no icecream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete the function body.
    match hour_of_day {
        0..=21 => Some(5),
        22..=23 => Some(0),
        _ => None,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
        let icecreams = maybe_ice_cream(12).unwrap_or_default();

        assert_eq!(icecreams, 5); // Don't change this line.
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}