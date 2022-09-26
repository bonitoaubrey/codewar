pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut result = Vec::new();
    for (years, handicap) in data {
        if years >= 55 && handicap > 7 {
            result.push(String::from("Senior"));
        } else {
            result.push(String::from("Open"));
        }
    }
    result
}
