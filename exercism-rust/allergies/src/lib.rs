pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergen {
    fn all() -> &'static [Allergen] {
        static ALL: &[Allergen] = &[
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        ALL
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score % 256)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // let c = *allergen as u32;
        // self.0 & c == c
        self.0 & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all()
            .iter()
            .cloned()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
