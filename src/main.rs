fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelveth",
    ];
    let presents = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let mut index = 0;
    while index < days.len() {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[index]
        );
        for i in (0..(index + 1)).rev() {
            if i == 0 && index > 0 {
                println!("And {}", presents[i].to_lowercase());
            } else {
                println!("{}", presents[i]);
            }
        }
        println!("");
        index += 1;
    }
}
