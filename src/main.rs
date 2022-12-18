use stylist::{css, style};
use stylist::yew::Global;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let increment = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let decrement = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    let global_style = css!(r#"
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        html {
            height: 100%;
        }
        
        body {
            background-color: #f0f0f0;
            display: flex;
            flex-direction: column;
            font-family: system-ui;
            min-height: 100%;
        }
    "#);

    let container_style = style!(r#"
        align-items: center;
        display: flex;
        flex: 1 0 auto;
        flex-direction: column;
        justify-content: center;
        padding: 2rem;
    "#).expect("Failed to mount style");

    let panel_style = style!(r#"
        align-items: center;
        background-color: #fff;
        border-radius: 16px;
        display: flex;
        flex: 1 0 auto;
        flex-direction: column;
        justify-content: center;
        padding: 2rem;
        width: min(960px, 100%);
    "#).expect("Failed to mount style");

    let title_style = style!(r#"
        font-size: 2rem;
        font-weight: 400;
    "#).expect("Failed to mount style");

    let counter_style = style!(r#"
        font-size: 8rem;
        font-weight: 700;
    "#).expect("Failed to mount style");

    let button_area_style = style!(r#"
        display: flex;
        gap: 1rem;
    "#).expect("Failed to mount style");

    let button_style = style!(r#"
        appearance: none;
        border: none;
        border-radius: 8px;
        cursor: pointer;
        flex-grow: 1;
        font-family: inherit;
        font-size: 1.5rem;
        font-weight: 700;
        padding: 0.5rem 1rem;
        width: 64px;
    "#).expect("Failed to mount style");

    let button_increment_style = style!(r#"
        background-color: #55c500;
    "#).expect("Failed to mount style");

    let button_decrement_style = style!(r#"
        background-color: #eee;
    "#).expect("Failed to mount style");

    html! {
        <>
            <Global css={global_style} />
            <div class={container_style}>
                <div class={panel_style}>
                    <h1 class={title_style}>{ "Counter with Yew" }</h1>
                    <span class={counter_style}>{ *counter }</span>
                    <div class={button_area_style}>
                        <button onclick={decrement} class={classes!(button_style.clone(), button_decrement_style)}>{ "-1" }</button>
                        <button onclick={increment} class={classes!(button_style.clone(), button_increment_style)}>{ "+1" }</button>
                    </div>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
