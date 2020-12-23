use warmup::{bst, list};

pub fn main() {
    let mut bst = bst::Bst::new();

    for num in &[5, 1, 0, 2, -3, 11, 99, 100, 11, 12, 6, 5, 5, 99] {
        bst.insert(num);
    }

    println!("Pre-order...");
    bst.preorder();
    println!();

    println!("In-order...");
    bst.inorder();
    println!();

    println!("Post-order...");
    bst.postorder();
    println!();

    let xs = list::List::Cons(
        1,
        Box::new(list::List::Cons(
            2,
            Box::new(list::List::Cons(
                3,
                Box::new(list::List::Cons(
                    4,
                    Box::new(list::List::Cons(5, Box::new(list::List::Nil))),
                )),
            )),
        )),
    );
    println!("len(xs) = {}", xs.len());
    xs.display();
    println!();
}
