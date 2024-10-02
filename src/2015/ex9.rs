use std::{collections::{HashMap, HashSet}, usize::MIN };

fn main() {
  let contents = std::fs::read_to_string("input/2015/9").unwrap();
  let mut distances: HashMap<String,usize> = HashMap::new();
  let mut places : HashSet<&str> = HashSet::new();
  //parse contents
  for line in contents.lines() {
    let place_names: Vec<&str> = line.split_whitespace().collect();
    let place_a = place_names[0];
    let place_b = place_names[2];
    let distance_a_b = place_names[4].parse::<usize>().unwrap();
    if !places.contains(place_a) { places.insert(place_a); }
    if !places.contains(place_b) { places.insert(place_b); }
    let place_a_to_b  = place_a.to_string() + place_b;
    let place_b_to_a = place_b.to_string() + place_a;
    distances.insert(place_a_to_b, distance_a_b);
    distances.insert(place_b_to_a, distance_a_b);
  }

  println!("{:?}", places);
  println!("{:?}", distances);

  let vec_places: Vec<&str> = places.into_iter().map(|x| {x}).collect();


  let permutations = permutations::permut(vec_places);
  let mut max= MIN;
  for permutation in permutations {
    let mut acc = 0;
    for i in 0..permutation.len() - 1{
      let mut curr = permutation[i].to_string();
      curr.push_str(permutation[i+1]);
      acc += distances.get(&curr).unwrap();
    }
    if max < acc {
      max = acc;
      println!("acc={acc:?}");
    }
  } 
  println!("{max:?}");
}

mod permutations {
  pub fn permut<T:Clone>(v: Vec<T>) -> Vec<Vec<T>> {
    if v.len() <= 1 {
      return vec![v]
    }
    let mut res: Vec<Vec<T>> = vec![];
    for i in 0..v.len() {
      let mut rem = v.clone();
      let current = rem.remove(i);
      let mut perms = self::permut(rem);
      for perm in perms.iter_mut(){
        perm.insert(0, current.clone());
      }
      res.append(&mut perms);
    }
    res
  }
}