use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

/// ## Extension Map
/// Allows for the creation of a generic map of types to serve as an "extension map" or
/// even a "service provider" data type. Can hold app-wide data or services that are
/// associated with a specific "App", such as config information, database connections, etc.
///
/// ### Example(s)
/// ```
/// use dungen::util::ExtensionMap;
///
/// #[derive(Default)]
/// struct FakeConfig {
///     some_flag: bool,
///     some_option: String,
/// }
///
/// #[derive(Default)]
/// struct FakeDatabase {
///     host: String,
///     port: u16,
///     user: String,
///     pass: String,
/// }
///
/// #[derive(Default)]
/// struct FakeService {
///     value1: usize,
///     value2: f64,
/// }
///
/// struct App {
///     extensions: ExtensionMap,
/// }
///
/// impl App {
///     fn new() -> Self {
///         Self {
///             extensions: ExtensionMap::default(),
///         }
///     }
/// }
///
/// // Create default / empty app.
/// let app = App::new();
///
/// // Add extension types to app.
/// app.extensions.insert(FakeConfig {
///     some_flag: true,
///     some_option: "no_std".to_string(),
/// });
/// app.extensions.insert(FakeDatabase {
///     host: "localhost".to_string(),
///     port: 5432,
///     user: "admin".to_string(),
///     pass: "8008135".to_string(),
/// });
///
/// // Get reference to extension types in map.
/// {
///     // Retrievals are scoped because multiple borrows in the same function
///     // in the same thread will cause a deadlock.
///     let config = app.extensions.get::<FakeConfig>();
///     let database = app.extensions.get::<FakeDatabase>();
///
///     assert!(config.some_flag);
///     assert_eq!(config.some_option, "no_std");
///     assert_eq!(database.host, "localhost");
///     assert_eq!(database.port, 5432);
///     assert_eq!(database.user, "admin");
///     assert_eq!(database.pass, "8008135");
/// }
///
/// // Get mutable reference to extension types in map.
/// {
///     app.extensions.get_mut::<FakeConfig>().some_flag = false;
///     app.extensions.get_mut::<FakeDatabase>().host = "198.111.3.1".to_string();
///     assert!(!app.extensions.get::<FakeConfig>().some_flag);
///     assert_eq!(app.extensions.get::<FakeDatabase>().host, "198.111.3.1".to_string());
///     assert_eq!(app.extensions.get::<FakeDatabase>().port, 5432);
/// }
///
/// // Types not already in the extension map will have a default instance
/// // created and added to the map if accessed.
/// {
///     let service = app.extensions.get::<FakeService>();
///     assert_eq!(service.value1, usize::default());
///     assert_eq!(service.value2, f64::default());
/// }
/// ```
///
/// ## Source
/// This type and associated code were shamelessly ***"borrowed"*** from [**THIS AWESOME BLOG POST**](`https://lucumr.pocoo.org/2022/1/6/rust-extension-map/`).
#[derive(Default)]
pub struct ExtensionMap {
    map: RefCell<HashMap<TypeId, Box<dyn Any>>>,
}

impl ExtensionMap {
    /// Inserts a new type into the extension map.
    pub fn insert<T: 'static>(&self, value: T) {
        self.map
            .borrow_mut()
            .insert(TypeId::of::<T>(), Box::new(value));
    }

    /// Gets a non-mutable reference to a type in the extension map.
    ///
    /// ## Panics
    /// - Does not panic.
    pub fn get<T: Default + 'static>(&self) -> Ref<'_, T> {
        self.ensure::<T>();
        Ref::map(self.map.borrow(), |m| {
            m.get(&TypeId::of::<T>())
                .and_then(|b| b.downcast_ref())
                .unwrap()
        })
    }

    /// Gets a mutable reference to a type in the extension map.
    ///
    /// ## Panics
    /// - ~If the type is already mutably borrowed.~
    /// - I think this function actually just deadlocks, not panics, and only from consecutive mutable
    /// borrows within the same function on the same thread.
    pub fn get_mut<T: Default + 'static>(&self) -> RefMut<'_, T> {
        self.ensure::<T>();
        RefMut::map(self.map.borrow_mut(), |m| {
            m.get_mut(&TypeId::of::<T>())
                .and_then(|b| b.downcast_mut())
                .unwrap()
        })
    }

    /// Tests whether the extension map contains the given type.
    pub fn contains<T: 'static>(&self) -> bool {
        self.map.borrow().contains_key(&TypeId::of::<T>())
    }

    /// Ensures that a [`Default`] instance of a type is in the extension map even if one
    /// has not been manually placed.
    fn ensure<T: Default + 'static>(&self) {
        if self.map.borrow().get(&TypeId::of::<T>()).is_none() {
            self.insert(T::default());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_data::{FakeConfig, FakeDatabase, FakeService};
    use super::*;

    struct App {
        extensions: ExtensionMap,
    }

    impl App {
        fn new() -> Self {
            Self {
                extensions: ExtensionMap::default(),
            }
        }
    }

    #[test]
    fn basics() {
        let app = App::new();

        app.extensions.insert(FakeConfig {
            some_flag: true,
            some_option: "no_std".to_string(),
        });
        app.extensions.insert(FakeDatabase {
            host: "localhost".to_string(),
            port: 5432,
            user: "admin".to_string(),
            pass: "8008135".to_string(),
        });

        let config = app.extensions.get::<FakeConfig>();
        let database = app.extensions.get::<FakeDatabase>();

        assert!(config.some_flag);
        assert_eq!(config.some_option, "no_std");
        assert_eq!(database.host, "localhost");
        assert_eq!(database.port, 5432);
        assert_eq!(database.user, "admin");
        assert_eq!(database.pass, "8008135");
    }

    #[test]
    fn mutability() {
        let app = App::new();

        app.extensions.insert(FakeConfig {
            some_flag: true,
            some_option: "no_std".to_string(),
        });

        assert!(app.extensions.get::<FakeConfig>().some_flag);

        app.extensions.get_mut::<FakeConfig>().some_flag = false;
        assert!(!app.extensions.get::<FakeConfig>().some_flag);
    }

    #[test]
    fn contains_or_not() {
        let app = App::new();

        assert!(!app.extensions.contains::<FakeConfig>());

        app.extensions.insert(FakeConfig {
            some_flag: true,
            some_option: "no_std".to_string(),
        });

        assert!(app.extensions.contains::<FakeConfig>());

        assert!(!app.extensions.contains::<FakeDatabase>());
        assert_eq!(app.extensions.get::<FakeDatabase>().host, String::default());
    }

    #[test]
    fn works_as_service_provider() {
        #[derive(Default)]
        struct Service1(FakeService);
        #[derive(Default)]
        struct Service2(FakeService);

        let app = App::new();

        let mut service1 = Service1(FakeService::new());
        service1.0.get();
        service1.0.get();
        service1.0.get();
        service1.0.get();
        app.extensions.insert(service1);

        let mut service2 = Service2::default();
        service2.0.get();
        app.extensions.insert(service2);

        assert!(app.extensions.contains::<Service1>());
        assert!(app.extensions.contains::<Service2>());

        assert_eq!(app.extensions.get_mut::<Service1>().0.get(), 5);
        assert_eq!(app.extensions.get_mut::<Service1>().0.get(), 6);
        assert_eq!(app.extensions.get_mut::<Service2>().0.get(), 2);
        assert_eq!(app.extensions.get_mut::<Service2>().0.get(), 3);
    }
}
