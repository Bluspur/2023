use crate::map::Map;

#[derive(Debug, PartialEq)]
pub struct Converter(Vec<Map>);

impl Converter {
    pub fn new(maps: Vec<Map>) -> Self {
        Self(maps)
    }

    pub fn convert(&self, value: usize) -> usize {
        for map in &self.0 {
            if let Some(converted_value) = map.convert(value) {
                return converted_value;
            }
        }
        value
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::map::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_CONVERTER: Converter = Converter::new(vec![
            Map::new(50, 98, 2),
            Map::new(0, 15, 38),
            Map::new(49, 53, 8),
        ]);
    }

    #[test]
    fn test_convert_within_range() {
        // First map
        assert_eq!(TEST_CONVERTER.convert(98), 50);
        assert_eq!(TEST_CONVERTER.convert(99), 51);

        // Second map
        assert_eq!(TEST_CONVERTER.convert(15), 0);
        assert_eq!(TEST_CONVERTER.convert(16), 1);
        assert_eq!(TEST_CONVERTER.convert(52), 37);

        // Third map
        assert_eq!(TEST_CONVERTER.convert(53), 49);
        assert_eq!(TEST_CONVERTER.convert(54), 50);
        assert_eq!(TEST_CONVERTER.convert(60), 56);
    }

    #[test]
    fn test_convert_out_of_range() {
        assert_eq!(TEST_CONVERTER.convert(97), 97);
        assert_eq!(TEST_CONVERTER.convert(100), 100);
        assert_eq!(TEST_CONVERTER.convert(14), 14);
        assert_eq!(TEST_CONVERTER.convert(61), 61);
    }
}
