/* 
    Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
    taking advantage of the repetition in the song.
*/

fn main() {
    let lines = ["A partridge in a pear tree",
                 "Two turtle doves, and",
                 "Three french hens",
                 "Four calling birds",
                 "Five golden rings",
                 "Six geese a-laying",
                 "Seven swans a-swimming",
                 "Eight maids a-milking",
                 "Nine ladies dancing",
                 "Ten lords a-leaping",
                 "Eleven pipers piping",
                 "Twelve drummers drumming"];

    let days = ["first",
                "second",
                "thrid",
                "fourth",
                "fifth",
                "sixth",
                "seventh",
                "eighth",
                "ninth",
                "tenth",
                "eleventh",
                "twelfth"];

    let mut count = 0;
    
    for day in days {
        println!("On the {} day of Christmas, my true love sent to me", day);

        count += 1;

        for i in (0..count).rev() {
            println!("{}", lines[i]);
        }

        println!("");
    }
}