use ic_cdk::caller;
use ic_cdk_macros::query;

#[query]
fn greet(name: String) -> String {
    format!("Hello {}", name)
}

#[query]
fn greet_caller() -> String {
    format!("Hello {}", caller())
}
