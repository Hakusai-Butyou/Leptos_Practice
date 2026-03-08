use leptos::prelude::*;
#[component]
pub fn Header() -> impl IntoView {
    let css=r"
        .header {
            background-color: red;
            width: 100%;
            height: 100px;
            display: flex;
            align-items: center;
            nav{
                display:flex;
                > *{
                    flex:1 1 0
                }
                flex:1 1 0;
            }
            .header-icon{
                width: 90px;
                height: 90px;
                margin:5px;
                object-fit: contain;
                flex:0 0 auto;
            }
        }";
    view! {
        <style inner_html=css />
        <header class="header">
            <img alt="アプリアイコン" src="/app_icons/icon.ico" class="header-icon"/>
            <nav>
                <p>通知</p>
                <p>新規登録</p>
                <p>ログイン</p>
            </nav>
        </header>
    }
}