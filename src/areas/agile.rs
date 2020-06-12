use crate::common::area::Area;
use crate::common::resource::Resource;
use crate::areas::agile_project::ProjectResource;

pub struct AgileArea {
    resources: std::vec::Vec<Box<dyn Resource>>,
}

impl AgileArea {
    pub fn new() -> AgileArea {
        let mut area = AgileArea {
            resources: vec![]
        };
        let project = ProjectResource::new(area.get_name());
        area.resources.push(Box::new(project));
        area
    }
}

impl Area for AgileArea {
    fn get_name(&self) -> &str {
        "agile"
    }

    fn get_description(&self) -> &str {
        "Manage your agile projects and workitems"
    }

    fn get_resources(&self) -> &std::vec::Vec<Box<dyn Resource>> {
        &self.resources
    }

}
