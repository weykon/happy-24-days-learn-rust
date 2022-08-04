use docopt::Docopt;

static USAGE: &'static str = "
Usage: wc [options] [<file>]

Options:
    -c, --bytes  print the byte counts
    -m, --chars  print the character counts
    -l, --lines  print the newline counts
    -w, --words  print the word counts
    -L, --max-line-length  print the length of the longest line
    -h, --help  display this help and exit
    -v, --version  output version information and exit
";

struct Args {
    arg_file: Option<String>,
    flag_bytes: bool,
    flag_chars: bool,
    flag_lines: bool,
    flag_words: bool,
    flag_max_line_length: bool,
}

pub fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);

    println!("Counting stuff in {}", args.get_str("arg_file"));
    if args.get_bool("flag_bytes") {
        println!("Counting bytes!");
    }
    if args.get_bool("flag_chars") {
        println!("Counting characters!");
    }
}
