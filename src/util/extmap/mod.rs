#![allow(clippy::module_name_repetitions)]

mod lock_map;
mod map;

pub use lock_map::LockingExtensionMap;
pub use map::ExtensionMap;

mod test_data {
    #[derive(Default)]
    pub struct FakeConfig {
        pub some_flag: bool,
        pub some_option: String,
    }

    #[derive(Default)]
    pub struct FakeDatabase {
        pub host: String,
        pub port: u16,
        pub user: String,
        pub pass: String,
    }

    #[derive(Default)]
    pub struct FakeServiceState {
        pub counter: usize,
    }

    #[derive(Default)]
    pub struct FakeService {
        pub state: FakeServiceState,
    }

    impl FakeService {
        // NOTE: This should not be necessary, these are clearly used as shown by the "x references" shown in VS Code.
        #[allow(dead_code)]
        pub fn new() -> Self {
            Self::default()
        }

        // NOTE: This should not be necessary, these are clearly used as shown by the "x references" shown in VS Code.
        #[allow(dead_code)]
        pub fn get(&mut self) -> usize {
            self.state.counter += 1;
            self.state.counter
        }
    }
}
