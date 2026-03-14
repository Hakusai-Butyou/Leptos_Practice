use leptos::prelude::*;
use leptos::web_sys;
use tracing;
use crate::server_func::login::get_google_auth_url;
#[component]
pub fn Login() -> impl IntoView {
    // サーバー関数を呼び出すためのActionを作成
    let login_action = Action::new(|_input: &()| async move {
        match get_google_auth_url().await {
            Ok(url) => {
                // web_sysを使用してブラウザをリダイレクト
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href(&url);
                }
            }
            Err(e) => {
                // エラーハンドリング（例: ログ出力）
                tracing::error!("URLの取得に失敗しました: {:?}", e);
            }
        }
    });

    view! {
        <button on:click=move |_| {login_action.dispatch(());}>
            "Googleでログイン"
        </button>
    }
}