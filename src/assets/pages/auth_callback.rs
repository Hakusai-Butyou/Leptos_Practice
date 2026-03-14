use leptos::prelude::*;
use leptos_router::hooks::*;

#[component]
pub fn AuthCallback() -> impl IntoView {
    // URLのクエリパラメータを取得
    let query = use_query_map();

    // "code" と "state" を抽出
    let code = move || query.with(|q| q.get("code"));
    let state = move || query.with(|q| q.get("state"));
    if true {//ここでstate検証
        
    }
    view! {
        <div>
            <h2>"認証コールバック"</h2>
            // codeが存在する場合は表示し、なければエラーメッセージを表示
            <Show
                when=move || code().is_some()
                fallback=move || view! { <p>"エラー: Codeが見つかりません"</p> }
            >
                {move || view! { <p>"取得したCode: " <strong>{code().unwrap()}</strong></p> }}
            </Show>
            <Show
                when=move || state().is_some()
                fallback=move || view! { <p>"エラー: Stateが見つかりません"</p> }
            >
                {move || view! { <p>"取得したState: " <strong>{state().unwrap()}</strong></p> }}
            </Show>
        </div>
    }
}