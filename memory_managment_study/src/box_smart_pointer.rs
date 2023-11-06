trait UIComponent {
    fn render(&self) {
        println!("rendering...");
    }
}

struct Button {
  text: String,
}

impl UIComponent for Button {}

struct Container {
  main: String,
  child: Box<Container>
}

impl UIComponent for Container {}

fn main () {
  let button = Button { text: "click me".to_owned() };
  let button_b = Box::new(Button { text: "click me".to_owned() });

  let button_c = button;
  let button_d = button_b;

  let components: Vec<Box<dyn UIComponent>> = vec![
    Box::new(button_c),
    button_d
  ];
}
