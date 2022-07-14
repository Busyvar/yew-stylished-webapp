use material_yew::text_inputs::*;
use material_yew::*;
use yew::prelude::*;

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    value: i64, // This will store the counter value
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let val: f32 = self.value as f32 / 10.;
        html! {
            <div>
                <span onclick={ctx.link().callback(|_| Msg::Increment)} >
                <MatButton label="first" outlined=true/>
                </span>
                <div class="panel">
                // A button to send the Increment message
                <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                { "+1" }
            </button>
                <MatButton label="sh" raised=false outlined=true disabled=true/>
                <MatFab icon="edit"/>
                <MatCircularProgress progress={val} />
                <MatIconButton icon="code"></MatIconButton>
                // <MatButton raised=true onclick={ctx.link().callback(|_| Msg::Increment)} label="Click me" />

                // A button to send the Decrement message
                <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                { "-1" }
            </button>
                    <MatTextField label="Standard" icon="event" field_type={TextFieldType::Date} />

                // A button to send two Increment messages
                <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Decrement])}>
                { "+1, +1" }
            </button>

                </div>

                // Display the current value of the counter
                <p class="counter">
                { self.value }
            </p>

                // Display the current date and time the page was rendered
                <p class="footer">
                { "Rendered: " }
            </p>
                </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
