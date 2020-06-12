use crate::common::op::Op;
use crate::common::resource::Resource;
use crate::areas::dictionary_user_get::GetOp;
use crate::areas::dictionary_user_list::ListOp;

pub struct UserResource {
    area_name: String,
    ops: std::vec::Vec<Box<dyn Op>>,
}

impl UserResource {
    pub fn new(area_name: &str) -> UserResource {
        let mut resource = UserResource {
            area_name: String::from(area_name),
            ops: vec![],
        };
        resource.ops.push(Box::new(GetOp::new(resource.get_area_name(), resource.get_name())));
        resource.ops.push(Box::new(ListOp::new(resource.get_area_name(), resource.get_name())));
        resource
    }
}

impl Resource for UserResource {
    fn get_area_name(&self) -> &str {
        &self.area_name
    }
    fn get_name(&self) -> &str {
        "users"
    }

    fn get_description(&self) -> &str {
        "Manage users"
    }

    fn get_ops(&self) -> &std::vec::Vec<std::boxed::Box<(dyn Op)>> {
        &self.ops
    }
}

