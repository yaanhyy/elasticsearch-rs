
fn main() {
    async_std::task::block_on(async move {
        println!("res:{:?}", "ok");
    });
}