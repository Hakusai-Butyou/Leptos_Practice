use leptos::prelude::*;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeVerifier};
use std::env::var;
use rand::{self, RngExt};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;

fn generate_state() -> String {
    let mut rng = rand::rng();      // スレッドローカルCSPRNG
    let mut bytes = [0u8; 32];      // 256-bit
    rng.fill(&mut bytes);           // ランダムで埋める
    URL_SAFE_NO_PAD.encode(bytes)   // Base64URLにエンコード
}
// サーバー側で実行される関数
#[server(GetGoogleAuthUrl, "/api")]
pub async fn get_google_auth_url() -> Result<String, ServerFnError> {
    // 実際の実装では環境変数 (std::env::var) から取得します
    let client_id = var("CLIENT_ID").unwrap(); 
    let redirect_uri = "http://localhost:3000/auth/callback";
    
    // CSRF対策のstateパラメータ。実際にはセキュアな乱数を生成し、サーバー側のセッションに保存します
    let state = generate_state(); 

    // Googleの認可エンドポイントURLを構築
    // ※本格的な実装では `oauth2` クレートを使用してURLを構築することを推奨します
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&state={}",
        client_id, redirect_uri, state
    );

    Ok(auth_url)
}