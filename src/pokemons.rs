use rand::Rng;

//#[derive(Copy)]  // see also #[derive(Debug, Copy, Clone)]
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


#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::Pokemon;
    use super::Fighter;

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

    /*#[test]
    fn verify_trait_copy() {
        let taupi = Pokemon::new("Taupiqueur", 10);
        let taupi_2 = taupi;
        assert_eq!(taupi.hp, 10);
    }*/
}
