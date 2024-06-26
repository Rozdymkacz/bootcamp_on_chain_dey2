use std::cell::RefCell;

thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, last_name: String, num1: i8 ) -> String {
    format!("Hello, {} {} {}!", name, last_name, num1, )
}

#[ic_cdk::update]
fn dodaj_wpis (wpis: String) {
    WPISY.with(|wpisy| {
        wpisy.borrow_mut().push(wpis)
    });
}

#[ic_cdk::query]
fn oddaj_wpisy () -> Vec<String>{
    WPISY.with(|wpisy| {
        wpisy.borrow().clone()
    })
}

#[ic_cdk::update]
fn wyczyść_wpisy (){
    WPISY.with(|wpisy| {
        *wpisy.borrow_mut() = Vec::new()
    });
}