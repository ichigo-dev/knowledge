use tauri::State;

use crate::AppState;
use crate::database::{ Term, Mask };

#[tauri::command]
pub async fn get_random_term_and_masks
(
    app_state: State<'_, AppState>,
) -> Result<(Term, Vec<Mask>), String>
{
    let sqlite_pool = &app_state.sqlite_pool;
    let quiz_result = &app_state.quiz_result;

    let mut tx = sqlite_pool.begin().await.unwrap();
    let sql = r#"
        SELECT
            *
        FROM
            `term`
        WHERE
            `term_id` not in
            (
                SELECT
                    `term_id`
                FROM
                    `answer`
                WHERE
                    `quiz_result_id` = ?
            )
        ORDER BY
            RANDOM()
        LIMIT
            1"#;
    let term: Term = sqlx::query_as(sql)
        .bind(quiz_result.quiz_result_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    let sql = r#"
        SELECT
            *
        FROM
            `mask`
        WHERE
            `term_id` = ?"#;
    let masks: Vec<Mask> = sqlx::query_as(sql)
        .bind(term.term_id)
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();
    Ok((term, masks))
}
