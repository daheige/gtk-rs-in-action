# 计数器实现
```rust
    // rc引用计数+cell内部可变性实现计数器
    // interior mutability 内在可变性，本身不能修改，但是内部的T可以修改
    let number = Rc::new(Cell::new(0));
    let number_cpy = number.clone();
    let but_incr = Button::builder()
        .label("incr")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let but_decr = Button::builder()
        .label("decr")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    but_incr.connect_clicked(move |_| number_cpy.set(number_cpy.get()+1));
    but_decr.connect_clicked(move |_| number.set(number.get()-1));
```
