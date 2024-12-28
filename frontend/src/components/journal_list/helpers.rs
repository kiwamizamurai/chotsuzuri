pub fn format_currency(amount: i32) -> String {
    let abs_amount = amount.abs();
    let formatted = abs_amount.to_string()
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();

    if amount < 0 {
        format!("-{} 円", formatted)
    } else {
        format!("{} 円", formatted)
    }
}