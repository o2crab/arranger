use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Block {
    pub name: String,
    pub content: String,
}

#[derive(Properties, PartialEq)]
pub struct BlockListProps {
    pub block_order: Vec<usize>,
    pub block_content: HashMap<usize, Block>,
    pub ondragstart: Callback<usize>,
    pub ondragend: Callback<()>,
    pub ondrop: Callback<usize>,
    pub oninput: Callback<(usize, InputEvent)>,
}

#[function_component]
pub fn BlockList(BlockListProps { block_order, block_content, ondragstart, ondragend, ondrop, oninput }: &BlockListProps) -> Html {
    block_order.iter().enumerate().map(|(i,block_id)| {
        let ondragstart = {
            let ondragstart = ondragstart.clone();
            Callback::from(move |_| {
                ondragstart.emit(i)
            })
        };
        let ondragend = {
            let ondragend = ondragend.clone();
            Callback::from(move |_| {
                ondragend.emit(())
            })
        };
        let ondrop = {
            let ondrop = ondrop.clone();
            Callback::from(move |_| {
                ondrop.emit(i)
            })
        };
        let oninput = {
            let oninput = oninput.clone();
            Callback::from(move |e| {
                oninput.emit((i, e))
            })
        };
        let not_found_block = Block {
            name: String::from("block???"),
            content: String::from("Content Not Found"),
        };
        let block = block_content.get(block_id).unwrap_or(&not_found_block);
        html! {
            <div class="block_container" key={i} draggable="true" {ondragstart} {ondragend} {ondrop} ondragover={|e: DragEvent| {e.prevent_default()}}>
                <p>{ block.name.clone() }</p>
                <textarea cols="60" rows="3" value={block.content.clone()} {oninput}></textarea>
            </div>
        }
    }).collect()
}
