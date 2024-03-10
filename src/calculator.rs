// Always type #[macro_export] in order to export a macro in a crate
#[macro_export]
macro_rules! calculator {
    ($operator: expr, $num1: expr, $num2: expr) => {
        match $operator {
            "add" => $num1 + $num2,
            "multiply" => $num1 * $num2,
            "subtract" => $num1 - $num2,
            "divide" => {
                if ($num2 != 0.0) {
                    $num1 / $num2
                } else {
                    panic!("Zero Division!")
                }
            },
            _ => panic!("Unknown operator: {}", $operator),
        }
    };
}
