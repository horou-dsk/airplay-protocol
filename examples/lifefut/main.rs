trait MyFuture {
    fn poll(&self) -> u32;
}

struct GeneratedAnonymousFuture<'a> {
    req: &'a str,
}

impl<'a> MyFuture for GeneratedAnonymousFuture<'a> {
    fn poll(&self) -> u32 {
        println!("{:?}", self.req);
        123123
    }
}

fn handle<'a>(req: &'a str) -> GeneratedAnonymousFuture<'a> {
    GeneratedAnonymousFuture { req }
}

fn make_service<F, Fut>(f: F)
where
    F: for<'a> Fn(&'a str) -> Fut + Clone + 'static,
    Fut: MyFuture,
{
    println!("result = {}", f("req").poll());
}

fn main() {
    let x = 1;
    // make_service(handle); error
}
