use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

// Here's were the state goes.
pub struct App {
    value: i64,
    console: ConsoleService,
    input: String
}

pub enum Msg {
    DoIt,
    Input(String)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            console: ConsoleService::new(),
            value: 0,
            input: String::from("")
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::DoIt => {
                // Update your model on events
                self.value = self.value + 1;
                self.console.log("+1 bitch");
            }

            Msg::Input(val) => {
                self.input = val 
            }
        }
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <p>{ self.value }</p>
                <button onclick=|_| Msg::DoIt>{ "Click me!" }</button>
                <input id="input" oninput=|e| Msg::Input(e.value)/>
                <p>{&self.input}</p>
            </div>
        }
    }
}
