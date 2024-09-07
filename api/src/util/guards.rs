use actix_web::guard::GuardContext;

pub const KEY_HEADER: &str = "Kromer-Admin";

pub fn admin_key_guard(ctx: &GuardContext) -> bool {
    let admin_key = std::env::var("KROMER_ADMIN_KEY").expect("No admin key provided");
    ctx.head()
        .headers()
        .get(KEY_HEADER)
        .map_or(false, |it| it.as_bytes() == admin_key.as_bytes())
}
