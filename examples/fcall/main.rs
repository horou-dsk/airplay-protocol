use std::future::Future;

// use httparse::Request;

// type ServiceFn = for<'a> fn(&'a str) -> BoxFuture<'a, u32>;

#[derive(Debug)]
struct TmpReq<'a> {
    _uri: &'a str,
}

// struct Service {
//     f: BoxServiceFactory,
// }

// impl Service {
//     pub fn new(f: BoxServiceFactory) -> Self {
//         Self { f }
//     }

//     pub async fn call(&self) {
//         let req = TmpReq { uri: "123" };
//         let result = self.f.call(req).await;
//         println!("result = {result}");
//     }
// }

trait AsyncFn<T>: (Fn(T) -> <Self as AsyncFn<T>>::Fut) {
    type Fut: Future<Output = <Self as AsyncFn<T>>::Output>;
    type Output;
}

impl<T, F, Fut> AsyncFn<T> for F
where
    F: Fn(T) -> Fut,
    Fut: Future,
{
    type Fut = Fut;
    type Output = Fut::Output;
}

async fn handle(req: &str) -> u32 {
    println!("{:?}", req);
    123123
}

async fn make_service<F>(f: F)
where
    F: for<'a> AsyncFn<&'a str, Output = u32>,
{
    println!("result = {}", f("req").await);
}

#[tokio::main]
async fn main() {
    make_service(handle).await;
}
