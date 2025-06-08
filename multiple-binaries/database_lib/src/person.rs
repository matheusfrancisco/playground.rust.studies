use crate::project::Project;


#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub projects: Vec<Project>,
}
