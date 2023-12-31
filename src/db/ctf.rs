#[derive(Debug)]
pub struct Ctf {
    pub name: String,
    pub challenges: HashMap<String, bool>
}

impl Ctf {
    pub fn new(name: String) -> Ctf {
        Ctf {
            name: name,
            challenges: HashMap::new()
        }
    }

    pub fn add_challenge(&mut self, name: String, solved: bool) {
        self.challenges.insert(name, solved);
    }
}
