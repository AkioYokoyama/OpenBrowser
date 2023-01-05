use sqlx::sqlite::{SqlitePool};
use webbrowser;

pub(crate) async fn lists(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
            SELECT id, name, url FROM sites ORDER BY id
        "#
    ).fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "{0: <02} | {1: <8} | {2}",
            rec.id,
            rec.name,
            rec.url,
        );
    }

    Ok(())
}

pub(crate) async fn find_by_name(pool: &SqlitePool, name: &str) -> anyhow::Result<()> {
    let rec = sqlx::query!(
        r#"
            SELECT url FROM sites WHERE name = ?
        "#,
        name
    ).fetch_one(pool)
    .await?;

    if webbrowser::open(&rec.url).is_err() {
        println!("'{}' is not url", rec.url);
    }

    Ok(())
}

pub(crate) async fn add(pool: &SqlitePool, name: &str, url: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        r#"
            INSERT INTO sites (name, url) VALUES(?, ?)
        "#,
        name,
        url
    ).execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub(crate) async fn delete(pool: &SqlitePool, name: &str) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;
    let _ = sqlx::query!(
        r#"
            DELETE FROM sites WHERE name = ?
        "#,
        name,
    ).execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(())
}
