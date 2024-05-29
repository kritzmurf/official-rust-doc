fn main() {
    verse_printer();
}

fn verse_printer() {
    const NUM_DAYS: i32 = 12;

    const DAY_ARRAY: [&str; 12] = [ "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
                                 "tenth", "eleventh", "twelvth"];

    const GIFT_ARRAY: [&str; 12] = [  "a partridge in a pear tree.",
                                        "two turtle doves,",
                                        "three French hens,",
                                        "four calling birds,",
                                        "five gold rings,",
                                        "six geese a-laying,",
                                        "seven swans a-swimming,",
                                        "eight maids a-milking,",
                                        "nine ladies dancing,",
                                        "ten lords a-leaping,",
                                        "eleven pipers piping,",
                                        "twelve drummers drumming,"
                                    ];

    let mut counter: i32 = 0;
    while counter < NUM_DAYS 
    {
        let day = DAY_ARRAY[counter as usize];
        println!("   On the {day} day of Christmas, my true love gave to me");
        gift_loop(GIFT_ARRAY, counter);
        println!("");
        counter += 1;
    }
}

fn gift_loop(gifts: [&str; 12], index: i32) {
    let mut counter: i32 = index;
    let mut use_and = false;
    if counter > 0 {
        use_and = true;
    }

    while counter > -1 && counter < (gifts.len() as i32) {
        let gift = gifts[counter as usize];

        if use_and && counter == 0 {
            println!("   and {gift}");
        } else {
            println!("   {gift}");
        }
        counter -= 1;
    }
}
