use std::{fs, fs::File, io::{Error, BufRead, BufReader}};

use regex::Regex;

#[derive(Debug)]
struct MyMapEntry {
  source_start: u64,
  destination_start: u64,
  range: u64,
}

impl MyMapEntry {
    fn can_map(&self, source: u64) -> bool {
        source >= self.source_start && source <= self.source_start + self.range
    }

    fn map(&self, source: u64) -> u64 {
        source - self.source_start + self.destination_start
    }
}

#[derive(Debug)]
struct MyMap {
    map_entries: Vec<MyMapEntry>
}

impl MyMap {
    fn map(&self, source: u64) -> u64 {
        match &self.map_entries.iter().find(|entry| entry.can_map(source)) {
            None => { source },
            Some(res) => { res.map(source) },
        }
    }
}

fn get_seeds(file: &File) -> Vec<u64> {
    let mut seeds: Vec<u64> = vec![];
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let mut iterator = buffer.split(' ');
    iterator.next();
    for m in iterator {
        seeds.push(m.trim().parse::<u64>().unwrap());
    }

    seeds
}

fn get_mapping(name: &str, content: &str) -> MyMap {
    let re = Regex::new(&("(?m)".to_owned() + name + r" map:\n((?:(?:\d+ )+\d+\n)+)")).unwrap();

    let mut map = MyMap {
        map_entries: vec![]
    };


    for (_, [m]) in re.captures_iter(content).map(|c| c.extract()) {
        let _: Vec<_> = m.split('\n').filter(|a| !a.is_empty()).map(|line| {
            let data: Vec<u64> = line.split(' ').filter(|a| !a.is_empty()).map(|f| {
                f.trim().parse::<u64>().unwrap()
            }).collect::<Vec<u64>>();

            map.map_entries.push(MyMapEntry {
                source_start: data[1],
                destination_start: data[0],
                range: data[2]
            });
        }).collect();
    }

    map
}



pub fn run() -> Result<u64, Error> {
  let file = File::open("inputs/day5_input.txt")?;
  let content = fs::read_to_string("inputs/day5_input.txt")?;

  let seeds: Vec<u64> = get_seeds(&file);
  println!("seeds: {:?}", seeds);

  let seed_to_soil = get_mapping("seed-to-soil", &content);
  let soil_to_fertilizer = get_mapping("soil-to-fertilizer", &content);
  let fertilizer_to_water = get_mapping("fertilizer-to-water", &content);
  let water_to_light = get_mapping("water-to-light", &content);
  let light_to_temperature = get_mapping("light-to-temperature", &content);
  let temperature_to_humidity = get_mapping("temperature-to-humidity", &content);
  let humidity_to_location = get_mapping("humidity-to-location", &content);

  let min_location = seeds.iter().map(|s| {
    println!("\n\nseed: {:?}", s);
    let soil = seed_to_soil.map(*s);
    println!("soil: {:?}", soil);
    let fertilizer = soil_to_fertilizer.map(soil);
    println!("fertilizer: {:?}", fertilizer);
    let water = fertilizer_to_water.map(fertilizer);
    println!("water: {:?}", water);
    let light = water_to_light.map(water);
    println!("light: {:?}", light);
    let temperature = light_to_temperature.map(light);
    println!("temperature: {:?}", temperature);
    let humidity = temperature_to_humidity.map(temperature);
    println!("humidity: {:?}", humidity);
    let location = humidity_to_location.map(humidity);
    println!("location: {:?}", location);
    location
  }).min().unwrap();


  println!("min_location: {:?}", min_location);


  Ok(min_location)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_outputs_the_proper_result() {
        assert_eq!(run().unwrap(), 309796150);
    }
}

