use dotenvy::dotenv;
use std::env;
use sqlx::sqlite::{SqlitePool};
use structopt::StructOpt;

pub(crate) mod database;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Add { name: String, url: String },
    Delete { name: String },
    List,
    Brow { names: Vec<String> },
    Freq,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::List) => {
            database::lists(&pool).await?;
        }
        Some(Command::Add { name, url}) => {
            database::add(&pool, &name, url).await?;
            println!("「{}」 is added!", name);
        }
        Some(Command::Delete { name }) => {
            database::delete(&pool, &name).await?;
            println!("「{}」 is deleted!", name);
        }
        Some(Command::Brow { names } ) => {
            let args_iter = names[0..].iter();
            for name in args_iter {
                database::find_by_name(&pool, name).await?;
                database::increase(&pool, name).await?;
            }
        }
        Some(Command::Freq) => {
            database::frequency(&pool).await?;
        }
        None => println!("Set arguments."),
    }

    Ok(())
}
