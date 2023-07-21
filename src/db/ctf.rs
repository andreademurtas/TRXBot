pub struct Ctf {
    pub name: String,
    pub challenges: HashMap<String, bool>,
    pub participants: Vec<String>
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

    pub fn add_participant(&mut self, name: String) {
        self.participants.push(name);
    }
}
