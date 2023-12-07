#[derive(Debug, PartialEq)]
pub struct Map {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl Map {
    pub fn new(
        destination_range_start: usize,
        source_range_start: usize,
        range_length: usize,
    ) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }

    fn is_in_range(&self, value: usize) -> bool {
        (self.source_range_start..self.source_range_start + self.range_length).contains(&value)
    }

    pub fn convert(&self, value: usize) -> Option<usize> {
        if self.is_in_range(value) {
            let offset = value - self.source_range_start;
            Some(self.destination_range_start + offset)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_in_range() {
        let map = Map {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };
        assert!(map.is_in_range(98));
        assert!(map.is_in_range(99));
        assert!(!map.is_in_range(100));
    }

    #[test]
    fn test_convert() {
        let map = Map {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };
        assert_eq!(map.convert(98), Some(50));
        assert_eq!(map.convert(99), Some(51));
        assert_eq!(map.convert(100), None);
    }
}
