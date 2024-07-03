use rand::Rng;

struct RandGen {
    seed: u64,
    rng: rand::rngs::StdRng,
}

impl RandGen {
    fn new(seed: u64) -> Self {
        RandGen {
            seed,
            rng: rand::SeedableRng::seed_from_u64(seed),
        }
    }
    fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        self.rng.gen_range(range)
    }
}

struct Urna<T> {
    items: Vec<T>,
    rand_gen: RandGen,
}

impl<T> Urna<T> {
    fn new(rand_gen: RandGen) -> Self {
        Urna {
            items: Vec::new(),
            rand_gen,
        }
    }
    fn doloz(&mut self, new_item: T) {
        self.items.push(new_item)
    }
    fn rozmiar(&mut self) -> usize {
        self.items.len()
    }
    fn losuj_bez_usuwania(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            None
        } else {
            let id_of_item = self.rand_gen.gen_range(0..self.items.len());
            Some(&self.items[id_of_item])
        }
    }
    fn losuj_z_usuwaniem(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            let id_of_item = self.rand_gen.gen_range(0..self.items.len());
            Some(self.items.swap_remove(id_of_item))
        }
    }
}

#[derive(Debug)]
enum Moneta {
    Orzel,
    Reszka,
}

fn main() {
    let mut example_urna = Urna::<i32>::new(RandGen::new(123));
    example_urna.doloz(10);
    example_urna.doloz(20);
    example_urna.doloz(30);
    example_urna.doloz(40);
    println!("Rozmiar urny: {}", example_urna.rozmiar());
    if let Some(item) = example_urna.losuj_z_usuwaniem() {
        println!("Wylosowany element z usuwaniem: {}", item);
    } else {
        println!("Urna jest pusta.");
    }
    if let Some(&item) = example_urna.losuj_bez_usuwania() {
        println!("Wylosowany element bez usuwania: {}", item);
    } else {
        println!("Urna jest pusta.");
    }
    println!("Rozmiar urny po usunieciu: {}", example_urna.rozmiar());

    let mut urna_moneta = Urna::<Moneta>::new(RandGen::new(123));
    urna_moneta.doloz(Moneta::Orzel);
    urna_moneta.doloz(Moneta::Reszka);
    println!("Urna<Moneta> rozmiar: {}", urna_moneta.rozmiar());
    if let Some(item) = urna_moneta.losuj_bez_usuwania() {
        println!("Wylosowany element Urna<Moneta> bez usuwania: {:?}", item);
    } else {
        println!("Urna<Moneta> jest pusta.");
    }
}
