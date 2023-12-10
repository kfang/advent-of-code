use std::fs;
use std::ops::Range;

type DstSrcRng = (u64, u64, u64);

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("unable to read the file");

    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil: Vec<DstSrcRng> = Vec::new();
    let mut soil_to_fertizer: Vec<DstSrcRng> = Vec::new();
    let mut fertilizier_to_water: Vec<DstSrcRng> = Vec::new();
    let mut water_to_light: Vec<DstSrcRng> = Vec::new();
    let mut light_to_temperature: Vec<DstSrcRng> = Vec::new();
    let mut temperature_to_humidity: Vec<DstSrcRng> = Vec::new();
    let mut humidity_to_location: Vec<DstSrcRng> = Vec::new();

    let mut curr_map = "";

    for line in contents.lines() {
        if line.starts_with("seeds: ") {
            seeds = line
                .strip_prefix("seeds: ")
                .unwrap()
                .split(" ")
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .collect();
        }
        else if line.ends_with(" map:") {
            curr_map = line.strip_suffix(" map:").unwrap();
        }
        else if !line.is_empty() {
            let nums: Vec<u64> = line.split(" ").map(str::parse::<u64>).map(Result::unwrap).collect();
            let tup = (nums[0], nums[1], nums[2]);
            match curr_map {
                "seed-to-soil" => seed_to_soil.push(tup),
                "soil-to-fertilizer" => soil_to_fertizer.push(tup),
                "fertilizer-to-water" => fertilizier_to_water.push(tup),
                "water-to-light" => water_to_light.push(tup),
                "light-to-temperature" => light_to_temperature.push(tup),
                "temperature-to-humidity" => temperature_to_humidity.push(tup),
                "humidity-to-location" => humidity_to_location.push(tup),
                _ => println!("uh oh"),
            };
        }
    }

    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds.clone() {
        let soil = find_dest(&seed, &seed_to_soil);
        let fert = find_dest(&soil, &soil_to_fertizer);
        let wter = find_dest(&fert, &fertilizier_to_water);
        let lght = find_dest(&wter, &water_to_light);
        let temp = find_dest(&lght, &light_to_temperature);
        let hmdy = find_dest(&temp, &temperature_to_humidity);
        let locn = find_dest(&hmdy, &humidity_to_location);

        //println!("{}, soil: {}, fert: {}, wter: {}, lght: {}, temp: {}, hmdy: {}, locn: {}", seed, soil, fert, wter, lght, temp, hmdy, locn);
        locations.push(locn);
    }
    println!("P1: {}", locations.iter().min().unwrap());

    fix_gaps(&mut seed_to_soil);
    fix_gaps(&mut soil_to_fertizer);
    fix_gaps(&mut fertilizier_to_water);
    fix_gaps(&mut water_to_light);
    fix_gaps(&mut light_to_temperature);
    fix_gaps(&mut temperature_to_humidity);
    fix_gaps(&mut humidity_to_location);

    let seed_ranges = parse_seed_ranges(&seeds);
    let soil_ranges: Vec<Range<u64>> = seed_ranges.iter().flat_map(|s| find_dest_ranges(s, &seed_to_soil)).collect();
    let fert_ranges: Vec<Range<u64>> = soil_ranges.iter().flat_map(|s| find_dest_ranges(s, &soil_to_fertizer)).collect();
    let watr_ranges: Vec<Range<u64>> = fert_ranges.iter().flat_map(|s| find_dest_ranges(s, &fertilizier_to_water)).collect();
    let lght_ranges: Vec<Range<u64>> = watr_ranges.iter().flat_map(|s| find_dest_ranges(s, &water_to_light)).collect();
    let temp_ranges: Vec<Range<u64>> = lght_ranges.iter().flat_map(|s| find_dest_ranges(s, &light_to_temperature)).collect();
    let hmdy_ranges: Vec<Range<u64>> = temp_ranges.iter().flat_map(|s| find_dest_ranges(s, &temperature_to_humidity)).collect();
    let loct_ranges: Vec<Range<u64>> = hmdy_ranges.iter().flat_map(|s| find_dest_ranges(s, &humidity_to_location)).collect();

    let p2 = loct_ranges.iter().map(|r| r.start).min().unwrap();
    println!("P2: {}", p2);
}

fn fix_gaps(mapping: &mut Vec<DstSrcRng>) {
    let mut local = mapping.clone();
    local.sort_by_key(|tup| tup.1);

    let mut last_range: u64 = 0;
    for (_dst_min, src_min, r) in local {
        if last_range.lt(&src_min) {
            let diff = src_min - last_range;
            let tup = (last_range, last_range, diff);
            mapping.push(tup);
        }
        last_range = src_min + r;
    }
    mapping.push((last_range, last_range, 1000000000));
}

fn parse_seed_ranges(seeds: &Vec<u64>) -> Vec<Range<u64>> {
    seeds.chunks(2).map(|chunk| {
        let start = chunk[0];
        let end = chunk[0] + chunk[1];
        Range { start, end }
    }).collect()
}

fn find_dest_ranges(src: &Range<u64>, mapping: &Vec<DstSrcRng>) -> Vec<Range<u64>> {
    let mut start = src.start;
    let mut res = Vec::new();

    while start < src.end {
        let r = mapping.iter().find(|(_dst_min, src_min, r)| {
            src_min.le(&start) && (src_min + r).gt(&start)
        });

        if r.is_none() {
            res.push(Range { start: start, end: start });
            start += 1;
        } else {
            let (dst_min, src_min, r) = r.unwrap();

            let diff_from_start = start - src_min;
            let dst_start = dst_min + diff_from_start;

            let seed_range_left = src.end - start;
            let dst_range_left = r - (diff_from_start);
            let range_left = u64::min(seed_range_left, dst_range_left);

            let dst_end = dst_start + range_left;

            res.push(Range { start: dst_start, end: dst_end });
            start += range_left;
        }

        // println!("next start: {} of {};", start, src.end);
    }

    return res;
}

fn find_dest(src: &u64, mapping: &Vec<DstSrcRng>) -> u64 {
    for (dst_min, src_min, range) in mapping {
        let src_max = src_min + range;
        if src_min.le(src) && src_max.gt(src) {
            let diff = src - src_min;
            return dst_min + diff;
        }
    }

    return src.to_owned();
}
