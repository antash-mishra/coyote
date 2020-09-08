#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class = "flex mb-4 h-max">
                <div class = "w-1/3 bg-blue-400 h-max py-20 text-center">
                    <h1 class="text-4xl font-bold font-bold text-white mt-10">{ "Around" }</h1>
                    <p class="text-lg font-sans text-white font-medium antialiased"> { "Dashboard for your professional news" }</p>                    
                    <div class="w-full max-w-xs py-20 display-flex text-center"> 
                        <form class="w-full max-w-sm py-8 md:item-center">
                            <div class="mb-6 px-4 m-auto" >
                                <input class="appearance-none bg-pink-200 border-gray rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                    id="name" 
                                    type="text" 
                                    placeholder="Name" />
                            </div>
                            <div class="px-4 mt-1 mb-4">
                                <input class="appearance-none border-gray bg-pink-200 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                    id="email"
                                    type="email"
                                    placeholder="Email" />
                            </div>
                            <div class="flex item-center px-4 mt-1 mb-4">
                                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 border onject-cover border-blue-700 rounded w-3/4">
                                    {"Proceed"}
                                </button>
                            </div>
                        </form>
                    </div>             
                </div>
                <div class = "w-2/3 bg-white h-max">
                    <div class= "cards">

                        <div class="card">  
                    
                            <div class="card_part card_part-one">
                            </div>
                
                            <div class="card_part card_part-two">              
                            </div>
              
                            <div class="card_part card_part-three">
                            </div>
              
                            <div class="card_part card_part-four">
                            </div>
              
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}