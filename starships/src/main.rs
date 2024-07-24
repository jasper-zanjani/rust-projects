#[derive(Debug)]
enum Class { 
    BIRD_OF_PREY,
    GALAXY,
}

#[derive(Debug)]
enum Affiliation {
    FEDERATION,
    KLINGON,
    ROMULAN
}

#[derive(Debug)]
struct Starship {
    pub name: String,
    class: Class,
    affiliation: Affiliation,
    hp: u8,
}

trait MissionCapable {
    fn deploy(&self) -> () {
        println!("Deploying...");
    }

    fn attack(&self, other: &mut Starship) -> () {
        println!("Attacking...");
    }
}

impl MissionCapable for Starship {
    fn deploy (&self) -> () {
        println!("{} embarking on a voyage of discovery..", self.name);
    }
    fn attack(&self, other: &mut Starship) -> () {
        let damage = rand::random::<u8>();
        other.hp = &other.hp - damage;
        println!("{} attacked {}, doing {} damage!", self.name, other.name, damage);
    }
}



fn main() {
    let mut enterprise = Starship{
        name: String::from("USS Enterprise"),
        affiliation: Affiliation::FEDERATION,
        class: Class::GALAXY,
        hp: 255,
    };

    let mut klingon = Starship {
        name: String::from("IKS Pagh"),
        affiliation: Affiliation::KLINGON,
        class: Class::BIRD_OF_PREY,
        hp: 255,
    };
    enterprise.deploy();
    klingon.attack(&mut enterprise);

    println!("{:?}", enterprise);
}
