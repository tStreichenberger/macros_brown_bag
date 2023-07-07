#![allow(unused)]

fn main() {
    print_new_line!("macros_rule!");
}

/// Remove boilerplate of having to manually add
/// a newline at the end of your print statements
/// 
/// author: atharvapatil949
macro_rules! print_new_line {
    ($($arg:tt)*) => {
        print!($($arg)*);
        print!("\n");
    };
}
pub(crate) use print_new_line;



/**
The hygiene rules for local variables allows you to avoid name collisions in your macros
```
fn main() {
    time! {
        let start = "Start";
        print_new_line!("Start = {start}");
    }
}
```
 */
macro_rules! time {
    ($($t:tt)*) => {
        let start = ::std::time::Instant::now();
        $($t)*
        print_new_line!("Operation Took: {:?}", start.elapsed());
    }
}
/**
The hygiene rules also mean that you cannot reference the name var within the macro
```
fn main() {
    let name = "Doesn't Compile";
    print_name!();
}
```
 */
macro_rules! print_name {
    () => {
        print_new_line!("{}", name);
    };
}



/**
Good example of how you can define your own syntax with a macro
```
fn main() {
    let foo = 1;
    let bar = 2;

    let equality = either!(foo == bar => "Equal"; "Not Equal");
    print_new_line!("{equality}");
}
```
 */
macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    }
}
/**
This recursivly expands forever at compile time
So the build will fail with a recursion limit reached error
```
fn main() {
    circle1!();
}
```
*/
macro_rules! circle1 {
    () => {
        circle2!();
    }
}
macro_rules! circle2 {
    () => {
        circle1!();
    };
}

/**
```
fn main() {
    // old_name does not exist anymore so this won't comile
    old_name();
    // new_name _does_ exist so it can be used here
    new_name();
}
```
*/
struct AttachToDocComment;

#[my_proc_macro::change_name(new_name)]
fn old_name() {
    print_new_line!("Hi");
}

/**
This derive macro just expands to this trait implemenation
```
impl Clone for MyStruct {
    fn clone(&self) -> Self {
        Self {
            field1: self.field1.clone(),
            field2: self.field2.clone(),
            field3: self.field3.clone(),
        }
    }
}
```
*/
#[derive(my_proc_macro::Clone)]
struct MyStruct {
    field1: String,
    field2: u64,
    field3: Vec<u8>,
}