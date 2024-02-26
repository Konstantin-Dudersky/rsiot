use futures::future::{BoxFuture, Future};

struct Config {
    func: Box<dyn Fn() -> BoxFuture<'static, u32>>,
}

impl Config {
    fn new<F>(f: fn() -> F) -> Config
    where
        F: Future<Output = u32> + Send + 'static,
    {
        Config {
            func: Box::new(move || Box::pin(f())),
        }
    }

    async fn run(&self) {
        (self.func)().await;
    }
}

async fn test_func() -> u32 {
    return 42;
}
async fn test_func2() -> u32 {
    return 33;
}

#[tokio::main]
async fn main() {
    let mut v: Vec<Config> = Vec::new();
    v.push(Config::new(test_func));
    v.push(Config::new(test_func2));
}
