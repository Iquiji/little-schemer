pub fn split_whitespace_not_in_parantheses(input: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut current_substring = String::new();

    let mut paranthesis_depth = 0;

    for current_char in input.chars() {
        // println!(
        //     "char: '{}',depth: {:?},result: {:?}",
        //     current_char, paranthesis_depth, result
        // );

        if current_char == '(' {
            paranthesis_depth += 1;
            current_substring.push(current_char);
        } else if current_char == ' ' && paranthesis_depth == 0 {
            if !current_substring.is_empty() {
                result.push(current_substring);
            }
            current_substring = String::new();
        } else if current_char == ')' {
            paranthesis_depth -= 1;
            current_substring.push(current_char);
        } else {
            current_substring.push(current_char);
        }
    }

    result.push(current_substring);

    //println!("result: {:?}", result);

    if paranthesis_depth != 0 {
        panic!("Parantheses not balanced!")
    }

    result
}

/// 'x -> (quote x) / '(a b c) -> (quote (a b c))
pub fn split_whitespace_not_in_parantheses_advanced_to_quote(input: &str) -> Vec<String> {
    let input = input.replace("\n", " ").replace("  ", " ");

    let mut result: Vec<String> = vec![];
    let mut current_substring = String::new();

    let mut paranthesis_depth = 0;
    let mut quote_stack: Vec<i32> = vec![];

    for current_char in input.chars() {
        // println!(
        //     "depth: {:?},char: {:?},stack: {:?}",
        //     paranthesis_depth, current_char, quote_stack
        // );
        if current_char == '\'' {
            quote_stack.push(paranthesis_depth);
            current_substring += "(quote ";
        } else if current_char == '(' {
            paranthesis_depth += 1;
            current_substring.push(current_char);
        } else if current_char == ' ' {
            if !quote_stack.is_empty() && quote_stack[quote_stack.len() - 1] == paranthesis_depth {
                current_substring.push(')');
                quote_stack.pop();
            }
            if paranthesis_depth == 0 {
                if !current_substring.is_empty() {
                    for _ in 0..quote_stack.len() {
                        current_substring.push(')');
                        quote_stack.pop();
                    }
                    result.push(current_substring);
                }
                current_substring = String::new();
            } else {
                current_substring.push(current_char);
            }
        } else if current_char == ')' {
            if !quote_stack.is_empty() && quote_stack[quote_stack.len() - 1] == paranthesis_depth {
                current_substring.push(')');
                quote_stack.pop();
            }
            paranthesis_depth -= 1;
            current_substring.push(current_char);
        } else {
            current_substring.push(current_char);
        }
        // println!(
        //     "depth: {:?},char: {:?},stack: {:?}",
        //     paranthesis_depth, current_char, quote_stack
        // );
    }
    for _ in 0..quote_stack.len() {
        current_substring.push(')');
        quote_stack.pop();
    }
    result.push(current_substring);

    //println!("result: {:?}", result);

    if paranthesis_depth != 0 {
        panic!("Parantheses not balanced!")
    }

    result
}
