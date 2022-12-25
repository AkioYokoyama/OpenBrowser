use std::env;
use sqlx::sqlite::{SqlitePool};
use webbrowser;
use structopt::StructOpt;

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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::List) => {
            lists(&pool).await?;
        }
        Some(Command::Add { name, url}) => {
            let _ = add(&pool, &name, url).await;
            println!("「{}」 is added!", name);
        }
        Some(Command::Delete { name }) => {
            let _ = delete(&pool, &name).await;
            println!("「{}」 is deleted!", name);
        }
        Some(Command::Brow { names } ) => {
            let args_iter = names[0..].iter();
            for name in args_iter {
                let _ = find_by_name(&pool, name).await;
            }
        }
        None => {
            let args: Vec<String> = env::args().collect();
            let args_iter = args[1..].iter();
            for name in args_iter {
                let _ = find_by_name(&pool, name).await;
            }
        }
    }

    Ok(())
}

async fn lists(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, name, url
FROM sites
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "{} | {} | {}",
            rec.id,
            rec.name,
            rec.url,
        );
    }

    Ok(())
}

async fn find_by_name(pool: &SqlitePool, name: &str) -> anyhow::Result<()> {
    let rec = sqlx::query!(
        r#"
SELECT url FROM sites WHERE name = ?
        "#,
        name
    )
    .fetch_one(pool)
    .await?;

    if webbrowser::open(&rec.url).is_err() {
        println!("'{}' is not url", rec.url);
    }

    Ok(())
}

async fn add(pool: &SqlitePool, name: &str, url: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        r#"
INSERT INTO sites (name, url) VALUES(?, ?)
        "#,
        name,
        url
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn delete(pool: &SqlitePool, name: &str) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;
    let _ = sqlx::query!(
        r#"
DELETE FROM sites WHERE name = ?
        "#,
        name,
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(())
}
