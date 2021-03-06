use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("OK!");

    post.request_review();
    assert_eq!("", post.content());
    println!("OK!");

    post.approve();
    assert_eq!("", post.content());
    println!("OK!");

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("OK!");

    post.add_text("");
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("OK!");
}
