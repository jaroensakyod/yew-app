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

          fn create(ctx: &Context<Self>) -> Self {
              
          }
}

fn main() {
    println!("Hello, world!");
}
