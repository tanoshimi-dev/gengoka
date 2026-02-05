use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::config::Config;
use crate::models::Category;
use crate::services::gemini::{build_challenge_prompt, GeminiClient};

pub async fn generate_challenges(pool: &PgPool, config: &Config, daily_count: u32) -> Result<u32> {
    let categories = fetch_active_categories(pool).await?;

    if categories.is_empty() {
        warn!("No active categories found, skipping challenge generation");
        return Ok(0);
    }

    let gemini_client = GeminiClient::new(config.gemini.clone())?;
    let challenges_per_category = calculate_challenges_per_category(daily_count, categories.len());

    let mut total_generated = 0u32;

    for (index, category) in categories.iter().enumerate() {
        let count = challenges_per_category[index];
        if count == 0 {
            continue;
        }

        info!(
            "Generating {} challenges for category: {}",
            count, category.name
        );

        match generate_challenges_for_category(&gemini_client, pool, category, count).await {
            Ok(generated) => {
                total_generated += generated;
                info!(
                    "Generated {} challenges for category: {}",
                    generated, category.name
                );
            }
            Err(e) => {
                error!(
                    "Failed to generate challenges for category {}: {}",
                    category.name, e
                );
            }
        }
    }

    Ok(total_generated)
}

async fn fetch_active_categories(pool: &PgPool) -> Result<Vec<Category>> {
    let categories = sqlx::query_as::<_, Category>(
        r#"
        SELECT id, name, description, icon, color, char_limit, sort_order, status, created_at, updated_at
        FROM categories
        WHERE status = 'active'
        ORDER BY sort_order ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch active categories")?;

    Ok(categories)
}

async fn generate_challenges_for_category(
    client: &GeminiClient,
    pool: &PgPool,
    category: &Category,
    count: u32,
) -> Result<u32> {
    let prompt = build_challenge_prompt(category, count);
    debug!("Generated prompt for category {}: {}", category.name, prompt);

    let response = client.generate_challenges(prompt).await?;
    let mut inserted_count = 0u32;

    for challenge in response.challenges {
        match insert_challenge(pool, category.id, &challenge.title, &challenge.description, challenge.char_limit).await {
            Ok(_) => {
                inserted_count += 1;
                debug!("Inserted challenge: {}", challenge.title);
            }
            Err(e) => {
                error!("Failed to insert challenge '{}': {}", challenge.title, e);
            }
        }
    }

    Ok(inserted_count)
}

async fn insert_challenge(
    pool: &PgPool,
    category_id: Uuid,
    title: &str,
    description: &str,
    char_limit: i32,
) -> Result<Uuid> {
    let now = Utc::now();
    let today = now.date_naive();

    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO challenges (category_id, title, description, char_limit, release_date, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 'active', $6, $6)
        RETURNING id
        "#,
    )
    .bind(category_id)
    .bind(title)
    .bind(description)
    .bind(char_limit)
    .bind(today)
    .bind(now)
    .fetch_one(pool)
    .await
    .context("Failed to insert challenge into database")?;

    Ok(id)
}

fn calculate_challenges_per_category(total: u32, category_count: usize) -> Vec<u32> {
    if category_count == 0 {
        return vec![];
    }

    let base_count = total / category_count as u32;
    let remainder = total % category_count as u32;

    let mut distribution = vec![base_count; category_count];

    // Distribute remainder across first categories
    for i in 0..(remainder as usize) {
        distribution[i] += 1;
    }

    distribution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_challenges_per_category() {
        // 5 challenges across 2 categories = [3, 2]
        let result = calculate_challenges_per_category(5, 2);
        assert_eq!(result, vec![3, 2]);

        // 10 challenges across 3 categories = [4, 3, 3]
        let result = calculate_challenges_per_category(10, 3);
        assert_eq!(result, vec![4, 3, 3]);

        // 3 challenges across 3 categories = [1, 1, 1]
        let result = calculate_challenges_per_category(3, 3);
        assert_eq!(result, vec![1, 1, 1]);

        // Empty categories
        let result = calculate_challenges_per_category(5, 0);
        assert_eq!(result, Vec::<u32>::new());
    }
}
