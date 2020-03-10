use ferris_says::say;
use std::io::BufWriter;
use std::io::stdout;

fn main() {
    let stdout = stdout();
    let out = b"Hello, Mario!";
    let width = 24;
    let mut writer = BufWriter::new(stdout.lock());
    let result = say(out, width, &mut writer);
    if result.is_ok() {
        println!("it's OK!");
    } else {
        println!("not so much");
    }
}
