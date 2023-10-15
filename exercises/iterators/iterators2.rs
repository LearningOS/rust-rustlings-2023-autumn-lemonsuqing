// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        /*
        `to_uppercase()` 是一个字符串方法，它将字符串中的所有字符转换为大写形式。在这个例子中，我们使用它来将第一个字符转换为大写形式。
        `collect()` 是一个通用方法，它将迭代器中的元素收集到一个新的数据结构中。在这个例子中，我们使用 `collect()` 将字符迭代器转换为字符串。
        `::<String>` 语法是 Rust 中的类型注释。它告诉 Rust 将 `collect()` 的结果转换为一个字符串类型。
        `+` 运算符用于连接两个字符串。在这个例子中，我们使用它将第一个字符和剩余的字符串连接起来。
        `as_str()` 是一个字符串方法，它返回一个字符串切片，其中包含迭代器中剩余的字符。在这个例子中，我们使用它来获取剩余的字符串。
        */
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
    /*
    我们使用 iter() 方法来创建一个字符串切片的迭代器。然后，
    我们使用 map() 方法将 capitalize_first() 函数应用于每个元素。
    最后，我们使用 collect() 方法将结果收集到一个新的向量中。
    */
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    capitalize_words_vector(words).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
