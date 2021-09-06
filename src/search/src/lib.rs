use ic_cdk::{api::time, caller, export::Principal};
use ic_cdk_macros::query;

#[query]
fn greet(name: String) -> String {
    format!("Hello {}", name)
}

#[query]
fn greet_caller() -> String {
    format!("Hello {}", caller())
}

trait Environment {
    fn get_caller(&self) -> Principal;
    fn get_time(&self) -> u64;
}

struct CanisterEnvironment;

impl Environment for CanisterEnvironment {
    fn get_caller(&self) -> Principal {
        caller()
    }

    fn get_time(&self) -> u64 {
        time()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestEnvironment {
        caller: Principal,
        time: u64,
    }

    impl Environment for TestEnvironment {
        fn get_caller(&self) -> Principal {
            self.caller
        }

        fn get_time(&self) -> u64 {
            self.time
        }
    }
}
