use std::collections::HashSet;

use sqlx::{prelude::FromRow, MySql, Pool};

const LEADERBOARD_CANDIDATE_QUERY: &str = r#"
    SELECT g.user_id, g.uuid, g.terminal_score AS best_score
    FROM games g
    INNER JOIN (
        SELECT user_id, MAX(terminal_score) AS max_score
        FROM games
        WHERE user_id IS NOT NULL
        AND has_terminated = TRUE
        AND terminal_score IS NOT NULL
        AND start_time >= ?
        AND start_time < ?
        GROUP BY user_id
    ) best ON g.user_id = best.user_id AND g.terminal_score = best.max_score
    ORDER BY g.terminal_score DESC, g.user_id ASC, g.uuid ASC
"#;

#[allow(dead_code)]
#[derive(FromRow, Debug)]
struct MonthlyMedalCandidate {
    user_id: i32,
    uuid: String,
    best_score: i32,
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

/// Gets leaderboard candidates for the given time period, ordered by score desc then user_id asc, and returns the top candidate for each user_id.
/// The returned Vec will contain at most 1 entry for each user.
async fn get_leaderboard(
    pool: &Pool<MySql>,
    start_time: &str,
    end_time: &str,
) -> Result<Vec<MonthlyMedalCandidate>, sqlx::Error> {
    let candidates: Vec<MonthlyMedalCandidate> = sqlx::query_as(LEADERBOARD_CANDIDATE_QUERY)
        .bind(&start_time)
        .bind(&end_time)
        .fetch_all(pool)
        .await?;

    let mut seen_user_ids = HashSet::new();
    let mut winners = Vec::new();
    for candidate in candidates.into_iter() {
        if seen_user_ids.insert(candidate.user_id) {
            winners.push(candidate);
        }
    }

    Ok(winners)
}
