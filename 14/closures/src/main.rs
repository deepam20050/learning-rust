use std::slice::SplitMut;

struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f64,
}

impl City {
    fn compare_name(&self, other_name: String) -> String {
        let new_s = format!("{}{}", self.name, other_name);
        new_s
    }
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities(cities: &mut Vec<City>) {
    // cities.sort_by_key(city_population_descending);
    // better way to write using closures (anonymous functions)
    cities.sort_by_key(|city: &City| -city.population);
}

fn sort_by_statstic(cities: &mut Vec<City>, other_name: String) {
    cities.sort_by_key(|city| city.compare_name(other_name.clone()));
}

// a function can take another functions as an argument
// to support closures we need to add the following modification
fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
where
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

fn has_monster_attacks(city: &City) -> bool {
    city.monster_attack_risk > 0.0
}

fn main() {
    let cities = Vec::<City>::new();
    let n = count_selected_cities(&cities, has_monster_attacks);
    println!("{}", n);
    println!("Hello, world!");

    println!(
        "{}",
        count_selected_cities(&cities, |city| { city.monster_attack_risk > 0.0 })
    );

    let mut i = 0;
    let mut incr = || {
        i += 1; // incr borrows a mut reference to i
        println!("Ding! i is now: {}", i);
    };
    incr();
    incr();

    // fn(&City) -> bool // fn type (functions only)
    // Fn(&City) -> bool // Fn trait (both functions and closures)
}
