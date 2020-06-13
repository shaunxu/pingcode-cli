use crate::common::op::Op;
use crate::common::resource::Resource;
use crate::areas::agile_projects_get::AgileProjectsGetOp;
use crate::areas::agile_projects_list::AgileProjectsListOp;

pub struct AgileProjectsResource {
    area_name: String,
    ops: std::vec::Vec<Box<dyn Op>>,
}

impl AgileProjectsResource {
    pub fn new(area_name: &str) -> AgileProjectsResource {
        let mut resource = AgileProjectsResource {
            area_name: String::from(area_name),
            ops: vec![],
        };
        resource.ops.push(Box::new(AgileProjectsGetOp::new(resource.get_area_name(), resource.get_name())));
        resource.ops.push(Box::new(AgileProjectsListOp::new(resource.get_area_name(), resource.get_name())));
        resource
    }
}

impl Resource for AgileProjectsResource {
    fn get_area_name(&self) -> &str {
        &self.area_name
    }
    fn get_name(&self) -> &str {
        "projects"
    }

    fn get_description(&self) -> &str {
        "Manage your agile projects"
    }

    fn get_ops(&self) -> &std::vec::Vec<std::boxed::Box<(dyn Op)>> {
        &self.ops
    }
}

