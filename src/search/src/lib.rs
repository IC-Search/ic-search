use ic_cdk::print;
use ic_cdk_macros::query;

#[query]
fn greet(name: String) -> String {
    print("Hello World from DFINITY!");
    format!("Hello {}", name)
}
