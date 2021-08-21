use ic_cdk_macros::query;

#[query]
fn print() {
    ic_cdk::print("Hello World from DFINITY!");
}
