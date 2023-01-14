use std::collections::HashSet;

use super::AOCSolutions; 

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position {
    x: i64, 
    y: i64, 
}

impl Position {
   pub fn increment_x(&mut self) {
      self.x += 1;
   }

   pub fn decrement_x(&mut self) {
      self.x -= 1;
   }

   pub fn increment_y(&mut self) {
      self.y += 1;
   }

   pub fn decrement_y(&mut self) {
      self.y -= 1;
   }

   pub fn manhattan_dist(&self, other: &Self) -> u64 {
      i64::abs_diff(self.x, other.x) + i64::abs_diff(self.y, other.y)
   }
}

#[derive(Debug, Clone, Copy, Hash)]
struct LinearEquation {
   slope: i64, 
   pass_through: Position, 
}

impl LinearEquation {
   pub fn from(p1: &Position, p2: &Position) -> LinearEquation {
      let pass_through = p1.clone(); 
      let slope = (p2.y - p1.y) / (p2.x - p1.x); 
      LinearEquation { slope, pass_through }
   }

   pub fn solve_at(&self, x: i64) -> i64 {
      (x - self.pass_through.x) * self.slope + self.pass_through.y
   }

   pub fn intersection(&self, other: &Self) -> Option<Position> {
      if self.slope == other.slope { return None; }
      let x = 
         (self.slope * self.pass_through.x - other.slope * other.pass_through.x - self.pass_through.y + other.pass_through.y) / 
         (self.slope - other.slope); 
      let y = self.solve_at(x); 
      return Some(Position { x, y }); 
   }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Sensor {
   sensor_pos: Position, 
   beacon_pos: Position, 
   beacon_dist: u64, 
}

impl Sensor {
   pub fn from_line(line: &str) -> Sensor {
      let mut line_itr = line.split(": ").take(2);
      let sensor_str = line_itr.next()
         .expect(&format!("[day15::Sensor::from_line] Malformed line \"{}\"", line)); 
      let beacon_str = line_itr.next()
         .expect(&format!("[day15::Sensor::from_line] Malformed line \"{}\"", line)); 
      let collect_into_coord_pair = |s: &str| { s.split_whitespace()
         .filter_map(|s| {
            if s.starts_with("x=") || s.starts_with("y=") {
               Some(s.trim_matches(|c| ['x', 'y', '=', ','].contains(&c))
                  .parse::<i64>()
                  .expect(&format!("[day15::Sensor::from_line] Malformed line \"{}\"", line))
               )
            } else {
               None
            }
         })
         .take(2)
         .collect::<Vec<i64>>()
      }; 

      let sensor_coords: Vec<i64> = collect_into_coord_pair(sensor_str); 
      let beacon_coords: Vec<i64> = collect_into_coord_pair(beacon_str); 
      if sensor_coords.len() < 2 || beacon_coords.len() < 2 {
         panic!("[day15::Sensor::from_line] Malformed line \"{}\"", line)
      }

      let sensor_pos = Position { x: sensor_coords[0], y: sensor_coords[1] }; 
      let beacon_pos = Position { x: beacon_coords[0], y: beacon_coords[1] }; 
      let dist = sensor_pos.manhattan_dist(&beacon_pos); 
      Sensor { sensor_pos, beacon_pos, beacon_dist: dist }
   }

   fn find_impossible_beacon_coords_along_axis(&self, x_axis: Option<i64>, y_axis: Option<i64>) -> Vec<Position> {
      match (x_axis, y_axis) {
         (Some(x), None) => {
            let min_pos_to_x = Position { x, y: self.sensor_pos.y };
            let mut manh_dist = min_pos_to_x.manhattan_dist(&self.sensor_pos); 
            if manh_dist > self.beacon_dist { return vec![]; }

            let (mut x_up, mut x_down): (Position, Position) = (min_pos_to_x.clone(), min_pos_to_x.clone()); 
            let mut impossible_coords: Vec<Position> = vec![min_pos_to_x]; 
            loop {
               manh_dist += 1; 
               if manh_dist > self.beacon_dist { break; }
               x_up.increment_y(); 
               if self.beacon_pos != x_up { impossible_coords.push(x_up.clone()); }
               x_down.decrement_y(); 
               if self.beacon_pos != x_down { impossible_coords.push(x_down.clone()); }
            }
            return impossible_coords; 
         }, 
         (None, Some(y)) => {
            let min_pos_to_y = Position { x: self.sensor_pos.x, y }; 
            let mut manh_dist = min_pos_to_y.manhattan_dist(&self.sensor_pos); 
            if manh_dist > self.beacon_dist { return vec![]; }

            let (mut y_left, mut y_right): (Position, Position) = (min_pos_to_y.clone(), min_pos_to_y.clone()); 
            let mut impossible_coords: Vec<Position> = vec![min_pos_to_y]; 
            loop {
               manh_dist += 1; 
               if manh_dist > self.beacon_dist { break; }
               y_right.increment_x(); 
               if self.beacon_pos != y_right { impossible_coords.push(y_right.clone()); }
               y_left.decrement_x(); 
               if self.beacon_pos != y_left { impossible_coords.push(y_left.clone()); }
            }
            return impossible_coords; 
         }, 
         (Some(x), Some(y)) => {
            let fixed_pos = Position { x, y }; 
            if fixed_pos.manhattan_dist(&self.sensor_pos) <= self.beacon_dist { 
               return vec![fixed_pos]; 
            } else {
               return vec![]; 
            }
         }, 
         (None, None) => unimplemented!(), 
      }
   }

   fn find_peripheries(&self) -> Vec<LinearEquation> {
      let dist: i64 = (self.beacon_dist + 1).try_into().unwrap();
      let top = Position { x: self.sensor_pos.x, y: self.sensor_pos.y + dist }; 
      let bottom = Position { x: self.sensor_pos.x, y: self.sensor_pos.y - dist }; 
      let left = Position { x: self.sensor_pos.x - dist, y: self.sensor_pos.y }; 
      let right = Position { x: self.sensor_pos.x + dist, y: self.sensor_pos.y }; 

      return vec![
         LinearEquation::from(&top, &right), 
         LinearEquation::from(&right, &bottom), 
         LinearEquation::from(&bottom, &left), 
         LinearEquation::from(&left, &top), 
      ]; 
   }
}


pub struct Day15; 

impl AOCSolutions for Day15 {
   fn get_star_1(input: &str) -> Result<i64, ()> {
      let y_axis = 2_000_000 as i64; 
      let mut impossible_loc_set: HashSet<Position> = HashSet::new(); 
      let sensors: Vec<Sensor> = input.lines().map(Sensor::from_line).collect(); 
      let known_beacons: HashSet<Position> = sensors.iter().map(|s| s.beacon_pos.clone()).collect(); 

      for sensor in sensors {
         let impossibles = sensor.find_impossible_beacon_coords_along_axis(None, Some(y_axis)); 
         impossibles.into_iter().for_each(|p| { 
            if !known_beacons.contains(&p) { impossible_loc_set.insert(p); }
         }); 
      }

      return Ok(impossible_loc_set.len().try_into().unwrap()); 
   }

   fn get_star_2(input: &str) -> Result<i64, ()> {
      let xy_range = 0..4_000_000 as i64; 
      let sensors: Vec<Sensor> = input.lines().map(Sensor::from_line).collect(); 
      let mut linear_eqns: Vec<LinearEquation> = Vec::with_capacity(sensors.len() * 4); 
      for sensor in sensors.iter() {
         linear_eqns.append(&mut sensor.find_peripheries()); 
      }

      let mut candidate_set: HashSet<Position> = HashSet::new(); 
      for eqn_1 in linear_eqns.iter() {
         for eqn_2 in linear_eqns.iter() {
            if let Some(p) = eqn_1.intersection(eqn_2) {
               if xy_range.contains(&p.x) && xy_range.contains(&p.y) { candidate_set.insert(p); } 
            }
         }
      }

      for p in candidate_set {
         if sensors.iter().all(|s| s.sensor_pos.manhattan_dist(&p) > s.beacon_dist ) {
            println!("[Day15::get_star_2] Found candidate at `{:?}`", p);
            return Ok(p.x * 4_000_000 + p.y);  
         }
      }
      eprintln!("[Day15::get_star_2] Cannot find candidate in give input"); 
      return Err(()); 
   }
}

#[cfg(test)]
mod tests {
   use super::*; 

   const SAMPLE_INPUT: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

   #[test]
   fn test_get_star_1() {
      assert_eq!(Day15::get_star_1(SAMPLE_INPUT).unwrap(), 26); 
   }

   #[test]
   fn test_get_star_2() {
      assert_eq!(Day15::get_star_2(SAMPLE_INPUT).unwrap(), 5600_0011); 
   }
}
