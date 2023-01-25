
pub struct Item {
    name: String,
    description: String,
}

impl Item {

    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_desc(&self) -> &str {
        &self.description
    }
}

pub struct Player {
    name: String,
    inventory: Vec<Item>
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            inventory: vec![
                Item::new(String::from("Apple"), String::from("A shiny red fruit")),
                Item::new(String::from("Box"), String::from("A cardboard box"))
            ]
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_inventory(&self) -> &Vec<Item> {
        &self.inventory
    }

    pub fn _loadinventory(&mut self, inventory: Vec<Item>) {
        self.inventory = inventory
    }
}