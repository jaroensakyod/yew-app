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
          fn up
}

fn main() {
    println!("Hello, world!");
}
