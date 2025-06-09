#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    CITIZEN = 0,
    UNDERCOVER,
    WHITE,
}

impl Role {
    pub fn to_string(&self) -> String {
        match self {
            Role::CITIZEN => { "CIVIL".into() }
            Role::UNDERCOVER => { "UNDERCOVER".into() }
            Role::WHITE => { "MR. WHITE".into() }
        }
    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    role: Role,
    alive: bool,
}

impl Player {
    fn assert_name(name: &String) {
        if name.is_empty() { panic!("Name cannot be empty"); }
        if name.len() > 30 { panic!("Name should be less than 30 bytes long"); }
    }

    pub fn new(name: String, role: Role) -> Player {
        Self::assert_name(&name);

        Player {
            name,
            role,
            alive: true,
        }
    }

    pub fn name(&self) -> &String { &self.name }
    pub fn set_name(&mut self, name: String) {
        Self::assert_name(&name);
        self.name = name;
    }

    pub fn role(&self) -> &Role { &self.role }
    pub fn set_role(&mut self, role: Role) {
        self.role = role;
    }

    pub fn alive(&self) -> &bool { &self.alive }
    pub fn set_alive(&mut self, val: bool) { self.alive = val; }
}
