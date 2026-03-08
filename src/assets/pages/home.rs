use leptos::prelude::*;
#[component]
pub fn Home() -> impl IntoView {
    let css=r"
    .card{
        --banner-height:150px;
        --user-data-height:150px;
        --icon-size:150px;
        margin:10px;
        border:solid 3px grey;
        border-radius:30px;
        background-color:white;
        display:flex;
        flex-direction:column;
        .banner-img{
            height:var(--banner-height);
            width:100%;
            background-color:lightblue;
            border-top-left-radius:30px;
            border-top-right-radius:30px;
        }
        .icon-img{
            height:var(--icon-size);
            width:var(--icon-size);
            margin-block:calc(var(--icon-size) * -0.5);
            margin-inline-start:25px;
            background-color:darkblue;
            clip-path: circle(50%);
        }
        .user-data-holder{
            --margin-inline-start:calc(25px + var(--icon-size));
            margin-inline-start:var(--margin-inline-start);
            width:calc(100% - var(--margin-inline-start));
            height:var(--user-data-height);
            text-align:left;
            .account-links{
                display:flex;
                flex-direction:column;
            }
        }
    }";
    view! {
        <style inner_html=css />
        <div class="card">
            <img src="" alt="バナー画像" class="banner-img"/>
            <img src="" alt="アイコン"  class="icon-img"/>
            <div class="user-data-holder">
                <h1>ユーザー名</h1>
                <p>説明</p>
                <div class="account-links">
                </div>
            </div>
        </div>
    }
}