use std::future::Future;

pub trait Handler: Clone + 'static {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, req: String) -> Self::Future;

    fn fuck(&self);
}

impl<F, Fut> Handler for F
where
    F: Fn(String) -> Fut + Clone + 'static,
    Fut: Future,
{
    type Output = Fut::Output;

    type Future = Fut;

    fn call(&self, req: String) -> Self::Future {
        (self)(req)
    }

    fn fuck(&self) {
        println!("FUCK！！！！");
    }
}

struct Service<F>
where
    F: Handler,
{
    f: F,
}

impl<F> Service<F>
where
    F: Handler,
{
    pub fn new(f: F) -> Self {
        Self { f }
    }

    pub async fn call(&self) {
        self.f.fuck();
        self.f.call("request".to_string()).await;
    }
}

async fn handle(req: String) -> u32 {
    println!("{}", req);
    123123
}

#[tokio::main]
async fn main() {
    handle.fuck();
    Service::new(handle).call().await;
}
