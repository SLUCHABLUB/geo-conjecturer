use crate::Arguments;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct AppReference {
    pub arguments: &'static Arguments,
    pub players: Arc<Mutex<Players>>,
}

impl AppReference {
    pub fn from_arguments(arguments: &'static Arguments) -> AppReference {
        AppReference {
            arguments,
            players: Arc::new(Mutex::new(Players::default())),
        }
    }
}

#[derive(Default)]
pub(crate) struct Players {
    names: HashMap<Uuid, Box<str>>,
    ids: HashMap<Box<str>, Uuid>,
}

impl Players {
    pub(crate) fn sign_up(&mut self, name: &str) -> Option<Uuid> {
        if self.ids.contains_key(name) {
            return None;
        }

        let uuid = Uuid::new_v4();

        self.names.insert(uuid, Box::from(name));
        self.ids.insert(Box::from(name), uuid);

        Some(uuid)
    }
}
