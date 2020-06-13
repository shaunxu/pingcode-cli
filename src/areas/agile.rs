use crate::common::area::Area;
use crate::common::resource::Resource;
use crate::areas::agile_projects::AgileProjectsResource;
use crate::areas::agile_epics::AgileEpicsResource;

pub struct AgileArea {
    resources: std::vec::Vec<Box<dyn Resource>>,
}

impl AgileArea {
    pub fn new() -> AgileArea {
        let mut area = AgileArea {
            resources: vec![]
        };
        area.resources.push(Box::new(AgileProjectsResource::new(area.get_name())));
        area.resources.push(Box::new(AgileEpicsResource::new(area.get_name())));
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
