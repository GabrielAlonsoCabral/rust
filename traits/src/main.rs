use traits::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        print!("Select box has the following properties \n");
        println!(
            "Height: {:?} \nWidth:{:?}\nOptions:{:?}\n",
            self.height, self.width, self.options
        )
    }
}

fn main() {
    let screen: Screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 32,
                height: 32,
                options: vec![String::from("yes"), String::from("no")],
            }),
            Box::new(Button {
                height: 64,
                width: 64,
                label: String::from("Click me"),
            }),
        ],
    };

    screen.run()
}
