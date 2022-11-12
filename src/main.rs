use std::env;
use webbrowser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_iter = args[1..].iter();

    for url in args_iter {
        if webbrowser::open(url).is_err() {
            println!("'{}' is not url", url);
        }
    }
}
