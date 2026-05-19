use chrono::{Datelike, Local};
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::{prelude::FromRow, MySql, Pool};

#[derive(Serialize, Debug, FromRow)]
pub struct LeaderboardEntry {
    pub user_id: i32,
    pub username: String,
    pub best_score: i32,
    pub num_games: i32,
}

#[derive(FromRow, Debug)]
struct Count {
    total: Option<i32>,
}

pub async fn award_monthly_medals(
    pool: &Pool<MySql>,
    year: i32,
    month: i32,
) -> Result<(), sqlx::Error> {
    // If any medals already exist for this year/month, abort to avoid double-awarding.
    let existing: Count = sqlx::query_as(
        "SELECT COUNT(*) AS total FROM medals WHERE awarded_year = ? AND awarded_month = ?",
    )
    .bind(year)
    .bind(month)
    .fetch_one(pool)
    .await?;

    if existing.total.unwrap_or_default() > 0 {
        // Already awarded; nothing to do.
        return Ok(());
    }

    let start_time = format!("{year:04}-{month:02}-01 00:00:00");
    let end_time = if month == 12 {
        format!("{:04}-01-01 00:00:00", year + 1)
    } else {
        format!("{:04}-{:02}-01 00:00:00", year, month + 1)
    };

    let winners = get_leaderboard(pool, &start_time, &end_time).await?;

    let mut tx = pool.begin().await?;
    let medal_types = ["GOLD", "SILVER", "BRONZE"];
    for (index, medal_type) in medal_types.iter().enumerate() {
        if let Some(winner) = winners.get(index) {
            sqlx::query(
                "INSERT INTO medals (awarded_year, awarded_month, type, user_id) VALUES (?, ?, ?, ?)",
            )
            .bind(year)
            .bind(month)
            .bind(*medal_type)
            .bind(winner.user_id)
            .execute(&mut tx)
            .await?;
        }
    }
    tx.commit().await?;

    Ok(())
}

/// Gets leaderboard entries for the given time period.
pub async fn get_leaderboard(
    pool: &Pool<MySql>,
    start_time: &str,
    end_time: &str,
) -> Result<Vec<LeaderboardEntry>, sqlx::Error> {
    let entries: Vec<LeaderboardEntry> = sqlx::query_as(
        "SELECT games.user_id, users.username as username, MAX(terminal_score) AS best_score, COUNT(*) AS num_games
         FROM games
         JOIN users ON games.user_id = users.user_id
         WHERE games.user_id IS NOT NULL
           AND games.has_terminated = TRUE
           AND games.terminal_score IS NOT NULL
           AND games.start_time >= ?
           AND games.start_time < ?
         GROUP BY games.user_id
         ORDER BY best_score DESC, games.user_id ASC",
    )
    .bind(start_time)
    .bind(end_time)
    .fetch_all(pool)
    .await?;

    Ok(entries)
}

/// API endpoint to get the current month's leaderboard
#[get("/leaderboard?<year>&<month>")]
pub async fn get_monthly_leaderboard(
    pool: &State<Pool<MySql>>,
    year: Option<i32>,
    month: Option<i32>,
) -> Json<Vec<LeaderboardEntry>> {
    let now = Local::now();
    let year = year.unwrap_or(now.year());
    let month = month.unwrap_or(now.month() as i32);

    let start_time = format!("{year:04}-{month:02}-01 00:00:00");
    let end_time = if month == 12 {
        format!("{:04}-01-01 00:00:00", year + 1)
    } else {
        format!("{:04}-{:02}-01 00:00:00", year, month + 1)
    };

    match get_leaderboard(pool.inner(), &start_time, &end_time).await {
        Ok(entries) => Json(entries),
        Err(_) => Json(Vec::new()),
    }
}
