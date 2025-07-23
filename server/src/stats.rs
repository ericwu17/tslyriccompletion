use std::sync::{Arc, Mutex};

use chrono::{NaiveDateTime, TimeZone, Utc};
use rocket::State;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool};

#[derive(Debug, Clone, Serialize)]
pub struct StatsResponse {
    all_time: StatsData,
    last_365_days: StatsData,
    last_30_days: StatsData,
    last_7_days: StatsData,
    stats_generation_time: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatsData {
    num_games: i32,
    num_guesses: i32,
    multiple_choice_guesses: i32,
    free_response_guesses: i32,
    skipped: i32,
    num_lifelines_earned: i32,
    num_lifelines_used: i32,
}

#[derive(FromRow, Debug)]
struct Count {
    total: i32,
}

/// API endpoint for getting statistics from the database.
/// returns a StatsResponse object
#[get("/stats")]
pub async fn get_stats(
    pool: &State<Pool<MySql>>,
    cache: &State<Arc<Mutex<Option<StatsResponse>>>>,
) -> String {
    let date_format_string = "%Y-%m-%d %H:%M"; // used for stats_generation_time

    {
        // READ FROM CACHE
        let cache = cache.lock().unwrap();
        if cache.is_some() {
            let inner = cache.clone().unwrap();

            let cache_computed_time =
                NaiveDateTime::parse_from_str(&inner.stats_generation_time, date_format_string)
                    .expect("cache should hold valid datetime string");
            let cache_computed_time = Utc.from_utc_datetime(&cache_computed_time);

            let now = Utc::now();

            let duration = if cache_computed_time > now {
                cache_computed_time - now
            } else {
                now - cache_computed_time
            };

            if duration.num_seconds().abs() < 3600 {
                return serde_json::to_string(&inner).unwrap();
            }
        }
    }

    let all_time = get_stats_from_recent_period(None, pool).await;
    let last_365_days = get_stats_from_recent_period(Some(365), pool).await;
    let last_30_days = get_stats_from_recent_period(Some(30), pool).await;
    let last_7_days = get_stats_from_recent_period(Some(7), pool).await;

    let time_now_utc = Utc::now();
    let stats_generation_time = format!("{}", time_now_utc.format(date_format_string));

    let response = StatsResponse {
        all_time,
        last_365_days,
        last_30_days,
        last_7_days,
        stats_generation_time,
    };

    {
        // SAVE TO CACHE
        let mut cache = cache.lock().unwrap();
        *cache = Some(response.clone());
    }

    return serde_json::to_string(&response).unwrap();
}

async fn get_stats_from_recent_period(
    period_in_days: Option<i32>,
    pool: &rocket::State<Pool<MySql>>,
) -> StatsData {
    let num_games: Count = match period_in_days {
        Some(n) => sqlx::query_as(
            "SELECT COUNT(*) AS total FROM games WHERE start_time > NOW() - INTERVAL ? DAY",
        )
        .bind(n)
        .fetch_one(pool.inner())
        .await
        .unwrap(),
        None => sqlx::query_as("SELECT COUNT(*) AS total FROM games")
            .fetch_one(pool.inner())
            .await
            .unwrap(),
    };
    let num_games = num_games.total;

    let num_guesses: Count = match period_in_days {
        Some(n) => sqlx::query_as(
            "SELECT COUNT(*) AS total FROM guesses WHERE submit_time > NOW() - INTERVAL ? DAY",
        )
        .bind(n)
        .fetch_one(pool.inner())
        .await
        .unwrap(),
        None => sqlx::query_as("SELECT COUNT(*) AS total FROM guesses")
            .fetch_one(pool.inner())
            .await
            .unwrap(),
    };
    let num_guesses = num_guesses.total;

    let multiple_choice_guesses: Count = match period_in_days {
        Some(n) => sqlx::query_as(
            "SELECT COUNT(*) AS total FROM guesses WHERE JSON_UNQUOTE(options) != \"[]\" AND submit_time > NOW() - INTERVAL ? DAY",
        )
        .bind(n)
        .fetch_one(pool.inner())
        .await
        .unwrap(),
        None => sqlx::query_as("SELECT COUNT(*) AS total FROM guesses WHERE JSON_UNQUOTE(options) != \"[]\"")
            .fetch_one(pool.inner())
            .await
            .unwrap(),
    };
    let multiple_choice_guesses = multiple_choice_guesses.total;

    let free_response_guesses: Count = match period_in_days {
        Some(n) => sqlx::query_as(
            "SELECT COUNT(*) AS total FROM guesses WHERE JSON_UNQUOTE(options) = \"[]\" AND submit_time > NOW() - INTERVAL ? DAY",
        )
        .bind(n)
        .fetch_one(pool.inner())
        .await
        .unwrap(),
        None => sqlx::query_as("SELECT COUNT(*) AS total FROM guesses WHERE JSON_UNQUOTE(options) = \"[]\"")
            .fetch_one(pool.inner())
            .await
            .unwrap(),
    };
    let free_response_guesses = free_response_guesses.total;

    let skipped : Count = match period_in_days {
        Some(n) => sqlx::query_as(
            "SELECT COUNT(*) AS total FROM guesses WHERE result = \"skipped\" AND submit_time > NOW() - INTERVAL ? DAY",
        )
        .bind(n)
        .fetch_one(pool.inner())
        .await
        .unwrap(),
        None => sqlx::query_as("SELECT COUNT(*) AS total FROM guesses WHERE result = \"skipped\"")
            .fetch_one(pool.inner())
            .await
            .unwrap(),
    };
    let skipped = skipped.total;

    let num_lifelines_earned : Count = match period_in_days {
        Some(n) => sqlx::query_as(
            "SELECT COUNT(*) AS total FROM guesses WHERE lifeline_earned IS NOT NULL AND submit_time > NOW() - INTERVAL ? DAY",
        )
        .bind(n)
        .fetch_one(pool.inner())
        .await
        .unwrap(),
        None => sqlx::query_as("SELECT COUNT(*) AS total FROM guesses WHERE lifeline_earned IS NOT NULL")
            .fetch_one(pool.inner())
            .await
            .unwrap(),
    };
    let num_lifelines_earned = num_lifelines_earned.total;

    let num_lifelines_used : Count = match period_in_days {
        Some(n) => sqlx::query_as(
            "SELECT CAST(SUM(JSON_LENGTH(lifelines_used)) AS SIGNED) as total FROM guesses WHERE submit_time > NOW() - INTERVAL ? DAY",
        )
        .bind(n)
        .fetch_one(pool.inner())
        .await
        .unwrap(),
        None => sqlx::query_as("SELECT CAST(SUM(JSON_LENGTH(lifelines_used)) AS SIGNED) as total FROM guesses")
            .fetch_one(pool.inner())
            .await
            .unwrap(),
    };
    let num_lifelines_used = num_lifelines_used.total;

    StatsData {
        num_games,
        num_guesses,
        multiple_choice_guesses,
        free_response_guesses,
        skipped,
        num_lifelines_earned,
        num_lifelines_used,
    }
}
