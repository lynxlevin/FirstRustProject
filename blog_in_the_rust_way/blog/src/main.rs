use blog::Post;

fn main() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");

  let post = post.request_review(); // post turns into PendingReviewPost.

  let post = post.reject(); // post turns into DraftPost.

  let post = post.request_review(); // post turns into PendingReviewPost.

  let post = post.approve(); // post turns into PendingAnotherReviewPost.

  let post = post.approve(); // post turns into Post.

  assert_eq!("I ate a salad for lunch today", post.content());

  println!("{}", post.content());
}