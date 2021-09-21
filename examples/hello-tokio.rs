async fn async_func() {
    println!("in async func");
}

#[tokio::main]
async fn main() {
    let f = async {
        println!("hoge hoge");
    };
    let f2 = async_func();

    println!("hello, world");

    f.await;
    f2.await;
}
