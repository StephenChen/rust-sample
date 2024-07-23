use state_pattern::Post;

fn main() {
    println!("Hello, world!");

    let mut post = Post::new();

    post.add_text("aaa");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("aaa", post.content());
}
