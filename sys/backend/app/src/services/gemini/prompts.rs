use crate::models::Category;

pub fn build_challenge_prompt(category: &Category, count: u32) -> String {
    let char_limit = category.char_limit;
    let category_name = &category.name;
    let category_desc = category
        .description
        .as_deref()
        .unwrap_or("No description provided");

    format!(
        r#"あなたは「言語化チャレンジ」アプリのお題作成AIです。

## カテゴリ情報
- カテゴリ名: {category_name}
- カテゴリ説明: {category_desc}
- 回答文字数制限: {char_limit}文字以内

## タスク
このカテゴリに適した、ユーザーが{char_limit}文字以内で回答できるチャレンジのお題を{count}個生成してください。

## お題の要件
1. ユーザーが自分の考えや経験を言語化できる質問であること
2. {char_limit}文字で十分に回答可能な範囲であること
3. 抽象的すぎず、具体的すぎない適度な粒度であること
4. ポジティブで前向きな回答を促すものであること
5. 多様な視点や回答が可能であること
6. 日本語で自然な表現であること

## 出力形式
以下のJSON形式で出力してください：

{{
  "challenges": [
    {{
      "title": "お題のタイトル（質問文）",
      "description": "お題の補足説明や回答のヒント",
      "char_limit": {char_limit}
    }}
  ]
}}

## 注意事項
- titleは「〜とは？」「〜を教えてください」などの質問形式にしてください
- descriptionは回答者の助けになるような補足情報を含めてください
- 必ず{count}個のお題を生成してください
- JSONのみを出力し、他の説明は含めないでください
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_build_challenge_prompt() {
        let category = Category {
            id: Uuid::new_v4(),
            name: "日常の気づき".to_string(),
            description: Some("日常生活での小さな発見や気づきを言語化する".to_string()),
            icon: None,
            color: None,
            char_limit: 30,
            sort_order: 1,
            status: "active".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let prompt = build_challenge_prompt(&category, 5);

        assert!(prompt.contains("日常の気づき"));
        assert!(prompt.contains("30文字"));
        assert!(prompt.contains("5個"));
    }
}
