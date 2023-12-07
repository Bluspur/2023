use rayon::prelude::*;
use std::error::Error;

use crate::converter::Converter;
use crate::parser::parse_almanac;

#[derive(Debug, PartialEq)]
pub struct Almanac {
    pub seeds: Vec<usize>,
    pub seed_to_soil_map: Converter,
    pub soil_to_fertilizer_map: Converter,
    pub fertilizer_to_water_map: Converter,
    pub water_to_light_map: Converter,
    pub light_to_temperature_map: Converter,
    pub temperature_to_humidity_map: Converter,
    pub humidity_to_location_map: Converter,
}

impl Almanac {
    pub fn from_string(input: &str) -> Result<Almanac, Box<dyn Error + '_>> {
        parse_almanac(input)
    }

    pub fn calculate_seed_locations(&self) -> Vec<usize> {
        self.seeds
            .iter()
            .map(|seed| self.convert_seed_to_location(*seed))
            .collect()
    }

    pub fn calculate_seed_locations_with_seed_ranges(&self) -> Vec<usize> {
        assert!(self.seeds.len() % 2 == 0, "Seeds must be in pairs");
        self.seeds
            .par_chunks(2)
            .flat_map(|pair| {
                (pair[0]..pair[0] + pair[1])
                    .into_par_iter()
                    .map(move |seed| self.convert_seed_to_location(seed))
            })
            .collect()
    }

    fn convert_seed_to_location(&self, seed: usize) -> usize {
        let mut res = self.seed_to_soil_map.convert(seed);
        res = self.soil_to_fertilizer_map.convert(res);
        res = self.fertilizer_to_water_map.convert(res);
        res = self.water_to_light_map.convert(res);
        res = self.light_to_temperature_map.convert(res);
        res = self.temperature_to_humidity_map.convert(res);
        self.humidity_to_location_map.convert(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::map::Map;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_ALMANAC: Almanac = Almanac {
            seeds: vec![79, 14, 55, 13],
            seed_to_soil_map: Converter::new(vec![Map::new(50, 98, 2), Map::new(52, 50, 48)]),
            soil_to_fertilizer_map: Converter::new(vec![
                Map::new(0, 15, 37),
                Map::new(37, 52, 2),
                Map::new(39, 0, 15),
            ]),
            fertilizer_to_water_map: Converter::new(vec![
                Map::new(49, 53, 8),
                Map::new(0, 11, 42),
                Map::new(42, 0, 7),
                Map::new(57, 7, 4),
            ]),
            water_to_light_map: Converter::new(vec![Map::new(88, 18, 7), Map::new(18, 25, 70)]),
            light_to_temperature_map: Converter::new(vec![
                Map::new(45, 77, 23),
                Map::new(81, 45, 19),
                Map::new(68, 64, 13),
            ]),
            temperature_to_humidity_map: Converter::new(vec![
                Map::new(0, 69, 1),
                Map::new(1, 0, 69),
            ]),
            humidity_to_location_map: Converter::new(vec![
                Map::new(60, 56, 37),
                Map::new(56, 93, 4),
            ]),
        };
    }

    #[test]
    fn test_calculate_seed_locations() {
        assert_eq!(
            TEST_ALMANAC.calculate_seed_locations(),
            vec![82, 43, 86, 35]
        );
    }
}
