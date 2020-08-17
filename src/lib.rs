use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    image_number: i32
}

enum Msg {
    AddOne,
    Change
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            image_number: 1
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value = self.value + 1,
            Msg::Change => if self.image_number == 2 {
                self.image_number = 1;
            } else {
                self.image_number = self.image_number + 1;
            }
        }
        
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "increment" }</button>
                <p>{ self.value }</p>
                <button onclick=self.link.callback(|_| Msg::Change)>{ "change image"}</button>
                <img src=format!("/images/{}.jpg", self.image_number) />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
