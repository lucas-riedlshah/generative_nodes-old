use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use druid::{Data, Lens};

use crate::core::{App, Direction};

pub struct CacheLens<T> {
    phantom: PhantomData<T>,
    node_index: usize,
    port_index: usize,
}

impl<T> CacheLens<T> {
    pub fn new(
        node_index: usize,
        port_index: usize,
    ) -> CacheLens<T> {
        CacheLens {
            phantom: PhantomData::<T>,
            node_index,
            port_index
        }
    }
}

impl<T: Data + 'static> Lens<Rc<RefCell<App>>, T> for CacheLens<T> {
    fn with<R, F: FnOnce(&T) -> R>(&self, data: &Rc<RefCell<App>>, f: F) -> R {
        let app = data.borrow();
        let cache_index = app
            .get_node(self.node_index)
            .get_ports()
            .get(self.port_index)
            .unwrap()
            .get_cache_index();
        let value = app.get_cache().get::<T>(cache_index).unwrap();
        f(value)
    }

    fn with_mut<R, F: FnOnce(&mut T) -> R>(&self, data: &mut Rc<RefCell<App>>, f: F) -> R {
        let app = data.borrow();
        let port = app
            .get_node(self.node_index)
            .get_ports()
            .get(self.port_index)
            .unwrap();
        // TODO: Investigate whether or not implementing PartialEq on Direction is more efficient than matches!() 
        let can_mutate = !port.is_connected() || matches!(port.get_direction(), Direction::Output);
        let cache_index = port
            .get_cache_index()
            .clone();
        let value = app.get_cache().get::<T>(&cache_index).unwrap();
        let mut new_value = value.clone();
        let result = f(&mut new_value);
        let changed = !new_value.same(value);

        drop(app);

        if can_mutate && changed {
            let mut app_mut = data.borrow_mut();
            app_mut.get_cache_mut().set(&cache_index, new_value);
        }

        result
    }
}
