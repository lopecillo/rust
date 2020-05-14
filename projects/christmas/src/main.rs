fn nthday_lyrics(n:usize) -> String {
    let lyrics = ["And a partridge in a pear tree",
                  "Two turtle doves",
                  "Three French hens",
                  "Four calling birds",
                  "Five gold rings",
                  "Six geese a laying",
                  "Seven swans a swimming",
                  "Eight maids a milking",
                  "Nine ladies dancing",
                  "Ten lords a leaping",
                  "Eleven pipers piping",
                  "12 drummers drumming"];
    let ordinal = ["first",
                   "second",
                   "third",
                   "fourth",
                   "fifth",
                   "sixth",
                   "seventh",
                   "eigth",
                   "ninth",
                   "tenth",
                   "eleventh",
                   "twelfth"];
    let mut full_verse = String::new();
    full_verse.push_str(&format!("On the {} day of Christmas\n", ordinal[n-1]));
    full_verse.push_str("My true love gave to me\n");

    if n == 1 {
        full_verse.push_str("A partridge in a pear tree\n");
    }
    else {
        for _i in (0..n).rev() {
            full_verse.push_str(&format!("{}\n", lyrics[_i]));
        }
    }
    full_verse // Return value
}

fn twelvedaysofchristmas() -> String {
    let mut full_song = String::new();
    for _i in 1..12 {
        full_song.push_str(&format!("{}\n", nthday_lyrics(_i)));
    }
    full_song // Return value
}

fn main() {
    print!("{}", twelvedaysofchristmas());
}
