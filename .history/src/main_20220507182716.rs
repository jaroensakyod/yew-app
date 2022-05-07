use yew::prelude::*;

enum Msg{
          AddOne
}
struct CounterComponent {
          count: i64,
}

impl Component for CounterComponent {
          type Message = Msg;
          type Properties = ();

          fn create(_ctx: &Context<Self>) -> Self {
              Self {count:0}
          }
          fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
              match msg {
                        Msg::AddOne => {
                                  self.count += 1;
                                  true
                    }
              }
          }
          fn view(&self, _ctx: &Context<Self>) ->Html {
                    let link = ctx.link();
                    html!{
                              <div id="counter">
                                        <p>{self.count}</p>
                                        <button onclick={line!()}> {"+1"} </button>
                              </div>
                    }
          }
}

fn main() {
    println!("Hello, world!");
}
