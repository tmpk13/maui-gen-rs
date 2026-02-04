
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    name: String,
    view_dir_path: String,
    view_model_dir_path: String,
    view_name: String,
    view_model_name: String,
}

pub struct ProjectBuilder {
    name: String,
    view_name: String,
    view_dir_path: String,
    view_model_name: String,
    view_model_dir_path: String,
}

impl ProjectBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            view_dir_path: "Views/ExampleViews/".to_string(),
            view_model_dir_path: "Views/ExampleViewModels/".to_string(),
            view_name: "ExampleView".to_string(),
            view_model_name: "ExampleViewModel".to_string(),
        }
    }

    pub fn view_dir_path(mut self, path: &str) -> ProjectBuilder {
        self.view_dir_path = path.to_string();
        self
    }
    pub fn view_model_dir_path(mut self, path: &str) -> ProjectBuilder {
        self.view_model_dir_path = path.to_string();
        self
    }
    pub fn view_name(mut self, path: &str) -> ProjectBuilder {
        self.view_name = path.to_string();
        self
    }
    pub fn view_model_name(mut self, path: &str) -> ProjectBuilder {
        self.view_model_name = path.to_string();
        self
    }

    pub fn new_view(mut self, view_name: &str) -> ProjectBuilder {
        let view_path = format!("{}/{}Views/", self.name, view_name);
        let view_name = format!("{}View", view_name);
        let view_path = format!("{}/{}Views/", self.name, view_name);
        let view_path = format!("{}/{}Views/", self.name, view_name);

        self.view_dir_path = view_path;
        self.view_model_dir_path = "".to_string();
        self.view_name = view_name.to_string();
        self.view_model_name = "".to_string();
        self
    }

    pub fn category(mut self, view_name: &str) -> ProjectBuilder {
        self
    }

    pub fn build(self) -> Project {
        Project {
            name: self.name,
            view_dir_path: self.view_dir_path,
            view_model_dir_path: self.view_model_dir_path,
            view_name: self.view_name,
            view_model_name: self.view_model_name,
        }
    }
}