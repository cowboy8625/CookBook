fn main() {}

trait Magic {
    fn cast(&self);
}

trait Melee {
    fn swing(&self);
}

struct Wizard;
impl Magic for Wizard {
    fn cast(&self) {
        println!("Cast Spell");
    }
}
struct Knight;

impl Melee for Knight {
    fn swing(&self) {
        println!("Swung sword");
    }
}
struct SpellSword;

impl Magic for SpellSword {
    fn cast(&self) {
        println!("Cast Spell");
    }
}
