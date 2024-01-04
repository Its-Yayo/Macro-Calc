// FIXME: Check this macro
macro_rules! calculator {
    ($operator: ident, $num1: expr, $num2: expr) => {
        match $operator {
            "add" => $num1 + $num2,
            "multiply" => $num1 * $num2,
            "subtract" => $num1 - $num2,
            "divide" => $num1 / $num2,
        }
    };
}

// FIXME: Check this function, it'll be useless
fn call_this(method: &str, int_1: i32, int_2: i32) {
    calculator!(method, int_1, int_2);
}