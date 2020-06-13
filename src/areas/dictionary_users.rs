use crate::common::op::Op;
use crate::common::resource::Resource;
use crate::areas::dictionary_users_get::DictionaryUsersGetOp;
use crate::areas::dictionary_users_list::DictionaryUsersListOp;

pub struct DictionaryUsersResource {
    area_name: String,
    ops: std::vec::Vec<Box<dyn Op>>,
}

impl DictionaryUsersResource {
    pub fn new(area_name: &str) -> DictionaryUsersResource {
        let mut resource = DictionaryUsersResource {
            area_name: String::from(area_name),
            ops: vec![],
        };
        resource.ops.push(Box::new(DictionaryUsersGetOp::new(resource.get_area_name(), resource.get_name())));
        resource.ops.push(Box::new(DictionaryUsersListOp::new(resource.get_area_name(), resource.get_name())));
        resource
    }
}

impl Resource for DictionaryUsersResource {
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

