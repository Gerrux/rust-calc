use std::f64::consts::PI;

pub struct Calculator {
    pub expression: String,
    pub display: String,
    pub history: Vec<HistoryEntry>,
    pub last_result: Option<f64>,
    pub angle_mode: AngleMode,
    pub open_parens: i32,
}

#[derive(Clone)]
pub struct HistoryEntry {
    pub expression: String,
    pub result: String,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AngleMode {
    Radians,
    Degrees,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            expression: String::new(),
            display: String::from("0"),
            history: Vec::new(),
            last_result: None,
            angle_mode: AngleMode::Degrees,
            open_parens: 0,
        }
    }
}

impl Calculator {
    /// Input a digit (0-9)
    pub fn input_digit(&mut self, digit: &str) {
        if self.display == "0" && digit != "." {
            self.display = digit.to_string();
        } else {
            self.display.push_str(digit);
        }
        self.expression.push_str(digit);
    }

    /// Input decimal point with validation
    pub fn input_decimal(&mut self) {
        // Find the last number in expression and check if it has a decimal
        let last_num = self
            .expression
            .rsplit(|c: char| "+-×÷()".contains(c))
            .next()
            .unwrap_or("");

        if !last_num.contains('.') {
            if self.display == "0" || self.display.ends_with(|c: char| "+-×÷(".contains(c)) {
                self.display.push_str("0.");
                self.expression.push_str("0.");
            } else {
                self.display.push('.');
                self.expression.push('.');
            }
        }
    }

    /// Input operator with validation (prevents consecutive operators)
    pub fn input_operator(&mut self, op: &str) {
        if self.expression.is_empty() {
            if op == "−" {
                self.expression.push_str(op);
                self.display = op.to_string();
            }
            return;
        }

        let last_char = self.expression.chars().last().unwrap();

        // Replace last operator if there is one
        if "+-×÷".contains(last_char) || "+-×÷".contains(self.get_last_char_normalized()) {
            self.expression.pop();
            if self.display.len() > 1 {
                self.display.pop();
            }
        }

        // Don't allow operator right after opening paren (except minus)
        if last_char == '(' && op != "−" {
            return;
        }

        self.expression.push_str(op);
        self.display.push_str(op);
    }

    /// Input function (sin, cos, etc.)
    pub fn input_function(&mut self, func: &str) {
        // Add implicit multiplication if needed
        if !self.expression.is_empty() {
            let last = self.expression.chars().last().unwrap();
            if last.is_ascii_digit() || last == ')' || last == 'π' || last == 'e' {
                self.expression.push('×');
                self.display.push('×');
            }
        }

        self.expression.push_str(func);
        self.expression.push('(');
        self.display.push_str(func);
        self.display.push('(');
        self.open_parens += 1;
    }

    /// Input constant (π, e)
    pub fn input_constant(&mut self, constant: &str) {
        // Add implicit multiplication if needed
        if !self.expression.is_empty() {
            let last = self.expression.chars().last().unwrap();
            if last.is_ascii_digit() || last == ')' || last == 'π' || last == 'e' {
                self.expression.push('×');
                self.display.push('×');
            }
        }

        self.expression.push_str(constant);
        self.display.push_str(constant);
    }

    /// Input opening parenthesis
    pub fn input_open_paren(&mut self) {
        // Add implicit multiplication if needed
        if !self.expression.is_empty() {
            let last = self.expression.chars().last().unwrap();
            if last.is_ascii_digit() || last == ')' || last == 'π' || last == 'e' {
                self.expression.push('×');
                self.display.push('×');
            }
        }

        self.expression.push('(');
        self.display.push('(');
        self.open_parens += 1;
    }

    /// Input closing parenthesis (only if there are open ones)
    pub fn input_close_paren(&mut self) {
        if self.open_parens > 0 {
            let last = self.expression.chars().last().unwrap_or('(');
            // Don't close empty parens or right after operator
            if last != '(' && !"+-×÷".contains(last) {
                self.expression.push(')');
                self.display.push(')');
                self.open_parens -= 1;
            }
        }
    }

    /// Input square (^2)
    pub fn input_square(&mut self) {
        if !self.expression.is_empty() {
            let last = self.expression.chars().last().unwrap();
            if last.is_ascii_digit() || last == ')' || last == 'π' || last == 'e' {
                self.expression.push_str("^2");
                self.display.push('²');
            }
        }
    }

    /// Input power (^)
    pub fn input_power(&mut self) {
        if !self.expression.is_empty() {
            let last = self.expression.chars().last().unwrap();
            if last.is_ascii_digit() || last == ')' || last == 'π' || last == 'e' {
                self.expression.push('^');
                self.display.push('^');
            }
        }
    }

    /// Input percent (/100)
    pub fn input_percent(&mut self) {
        if !self.expression.is_empty() {
            let last = self.expression.chars().last().unwrap();
            if last.is_ascii_digit() || last == ')' {
                self.expression.push_str("/100");
                self.display.push('%');
            }
        }
    }

    /// Toggle sign of current number
    pub fn toggle_sign(&mut self) {
        if self.expression.is_empty() || self.display == "0" {
            return;
        }

        // Find the start of the current number
        let mut start_idx = self.expression.len();
        for (i, c) in self.expression.char_indices().rev() {
            if "+-×÷(".contains(c) {
                start_idx = i + c.len_utf8();
                break;
            }
            if i == 0 {
                start_idx = 0;
            }
        }

        // Check if there's already a negative sign
        if start_idx > 0 {
            let prev_char = self.expression[..start_idx].chars().last();
            if prev_char == Some('−') || prev_char == Some('-') {
                // Remove the minus
                let remove_pos = start_idx - '−'.len_utf8();
                self.expression = format!(
                    "{}{}",
                    &self.expression[..remove_pos],
                    &self.expression[start_idx..]
                );
            } else {
                // Add minus
                self.expression.insert(start_idx, '−');
            }
        } else {
            // At the beginning
            if self.expression.starts_with('−') || self.expression.starts_with('-') {
                self.expression = self.expression.chars().skip(1).collect();
            } else {
                self.expression.insert(0, '−');
            }
        }

        self.display = self.expression.clone();
    }

    /// Clear everything
    pub fn clear(&mut self) {
        self.expression.clear();
        self.display = String::from("0");
        self.open_parens = 0;
    }

    /// Clear last entry (backspace) - handles multi-byte unicode
    pub fn clear_entry(&mut self) {
        if self.expression.is_empty() {
            return;
        }

        // Get the last character
        let last_char = self.expression.chars().last().unwrap();

        // Update open_parens counter
        if last_char == '(' {
            self.open_parens -= 1;
        } else if last_char == ')' {
            self.open_parens += 1;
        }

        // Remove last character properly (handles unicode)
        let new_len = self.expression.len() - last_char.len_utf8();
        self.expression.truncate(new_len);

        // Update display
        if self.expression.is_empty() {
            self.display = String::from("0");
        } else {
            self.display = self.expression.clone();
        }
    }

    /// Calculate result
    pub fn calculate(&mut self) -> Result<f64, String> {
        if self.expression.is_empty() {
            return Ok(0.0);
        }

        // Check for invalid ending
        let last = self.expression.chars().last().unwrap();
        if "+-×÷(".contains(last) {
            return Err("Incomplete".to_string());
        }

        let expr = self.preprocess_expression();

        match std::panic::catch_unwind(|| meval::eval_str(&expr)) {
            Ok(eval_result) => match eval_result {
                Ok(result) => {
                    if result.is_nan() {
                        return Err("Undefined".to_string());
                    }
                    if result.is_infinite() {
                        return Err("Infinity".to_string());
                    }

                    let formatted = format_result(result);

                    self.history.push(HistoryEntry {
                        expression: self.display.clone(),
                        result: formatted.clone(),
                    });

                    if self.history.len() > 100 {
                        self.history.remove(0);
                    }

                    self.last_result = Some(result);
                    self.expression = formatted.clone();
                    self.display = formatted;
                    self.open_parens = 0;

                    Ok(result)
                }
                Err(e) => Err(simplify_error(&e.to_string())),
            },
            Err(_) => Err("Error".to_string()),
        }
    }

    /// Use result from history
    pub fn use_history(&mut self, result: &str) {
        if self.display == "0" {
            self.expression = result.to_string();
            self.display = result.to_string();
        } else {
            self.expression.push_str(result);
            self.display.push_str(result);
        }
    }

    fn get_last_char_normalized(&self) -> char {
        self.expression
            .chars()
            .last()
            .map(|c| match c {
                '−' => '-',
                '×' => '*',
                '÷' => '/',
                _ => c,
            })
            .unwrap_or(' ')
    }

    fn preprocess_expression(&self) -> String {
        let mut expr = self.expression.clone();

        // Replace display symbols with math symbols
        expr = expr.replace("×", "*");
        expr = expr.replace("÷", "/");
        expr = expr.replace("−", "-");
        expr = expr.replace("π", "pi");

        // Auto-close parentheses
        for _ in 0..self.open_parens {
            expr.push(')');
        }

        // Add implicit multiplication for constants
        expr = add_implicit_multiplication(&expr);

        // Convert degrees to radians for trig functions
        if self.angle_mode == AngleMode::Degrees {
            expr = convert_trig_to_radians(&expr);
        }

        expr
    }

    pub fn toggle_angle_mode(&mut self) {
        self.angle_mode = match self.angle_mode {
            AngleMode::Radians => AngleMode::Degrees,
            AngleMode::Degrees => AngleMode::Radians,
        };
    }

    pub fn get_open_parens(&self) -> i32 {
        self.open_parens
    }
}

fn convert_trig_to_radians(expr: &str) -> String {
    let mut result = expr.to_string();
    let deg_to_rad = PI / 180.0;

    for func in ["sin", "cos", "tan"] {
        let pattern = format!("{}(", func);
        let mut new_result = String::new();
        let mut remaining = result.as_str();

        while let Some(pos) = remaining.find(&pattern) {
            new_result.push_str(&remaining[..pos]);
            remaining = &remaining[pos..];

            if let Some(end) = find_closing_paren(remaining, func.len()) {
                let inner = &remaining[func.len() + 1..end];
                new_result.push_str(&format!("{}(({})*{})", func, inner, deg_to_rad));
                remaining = &remaining[end + 1..];
            } else {
                new_result.push_str(&remaining[..pattern.len()]);
                remaining = &remaining[pattern.len()..];
            }
        }
        new_result.push_str(remaining);
        result = new_result;
    }

    result
}

fn find_closing_paren(s: &str, func_len: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let start = func_len + 1;

    if start >= bytes.len() || bytes[func_len] != b'(' {
        return None;
    }

    let mut depth = 1;
    for (i, &byte) in bytes.iter().enumerate().skip(start) {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn add_implicit_multiplication(expr: &str) -> String {
    let mut result = String::with_capacity(expr.len() * 2);
    let chars: Vec<char> = expr.chars().collect();
    let functions = [
        "sin", "cos", "tan", "asin", "acos", "atan", "log10", "ln", "sqrt", "abs", "exp", "pi",
    ];

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];

        // Check for function
        let mut found_func = false;
        for func in &functions {
            if expr[char_to_byte_index(expr, i)..].starts_with(func) {
                if !result.is_empty() {
                    let last = result.chars().last().unwrap();
                    if last.is_ascii_digit() || last == ')' {
                        result.push('*');
                    }
                }
                result.push_str(func);
                i += func.len();
                found_func = true;
                break;
            }
        }

        if found_func {
            continue;
        }

        // Handle ( after digit or )
        if c == '(' && !result.is_empty() {
            let last = result.chars().last().unwrap();
            if last.is_ascii_digit() || last == ')' {
                result.push('*');
            }
        }

        // Handle digit after )
        if c.is_ascii_digit() && !result.is_empty() {
            let last = result.chars().last().unwrap();
            if last == ')' {
                result.push('*');
            }
        }

        result.push(c);
        i += 1;
    }

    result
}

fn char_to_byte_index(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

fn format_result(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e12 {
        format!("{}", value as i64)
    } else if value.abs() < 1e-8 || value.abs() >= 1e12 {
        format!("{:.6e}", value)
    } else {
        let formatted = format!("{:.10}", value);
        formatted
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

fn simplify_error(err: &str) -> String {
    if err.contains("parse") || err.contains("Parse") {
        "Syntax error".to_string()
    } else if err.contains("parenthesis") {
        "Missing )".to_string()
    } else {
        "Error".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let calc = Calculator::default();
        assert_eq!(calc.display, "0");
        assert!(calc.expression.is_empty());
        assert_eq!(calc.open_parens, 0);
        assert_eq!(calc.angle_mode, AngleMode::Degrees);
    }

    #[test]
    fn test_input_digit() {
        let mut calc = Calculator::default();
        calc.input_digit("5");
        assert_eq!(calc.display, "5");
        assert_eq!(calc.expression, "5");

        calc.input_digit("3");
        assert_eq!(calc.display, "53");
        assert_eq!(calc.expression, "53");
    }

    #[test]
    fn test_input_decimal() {
        let mut calc = Calculator::default();
        calc.input_digit("3");
        calc.input_decimal();
        calc.input_digit("1");
        calc.input_digit("4");
        assert_eq!(calc.expression, "3.14");

        // Should not add second decimal
        calc.input_decimal();
        assert_eq!(calc.expression, "3.14");
    }

    #[test]
    fn test_basic_addition() {
        let mut calc = Calculator::default();
        calc.input_digit("2");
        calc.input_operator("+");
        calc.input_digit("3");
        let result = calc.calculate().unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_basic_subtraction() {
        let mut calc = Calculator::default();
        calc.input_digit("1");
        calc.input_digit("0");
        calc.input_operator("−");
        calc.input_digit("4");
        let result = calc.calculate().unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_basic_multiplication() {
        let mut calc = Calculator::default();
        calc.input_digit("6");
        calc.input_operator("×");
        calc.input_digit("7");
        let result = calc.calculate().unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_basic_division() {
        let mut calc = Calculator::default();
        calc.input_digit("2");
        calc.input_digit("0");
        calc.input_operator("÷");
        calc.input_digit("4");
        let result = calc.calculate().unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_power() {
        let mut calc = Calculator::default();
        calc.input_digit("2");
        calc.input_power();
        calc.input_digit("1");
        calc.input_digit("0");
        let result = calc.calculate().unwrap();
        assert_eq!(result, 1024.0);
    }

    #[test]
    fn test_square() {
        let mut calc = Calculator::default();
        calc.input_digit("9");
        calc.input_square();
        let result = calc.calculate().unwrap();
        assert_eq!(result, 81.0);
    }

    #[test]
    fn test_parentheses() {
        let mut calc = Calculator::default();
        calc.input_open_paren();
        calc.input_digit("2");
        calc.input_operator("+");
        calc.input_digit("3");
        calc.input_close_paren();
        calc.input_operator("×");
        calc.input_digit("4");
        let result = calc.calculate().unwrap();
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_sqrt_function() {
        let mut calc = Calculator::default();
        calc.input_function("sqrt");
        calc.input_digit("1");
        calc.input_digit("4");
        calc.input_digit("4");
        calc.input_close_paren();
        let result = calc.calculate().unwrap();
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_pi_constant() {
        let mut calc = Calculator::default();
        calc.input_constant("π");
        let result = calc.calculate().unwrap();
        assert!((result - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_e_constant() {
        let mut calc = Calculator::default();
        calc.input_constant("e");
        let result = calc.calculate().unwrap();
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }

    #[test]
    fn test_sin_degrees() {
        let mut calc = Calculator::default();
        calc.angle_mode = AngleMode::Degrees;
        calc.input_function("sin");
        calc.input_digit("3");
        calc.input_digit("0");
        calc.input_close_paren();
        let result = calc.calculate().unwrap();
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_cos_degrees() {
        let mut calc = Calculator::default();
        calc.angle_mode = AngleMode::Degrees;
        calc.input_function("cos");
        calc.input_digit("6");
        calc.input_digit("0");
        calc.input_close_paren();
        let result = calc.calculate().unwrap();
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_clear() {
        let mut calc = Calculator::default();
        calc.input_digit("1");
        calc.input_digit("2");
        calc.input_digit("3");
        calc.clear();
        assert_eq!(calc.display, "0");
        assert!(calc.expression.is_empty());
    }

    #[test]
    fn test_clear_entry() {
        let mut calc = Calculator::default();
        calc.input_digit("1");
        calc.input_digit("2");
        calc.input_digit("3");
        calc.clear_entry();
        assert_eq!(calc.expression, "12");
        calc.clear_entry();
        assert_eq!(calc.expression, "1");
        calc.clear_entry();
        assert_eq!(calc.display, "0");
    }

    #[test]
    fn test_history() {
        let mut calc = Calculator::default();
        calc.input_digit("5");
        calc.input_operator("+");
        calc.input_digit("5");
        calc.calculate().unwrap();

        assert_eq!(calc.history.len(), 1);
        assert_eq!(calc.history[0].result, "10");
    }

    #[test]
    fn test_percent() {
        let mut calc = Calculator::default();
        calc.input_digit("5");
        calc.input_digit("0");
        calc.input_percent();
        let result = calc.calculate().unwrap();
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_toggle_angle_mode() {
        let mut calc = Calculator::default();
        assert_eq!(calc.angle_mode, AngleMode::Degrees);
        calc.toggle_angle_mode();
        assert_eq!(calc.angle_mode, AngleMode::Radians);
        calc.toggle_angle_mode();
        assert_eq!(calc.angle_mode, AngleMode::Degrees);
    }

    #[test]
    fn test_division_by_zero() {
        let mut calc = Calculator::default();
        calc.input_digit("1");
        calc.input_operator("÷");
        calc.input_digit("0");
        let result = calc.calculate();
        assert!(result.is_err() || result.unwrap().is_infinite());
    }

    #[test]
    fn test_empty_expression() {
        let mut calc = Calculator::default();
        let result = calc.calculate().unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_incomplete_expression() {
        let mut calc = Calculator::default();
        calc.input_digit("5");
        calc.input_operator("+");
        let result = calc.calculate();
        assert!(result.is_err());
    }

    #[test]
    fn test_format_result_integer() {
        assert_eq!(format_result(42.0), "42");
        assert_eq!(format_result(-100.0), "-100");
    }

    #[test]
    fn test_format_result_decimal() {
        assert_eq!(format_result(3.14), "3.14");
        assert_eq!(format_result(2.5), "2.5");
    }

    #[test]
    fn test_implicit_multiplication() {
        let mut calc = Calculator::default();
        calc.input_digit("2");
        calc.input_constant("π");
        let result = calc.calculate().unwrap();
        assert!((result - 2.0 * std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_nested_parentheses() {
        let mut calc = Calculator::default();
        calc.input_open_paren();
        calc.input_open_paren();
        calc.input_digit("2");
        calc.input_operator("+");
        calc.input_digit("3");
        calc.input_close_paren();
        calc.input_operator("×");
        calc.input_digit("2");
        calc.input_close_paren();
        let result = calc.calculate().unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_exp_function() {
        let mut calc = Calculator::default();
        calc.expression = "exp(0)".to_string();
        calc.display = calc.expression.clone();
        let result = calc.calculate().unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln_function() {
        let mut calc = Calculator::default();
        calc.input_function("ln");
        calc.input_constant("e");
        calc.input_close_paren();
        let result = calc.calculate().unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }
}
