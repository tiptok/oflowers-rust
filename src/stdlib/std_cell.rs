use std::cell::{RefCell,Cell};

// Cell
// Cell<T>是一个提供内部可变性的类型，它允许在持有不可变引用的情况下修改其内部值。
// Cell通过复制语义来工作，这意味着它只能包含实现Copy trait的类型（例如基本类型，如整数和浮点数）。
// 使用Cell可以绕过Rust的借用检查器的限制，在表面上看起来不可变的情况下进行修改。
#[test]
fn cell(){
    let cell = Cell::new(42);
    println!("Initial value:{}",cell.get());

    cell.set(100);
    println!("Modified value:{}",cell.get());
}

// RefCell
// RefCell<T>也是一个提供内部可变性的类型，它允许在持有不可变引用的情况下修改其内部值。
// 与Cell不同，RefCell通过运行时检查借用规则，而不是编译时检查。这意味着可以在运行时检测出违反借用规则的情况。
// RefCell允许借用其内部值的可变和不可变引用，通过调用borrow和borrow_mut方法来获取。
#[test]
fn ref_cell(){
    let ref_cell = RefCell::new(42);

    {
        let value = ref_cell.borrow();
        println!("Value: {}", *value);
    }

    // Borrowing a mutable reference
    {
        let mut value = ref_cell.borrow_mut();
        *value = 100;
    }

    // Borrowing again to see the modified value
    {
        let value = ref_cell.borrow();
        println!("Modified value: {}", *value);
    }
}

// 主要区别
// Cell适用于包含实现Copy trait的类型，且不能借用其内部值，只能通过get和set方法访问。
// RefCell适用于任何类型，并允许在运行时检查借用规则，可以借用其内部值的可变和不可变引用。