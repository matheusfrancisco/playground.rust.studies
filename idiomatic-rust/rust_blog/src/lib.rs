use rand::prelude::*;
#[derive(Debug)]
pub enum Role {
    Guest,
    Viewer,
    Creator,
    Admin,
}

#[derive(Debug)]
pub struct User {
    id: u32,
    name: String,
    role: Role,
}

impl User {
    pub fn new(name: String, role: Role) -> Result<Self, String> {
        if name == "chico" {
            return Err("Invalid username".to_owned());
        }
        Ok(Self {
            id: thread_rng().gen_range(0..999999999),
            name,
            role,
        })
    }
}

impl Default for User {
    fn default() -> Self {
        let id = thread_rng().gen_range(0..999999999);

        Self {
            id,
            name: format!("guest{id}"),
            role: Role::Guest,
        }
    }
}

#[derive(Debug, Default, derive_new::new)]
pub struct Post {
    content: String,
   #[new(value = "vec![\"rusty\".to_owned()]")]
    tags: Vec<String>,
   #[new(default)]
    likes: u32,
}
