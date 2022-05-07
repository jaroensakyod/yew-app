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
              
          }
}

fn main() {
    println!("Hello, world!");
}
