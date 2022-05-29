use rand::Rng;

#[derive(Debug)]  // see also #[derive(Debug, Copy, Clone)]
// Pokemons can't derive Copy because it contains a String (String, like Vec and any other variable-sized container, contains a pointer to some variable amount of heap memory)
// The only correct way to copy a String is to allocate a new block of heap memory to copy all the characters into, which is what String's Clone implementation does.
// someone recommends to read this to be productive with Rust : https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
pub struct Pokemon {
    pub name: String,
    pub hp: i32,
    pub mp: Option<i32>
}

impl Pokemon {

    /// Create a Pokemon
    pub fn new(name: &str, hp: i32) -> Pokemon {
        Pokemon {
            name : name.to_string(),
            hp : hp,
            mp : Some(5)
        }
    }

    pub fn details(&self) -> String {
        //! Return Pokemon details
        format!("{}: {}hp {}mp", self.name, self.hp, self.mp.unwrap_or(0))
    }
}

trait Fighter {
    fn get_hp(self) -> i32;
    fn strength(self) -> i32;
}

impl Fighter for Pokemon {
    fn get_hp(self) -> i32 {
        self.hp
    }
    fn strength(self) -> i32 {
        let i = rand::thread_rng().gen_range(0..100);
        i
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


#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::Pokemon;
    use super::Fighter;
    use super::say_hello;
    use super::evolve;

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
}
