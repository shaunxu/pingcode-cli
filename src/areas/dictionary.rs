use crate::areas::dictionary_user::UserResource;
use crate::common::area::Area;
use crate::common::resource::Resource;

pub struct DictionaryArea {
    resources: std::vec::Vec<Box<dyn Resource>>,
}

impl DictionaryArea {
    pub fn new() -> DictionaryArea {
        let mut area = DictionaryArea {
            resources: vec![]
        };
        let user = UserResource::new(area.get_name());
        area.resources.push(Box::new(user));
        area
    }
}

impl Area for DictionaryArea {
    fn get_name(&self) -> &str {
        "dictionary"
    }

    fn get_description(&self) -> &str {
        "Manage dictionary infomation (user, role, etc.)"
    }

    fn get_resources(&self) -> &std::vec::Vec<Box<dyn Resource>> {
        &self.resources
    }

}
