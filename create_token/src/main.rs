use aws_lc_rs::rand::fill;
use clap::Parser;
use ides::{
    auth::{Role, Token},
    bytes::Bytes,
    db,
    prelude::*,
};

#[derive(Parser)]
#[command(name = "create-admin")]
#[command(
    about = "Add an admin user. Connects to a database based on $DATABASE_URL"
)]
struct Args {
    #[arg(long)]
    name: String,
    #[arg(long)]
    role: String,
}

#[tokio::main]
async fn main() -> std::result::Result<(), ()> {
    let result: Result<()> = async {
        let args = Args::parse();
        let db = db::create_pg_pool().await?;

        let mut buffer = [0u8; 66];
        fill(&mut buffer).map_err(|e| {
            ErrStack::new(ErrT::Invariant)
                .ctx(format!("aws says no random bytes for you: {e}"))
        })?;
        let token = Token::new(buffer.to_base64());
        let digest = token.sha256_hex();

        let role: Role =
            args.role.clone().try_into().map_err(|e: ErrStack| {
                e.wrap(ErrT::ValidationError)
                    .ctx(format!("{} is not a valid role", args.role))
            })?;
        let role_id: i32 = role.into();
        query!(
            "insert into token
        (
            token_digest,
            name,
            role_id
        ) values ($1, $2, $3)",
            digest,
            args.name,
            role_id
        )
        .execute(&db)
        .await
        .map_err(|e| ErrStack::new(ErrT::SqlxError).ctx(e.to_string()))?;

        println!(
            "user {} created. Token is '{}'",
            args.name,
            token.display_secret_value()
        );

        Ok(())
    }
    .await;
    if let Err(ref e) = result {
        println!("{e}");
    };
    result.map_err(|_| {})
}
