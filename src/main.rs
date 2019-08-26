use regex::{Regex, Captures};

fn replace_half_width_number(string: &str) -> String {
    let re = Regex::new(r"(\d+(.\d+)?)").unwrap();
    re.replace_all(string, |caps: &Captures| {
        let num: f64 = (&caps[0]).parse().unwrap();
        format!("{}", num * 100.0)
    }).to_string()
}

fn replace_full_width_number(string: &str) -> String {
    let re = Regex::new(r"(?P<number>[０-９]+)").unwrap();
    re.replace_all(string, |caps: &Captures| {
        format!("{}００", &caps[0])
    }).to_string()
}

fn replace(string: &str) -> String {
    replace_full_width_number(&replace_half_width_number(string))
}

fn main() {
    let after = replace("aaa20bbb300");
    println!("{}", after);
}
