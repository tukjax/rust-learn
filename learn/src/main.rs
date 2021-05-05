fn main() {

    // immutable const
    // 1. every thing is immutable by default
    let a = "x";

    //of cause,we can also make variable mutable,just like this
    let mut a = 3;
    //you may noticed , we changed the definition of variable "a" from immutable to mutable
    //that is called Shadowing in rust




    // special notice, rust has a special concept which called "ownership"
    //see below
    let s1 = String::from("hello");
    let s2 = s1; // after this , s1 is dead!!!
    // any operator to s1 below this line is wrong
    // println!("{}", s1); this is not right

    // you can do this like, this will clone a brand new space in heap,so s1 and s2 points different memory space
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);


    //Reference AND Borrow
    //sometimes, you dont want waster too much space, you can do that
    let s1 = String::from("hello");
    let s2 = & s1;
    println!("s1 is {}, s2 is {}", s1, s2);
    // that called Borrow,but you can't write anything to s2, s2 is readonly
    // wait, there is more, s1 could lost the ownership with "hello" ! which may result in
    let s3 = s1;
    //Oops! s2 cant be used now!
    //println!(" s2 is {}",s2); that wont pass the compile
    //so , we should do that:
    let s2 = &s3;
    //that make s2 useful again

    //Reference
    //if you are familiar with java or c++ like languages
    //    let s1 = String::from("hello");
    // this means, we open a new memory space in mem, and s1 is a pointer in stack which point at the object in heap.



}
