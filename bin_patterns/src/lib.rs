use lib_object_trait_pattern::*;
use lib_oops_pattern::*;

pub fn try_object_trait_pattern() {
    println!("\nObject Trait Pattern");
    let select_box_component = SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("No"),
            String::from("Maybe"),
        ]
    };

    let button_component = Button {
        width: 50,
        height: 10,
        label: String::from("OK")
    };

    let screen = Screen {
        components: vec![
            Box::new(select_box_component),
            Box::new(button_component),
        ]
    };

    screen.run();
}

pub fn try_oops_pattern() {
    println!("\nOops Pattern");
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    println!("Post : {:?}", post.content);
}