use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let css=r".sidebar {
                width: 200px;
                background-color: lightblue;
                nav{
                    display:flex;
                    flex-direction: column;
                }
            }";
    view! {
        <style inner_html=css />
        <div class="sidebar">
            <nav>
                <p>ホーム</p>
                <p>マイページ</p>
                <p>検索</p>
            </nav>
        </div>
    }
}