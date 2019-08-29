use regex::Regex;

fn is_contain(title: &str) -> bool {
    // slow!!
    let re = Regex::new(r"[\d|０-９|十|百|千|万|億]").unwrap();
    re.is_match(title)
}

fn main() {
    let mut title = String::new();
    loop {
        match std::io::stdin().read_line(&mut title) {
            Ok(_) => {
                if is_contain(&title) {
                    print!("{}", title);
                }
            },
            Err(err) => {
                // println!("{:?}", err);
                break;
            }
        }
        title.clear();
    }
}