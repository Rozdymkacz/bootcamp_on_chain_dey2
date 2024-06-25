#[ic_cdk::query]
fn greet(name: String, last_name: String, num1: i8 ) -> String {
    format!("Hello, {} {} {}!", name, last_name, num1, )
}
