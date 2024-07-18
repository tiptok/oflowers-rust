use std::boxed;


#[test]
fn boxed(){
    // 1.在堆上分配内存
    let boxed_value = Box::new(5);
    println!("Boxed value:{}",boxed_value);
    // 2.解引用
    let value = *boxed_value;
    println!("Out boxed value:{}",value);

    // Box 提供了所有权语义，当 Box 被移动时，所有权也随之移动，这有助于管理资源的生命周期。
    let another_box = boxed_value; // 所有权转移
    //println!("{}", boxed_value); // 错误，boxed_value 的所有权已转移

    use List::{Cons,Nil};
    let list = Cons(1,Box::new(Cons(2,Box::new(Nil))));
    println!("Value: {:?}",list);

    // 4.特征对象
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    for item in animals.iter(){
        item.speak();
    }

    Box::leak(Box::new(4));
}

/*
使用 Box 的注意事项
性能开销：
使用 Box 会带来一定的性能开销，因为它涉及堆内存分配和释放。尽量在需要时才使用 Box，否则应尽量使用栈分配的变量。

生命周期管理：
由于 Box 是一个智能指针，它会在超出作用域时自动释放内存。这意味着你不需要手动管理内存，但需要注意所有权和生命周期的问题，以避免悬挂指针或双重释放。
*/

trait Animal{
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat{
    fn speak(&self) {
        println!("Meow!");
    }
}

#[derive(Debug)]
enum List{
    Cons(i32,Box<List>),
    Nil,
}

// 测试对象销毁
struct Foo(u8);
impl Drop for Foo{
    fn drop(&mut self) {
        println!("{}",self.0);
    }
}
#[test]
fn test_drop(){
    {
        Foo(0);
    }
    let _foo = Foo(1);
    let _ = Foo(2);
    let _foo = Foo(3);
    Box::leak(Box::new(Foo(4)));
    const FOO_CONST: Foo = Foo(5);
    static mut FOO_STATIC:Foo = Foo(6);
    (0..2).for_each(|_element|{
        test1();
    });
    drop(FOO_CONST);
    unsafe {
        FOO_STATIC = FOO_CONST;
    }
}

fn test1(){
    static mut FOO:i32 = 7;
    unsafe {
        println!("{}",FOO);
        FOO +=1;
    }
}