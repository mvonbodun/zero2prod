use zero2prod::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
