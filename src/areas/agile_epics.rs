use crate::common::op::Op;
use crate::common::resource::Resource;
use crate::areas::agile_epics_create::AgileEpicsCreateGetOp;

pub struct AgileEpicsResource {
    area_name: String,
    ops: std::vec::Vec<Box<dyn Op>>,
}

impl AgileEpicsResource {
    pub fn new(area_name: &str) -> AgileEpicsResource {
        let mut resource = AgileEpicsResource {
            area_name: String::from(area_name),
            ops: vec![],
        };
        resource.ops.push(Box::new(AgileEpicsCreateGetOp::new(resource.get_area_name(), resource.get_name())));
        // resource.ops.push(Box::new(AgileProjectsListOp::new(resource.get_area_name(), resource.get_name())));
        resource
    }
}

impl Resource for AgileEpicsResource {
    fn get_area_name(&self) -> &str {
        &self.area_name
    }
    fn get_name(&self) -> &str {
        "epics"
    }

    fn get_description(&self) -> &str {
        "Manage your epics in an agile project"
    }

    fn get_ops(&self) -> &std::vec::Vec<std::boxed::Box<(dyn Op)>> {
        &self.ops
    }
}

