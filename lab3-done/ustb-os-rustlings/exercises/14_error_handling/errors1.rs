// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.



/*
pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        None
    } else {
        Some(format!("Hi! My name is {}", name))
    }
}
*/

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 2. 失败时返回 Err，并带上测试用例要求的错误信息
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        // 3. 成功时返回 Ok，包裹住生成的字符串
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
