const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtledoves",
    "three french hens",
    "four calling birds",
    "five golden rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];


fn main() {
    for i in 1..13 {
        println!("{}", gen_day_lyrics(i))
    }
}

fn gen_day_lyrics(n: usize) -> String {
    let day = if n == 1 {
        String::from("1st")
    } else if n == 2 {
        String::from("2nd")
    } else if n == 3 {
        String::from("3rd")
    } else {
        format!("{}th", n)
    };

    let gifts = if n == 1 {
        String::from(GIFTS[0])
    } else {
        let mut gifts = format!("and {}", GIFTS[0]);

        for i in 1..n {
            gifts = format!("{}, {}", GIFTS[i], gifts)
        }
        gifts
    };

    format!("On the {} day of Christmas, my true love gave to me {}", day, gifts)
}
