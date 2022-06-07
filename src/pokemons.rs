use rand::Rng;
use std::fmt::{Formatter, Display};
use std::fmt;

#[derive(Debug)]  // see also #[derive(Debug, Copy, Clone)]
// Pokemons can't derive Copy because it contains a String (String, like Vec and any other variable-sized container, contains a pointer to some variable amount of heap memory)
// The only correct way to copy a String is to allocate a new block of heap memory to copy all the characters into, which is what String's Clone implementation does.
// someone recommends to read this to be productive with Rust : https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
pub struct Pokemon {
    pub name: String,
    pub hp: i32,
    pub mp: Option<i32>,
}

impl Pokemon {
    /// Create a Pokemon
    pub fn new(name: &str, hp: i32) -> Pokemon {
        Pokemon {
            name: name.to_string(),
            hp: hp,
            mp: Some(5),
        }
    }

    pub fn details(&self) -> String {
        //! Return Pokemon details
        format!("{}: {}hp {}mp", self.name, self.hp, self.mp.unwrap_or(0))
    }
}


impl Display for Pokemon {
    // Display is similar to Debug, but Display is for user-facing output, and so cannot be derived.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

trait Fighter {
    fn get_hp(&self) -> i32;
    fn strength(&self) -> i32;
}


impl Fighter for Pokemon {
    fn get_hp(&self) -> i32 {  // without &, get_hp() would take ownership of self...
        self.hp
    }
    fn strength(&self) -> i32 {  // without &, strength() would take ownership of self...
        let i = rand::thread_rng().gen_range(0..100);
        i
    }
}

trait Comparable {
    fn compare_to(self: &Self, other: &Self) -> bool;  // 'Self' is the type that implements Comparable
}

impl Comparable for Pokemon {
    fn compare_to(self: &Pokemon, other: &Pokemon) -> bool {
        return self.get_hp() == other.get_hp();
    }
}

fn say_hello(p: &Pokemon) {
    println!("hello {}", p.name);
}

fn evolve(p: &mut Pokemon) {
    if p.name == "Taupiqueur" {
        p.name = "Triopikeur".to_string();
    }
}


fn concat<T: Display>(array: &[T]) -> String {
    // why returning a String (and not a &str) ?
    // basically, &str is for immutatble strings, String is for mutable strings
    // BUT you cannot return a &str if you've allocated the String in the function. (https://stackoverflow.com/a/43080280)
    let str = array
        .iter()// transform to iterator (with unknown size)
        .map(|x| format!("{}", x))  // map
        .fold(String::new(), |a, b| a + ", " + b.as_str());  // reduce
    return String::from(&str[2..]); // remove the 2 first characters and return a String
}


#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;

    #[test]
    fn verify_greet() {
        let details = Pokemon::new("Taupiqueur", 10).details();
        assert_eq!(details, "Taupiqueur: 10hp 5mp")
    }

    #[test]
    fn verify_trait_fighter() {
        let strength = Pokemon::new("Taupiqueur", 10).strength();
        assert!(strength >= 0);
        assert!(strength <= 100);
    }

    #[test]
    fn verify_trait_debug() {
        let taupi = Pokemon::new("Taupiqueur", 10);
        let formated = format!("{:?}", taupi);  // breaks without the Debug trait
        assert!(formated.contains("Pokemon { name: \"Taupiqueur\", "));
    }

    #[test]
    fn verify_say_hello_ref() {
        let taupi = Pokemon::new("Taupiqueur", 10);
        say_hello(&taupi);  // `taupi` is borrowed for the time of the call
        say_hello(&taupi);  // works because we just pass a reference to say_hello
    }

    #[test]
    fn verify_evolve_mut_ref() {
        let mut taupi = Pokemon::new("Taupiqueur", 10);
        evolve(&mut taupi);  // `taupi` is borrowed mutably, works because `taupi` is declared mutable
        say_hello(&taupi);  // works because we just pass a reference to say_hello
        assert_eq!(taupi.name, "Triopikeur");
    }

    #[test]
    fn verify_copy() {
        let n = 7;
        let o = n;  // `o` is a copy of `n`
        let p = n;  // same. `n` is neither moved nor borrowed. (Can't work with a Pokemon since it misses the Copy trait)
        assert_eq!(o, 7);
        assert_eq!(p, 7);
    }

    #[test]
    fn verify_compare() {
        let taupi1 = Pokemon::new("Taupiqueur", 10);
        let taupi2 = Pokemon::new("Taupiqueur", 10);
        let trio = Pokemon::new("Triopikeur", 20);

        assert_eq!(taupi1.compare_to(&taupi2), true);
        assert_eq!(taupi1.compare_to(&trio), false);
    }

    #[test]
    fn verify_display_trait() {
        let taupi = Pokemon::new("Taupiqueur", 10);
        assert_eq!(format!("{}", taupi), "Taupiqueur");
    }

    #[test]
    fn verify_concat() {
        let array = [Pokemon::new("Taupiqueur", 10), Pokemon::new("Triopikeur", 20), Pokemon::new("Colossinge", 15)];
        assert_eq!(concat(&array), "Taupiqueur, Triopikeur, Colossinge");
    }


}
