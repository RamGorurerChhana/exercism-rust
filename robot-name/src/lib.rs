use rand::Rng;

// since we want to remember all previously generated names
// declare a static variable in global scope
static mut ROBOT_NAMES: Vec<String> = vec![];

// push a name into the global variable
fn insert_robot_name(name: &String) {
    unsafe {
        ROBOT_NAMES.push(name.clone());
    }
}

// check if a name already exists
fn check_robot_name(name: &String) -> bool {
    unsafe { ROBOT_NAMES.contains(name) }
}

// generate a new name
// check if it already exists
// if not return otherwise try again
fn generate_name() -> String {
    loop {
        let fl = rand::thread_rng().gen_range('A'..='Z');
        let sl = rand::thread_rng().gen_range('A'..='Z');
        let number = rand::thread_rng().gen_range(100..=1000);
        let name = format!("{fl}{sl}{number}");
        if !check_robot_name(&name) {
            return name;
        }
    }
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let name = generate_name();
        insert_robot_name(&name);
        Self { name }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        let name = generate_name();
        insert_robot_name(&name);
        self.name = name;
    }
}
