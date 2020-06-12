use crate::common::op::Op;
use crate::common::resource::Resource;
use crate::areas::agile_project_get::GetOp;
use crate::areas::agile_project_list::ListOp;

pub struct ProjectResource {
    area_name: String,
    ops: std::vec::Vec<Box<dyn Op>>,
}

impl ProjectResource {
    pub fn new(area_name: &str) -> ProjectResource {
        let mut resource = ProjectResource {
            area_name: String::from(area_name),
            ops: vec![],
        };
        resource.ops.push(Box::new(GetOp::new(resource.get_area_name(), resource.get_name())));
        resource.ops.push(Box::new(ListOp::new(resource.get_area_name(), resource.get_name())));
        resource
    }
}

impl Resource for ProjectResource {
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

