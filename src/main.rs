extern crate rand;

mod pokemons;


fn main() {

    use pokemons::Pokemon;

    let mut p = Pokemon::new("Taupiqueur", 10);
    println!("{}", p.details());

    {
        let p2 = &mut p;
        p2.name += "---";

        println!("{}", p2.details());

    }
    println!("{}", p.details());

}
