pub fn verse(n: u32) -> String {
    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, {} bottles of beer on the wall.\n", 99),
        1 => format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", 1, 1),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", 2, 2, 1),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1),
    }
}


pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();

    for i in (end..=start).rev() {
	song.push_str(&verse(i));
        if i != end {
            song.push_str("\n");
        }
    }
 
   song
}


