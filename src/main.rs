use std::collections::HashMap;
use yew::prelude::*;
use web_sys::HtmlInputElement;

mod block;
use block::*;

#[function_component(App)]
fn app() -> Html {

    let block_order = use_state(|| vec![0,1,2]);

    let mut block_content = HashMap::new();
    for i in 0..(*block_order).len() {
        block_content.insert(
            i,
            Block {
                name: format!("block{}", i),
                content: format!("This is block{}.", i),
            }
        );
    }
    let block_content = use_state(|| block_content);

    let dragged = use_state(|| None);

    let ondragstart = {
        let dragged = dragged.clone();
        Callback::from(move |i| {
            dragged.set(Some(i));
        })
    };
    let ondragend = {
        let dragged = dragged.clone();
        Callback::from(move |_| {
            dragged.set(None);
        })
    };
    let ondrop = {
        let block_order = block_order.clone();
        let dragged = dragged.clone();
        Callback::from(move |dropzone| {
            if let Some(dragged) = *dragged {
                let mut v = (*block_order).clone();
                let dragged_block_id = v.remove(dragged);
                v.insert(dropzone, dragged_block_id);
                block_order.set(v);
            }
        })
    };
    let oninput = {
        let block_order = block_order.clone();
        let block_content = block_content.clone();
        Callback::from(move |(i, e): (usize, InputEvent)| {
            let new_content = e.target_unchecked_into::<HtmlInputElement>().value();
            let mut map = (*block_content).clone();
            if let Some(v) = map.get_mut(&block_order[i]) {
                v.content = new_content;
            }
            block_content.set(map);
        })
    };

    // let onkeydown = {
        // let block_content = block_content.clone();
        // move |e: KeyboardEvent| {
            // if e.is_composing() {
                // return;
            // }
            // if e.code() == String::from("Enter") {
                // let block_id = *next_block_id.borrow_mut();
                // let mut map = (*block_content).clone();
                // map.insert(
                    // block_id,
                    // Block {
                        // name: format!("block{}", block_id),
                        // content: format!("this is block{}", block_id),
                    // }
                // );
                // block_content.set(map);
                // *next_block_id.borrow_mut() += 1;
            // }
        // }
    // };


    html! {
        <>
            <h1>{ "Arranger" }</h1>
            <BlockList block_order={(*block_order).clone()} block_content={(*block_content).clone()} {ondragstart} {ondragend} {ondrop} {oninput}/>
            if let Some(i) = *dragged {
                <h2>{ format!("dragging: {}", i) }</h2>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

