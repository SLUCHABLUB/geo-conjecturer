use crate::Arguments;
use crate::host::PASSWORD_COOKIE_NAME;
use axum_extra::extract::CookieJar;
use std::collections::HashMap;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct AppReference {
    pub map_defaults: MapDefaults,
    pub password_store: PasswordStore,
    pub players: Arc<Mutex<Players>>,
}

impl From<&Arguments> for AppReference {
    fn from(arguments: &Arguments) -> AppReference {
        AppReference {
            map_defaults: MapDefaults::from(arguments),
            password_store: PasswordStore::from(arguments),
            players: Arc::<Mutex<Players>>::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct MapDefaults {
    pub latitude: f32,
    pub longitude: f32,
    pub zoom: u8,
}

impl From<&Arguments> for MapDefaults {
    fn from(arguments: &Arguments) -> Self {
        MapDefaults {
            latitude: arguments.latitude,
            longitude: arguments.longitude,
            zoom: arguments.zoom,
        }
    }
}

#[derive(Clone)]
pub(crate) struct PasswordStore {
    hash: u64,
}

impl PasswordStore {
    pub(crate) fn is_correct(&self, password: &str) -> bool {
        let mut hasher = DefaultHasher::new();

        password.hash(&mut hasher);

        let hash = hasher.finish();

        hash == self.hash
    }

    pub(crate) fn is_authorised(&self, cookies: &CookieJar) -> bool {
        cookies
            .get(PASSWORD_COOKIE_NAME)
            .is_some_and(|cookie| self.is_correct(cookie.value()))
    }
}

impl From<&Arguments> for PasswordStore {
    fn from(arguments: &Arguments) -> Self {
        PasswordStore {
            hash: arguments.host_password_hash,
        }
    }
}

pub(crate) struct PlayerReference<'store> {
    pub name: &'store str,
    pub id: Uuid,
}

#[derive(Default)]
pub(crate) struct Players {
    names: HashMap<Uuid, Box<str>>,
    ids: HashMap<Box<str>, Uuid>,
}

impl Players {
    pub(crate) fn iter(&self) -> impl Iterator<Item = PlayerReference<'_>> {
        // TODO: Sort?
        self.ids
            .iter()
            .map(|(name, id)| PlayerReference { name, id: *id })
    }

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
