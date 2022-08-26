use std::cell::RefCell;
use wit_bindgen_guest_rust::Handle;

wit_bindgen_guest_rust::import!("./wit/delegate.wit");

use crate::delegate::Delegate;

wit_bindgen_guest_rust::export!("./wit/service.wit");

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Service;
impl service::Service for Service {}

#[derive(Default)]
pub struct Manager {
    delegates: RefCell<Vec<String>>,
}

impl service::Manager for Manager {
    fn create() -> Handle<Manager> {
        Handle::new(Manager::default())
    }

    fn add_delegate(&self, obj: Handle<Delegate>) {
        self.delegates.borrow_mut().push(obj.name())
    }

    fn delegate_names(&self) -> Vec<String> {
        self.delegates.borrow().clone()
    }
}
