use rand::seq::SliceRandom;

pub fn main(){

    /*
     * Initial starting deck of a vector with 52 unsigned integers 
    */
    let mut init_deck = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52];

    let mut deck = play(&mut init_deck); //deck gets shuffled
    let deck_output = &deck[..];
    println!("Deck without permutations: {:?}", deck_output); //shows the deck without permutations
    
    println!("");

    let vec_deck = with_permutations(&mut deck); //pass on array of numbers to be converted to an array of cards

    /*
     * split with permutaitons deck into north, south, east west 
    */
    let (per_north, per_temp1) = vec_deck.split_at(13);

    let (per_south, per_temp2) = per_temp1.split_at(13);

    let (per_east, per_west) = per_temp2.split_at(13);

    println!("");

    println!("Deck with permutations: {:?}", vec_deck); //Prints out deck with permutations
    println!("North: {:?}", per_north); //Prints out North's hand before formatting
    println!("South: {:?}", per_south); //Prints out South's hand before formatting
    println!("East: {:?}", per_east); //Prints out East's hand before formatting
    println!("West: {:?}", per_west); //Prints out West's hand before formatting

    /*
     * Counts the points for each players based on their cards 
    */
    let north_sum: i32 = count_points(per_north);
    let south_sum: i32 = count_points(per_south);
    let east_sum: i32 = count_points(per_east);
    let west_sum: i32 = count_points(per_west);
    let declarer: &str = compare_sums(north_sum, south_sum, east_sum, west_sum); //find the declarer
    
    println!("");

    format_game(&mut per_north.to_vec(), &mut per_south.to_vec(), &mut per_east.to_vec(), &mut per_west.to_vec()); //takes in players and formats the game

    println!("");
    println!("North's points: {}", north_sum); //prints out the North's total points
    println!("South's points: {}", south_sum); //prints out the South's total points
    println!("East's points: {}", east_sum); //prints out the East's total points
    println!("West's points: {}", west_sum); //prints out the West's total points
    println!("");
    println!("Declarer: {}", declarer); //prints out the declarer based on the summed points of each player

}

/*
 * This function is used to format the current state of the board 
 */
pub fn format_game(north: &mut [&str], south: &mut [&str], east: &mut [&str], west: &mut [&str]){
    /*
     * Sorts hand of the north, south, east, and west based on their suits 
    */

    let north_c = sort_clubs(north);
    let north_d = sort_diamonds(north);
    let north_h = sort_hearts(north);
    let north_s = sort_spades(north);

    let west_c = sort_clubs(west);
    let west_d = sort_diamonds(west);
    let west_h = sort_hearts(west);
    let west_s = sort_spades(west);

    let east_c = sort_clubs(east);
    let east_d = sort_diamonds(east);
    let east_h = sort_hearts(east);
    let east_s = sort_spades(east);

    let south_c = sort_clubs(south);
    let south_d = sort_diamonds(south);
    let south_h = sort_hearts(south);
    let south_s = sort_spades(south);

    /*
     * Formatting of the game 
    */
    println!("              {0:}", "North: ");
    println!("              {0:}", north_c);
    println!("              {0:}", north_d);
    println!("              {0:}", north_h);
    println!("              {0:}", north_s);

    print!("{0: <30}", "West: "); print!("{0: <10}", "East: "); println!("");
    print!("{0: <30}", west_c); print!("{0: <10}", east_c); println!("");
    print!("{0: <30}", west_d); print!("{0: <10}", east_d); println!("");
    print!("{0: <30}", west_h); print!("{0: <10}", east_h); println!("");
    print!("{0: <30}", west_s); print!("{0: <10}", east_s); println!("");

    println!("              {0:}", "South: ");
    println!("              {0:}", south_c);
    println!("              {0:}", south_d);
    println!("              {0:}", south_h);
    println!("              {0:}", south_s);
   
}

/*
 * Sorts the clubs in the player's hand 
*/
pub fn sort_clubs(clubs: &mut [&str]) -> String{
    let mut vec_clubs = vec!["C"];
    let mut vec_letters_c: Vec<&str> = Vec::new();
    let mut last1 = 0;
    let mut i = 0;
    let mut club_str = String::from("C");

    /*
     * if clubs is not a letter saves to vec_clubs, 
     * else if clubs is a letter save it to vec_letters_c
     */
    for club in clubs{
        if club.contains("C") && !(club.contains("CA") || club.contains("CK") || club.contains("CQ") || club.contains("CJ")){
            vec_clubs.push(club);
        }
        else if club.contains("C") && (club.contains("CA") || club.contains("CK") || club.contains("CQ") || club.contains("CJ")){
            vec_letters_c.push(club);
        }
    }

    vec_clubs.sort();

    let mut last1 = 0;

    /*
     * Push letter cards to the head of vec_clubs 
     */
    while last1 < vec_letters_c.len(){
        if vec_letters_c[last1].contains("A"){
            vec_clubs.insert(1, vec_letters_c[last1]);
        }
        if vec_letters_c[last1].contains("K"){
            vec_clubs.insert(1, vec_letters_c[last1]);
        }
        if vec_letters_c[last1].contains("Q"){
            vec_clubs.insert(1, vec_letters_c[last1]);
        }
        if vec_letters_c[last1].contains("J"){
            vec_clubs.insert(1, vec_letters_c[last1]);
        }
        else{
            break;
        }
        last1 += 1;
    }
    
    /*
     * Convert vec_clubs to string for output 
     */
    while i < vec_clubs.len(){

        let (_, face) = vec_clubs[i].split_at(1);
        club_str.push_str(&face);
        club_str.push_str(" ");

        i += 1;
    }
    club_str
}

/*
 * Sorts the diamonds in the player's hand 
*/
pub fn sort_diamonds(diamonds: &mut [&str]) -> String{
    let mut vec_diamonds = vec!["D"];
    let mut vec_letters_d: Vec<&str> = Vec::new();
    let mut last2 = 0;
    let mut i = 0;
    let mut diamond_str = String::from("D");

    /*
     * if clubs is not a letter saves to vec_diamonds, 
     * else if clubs is a letter save it to vec_letters_d
     */
    for diamond in diamonds{
        if diamond.contains("D") && !(diamond.contains("DA") || diamond.contains("DK") || diamond.contains("DQ") || diamond.contains("DJ")){
            vec_diamonds.push(diamond);
        }
        else if diamond.contains("D") && (diamond.contains("DA") || diamond.contains("DK") || diamond.contains("DQ") || diamond.contains("DJ")){
            vec_letters_d.push(diamond);
        }
    }

    vec_diamonds.sort();

    let mut last2 = 0;

     /*
     * Push letter cards to the head of vec_diamonds 
     */
    while last2 < vec_letters_d.len(){
        if vec_letters_d[last2].contains("A"){
            vec_diamonds.insert(1, vec_letters_d[last2]);
        }
        if vec_letters_d[last2].contains("K"){
            vec_diamonds.insert(1, vec_letters_d[last2]);
        }
        if vec_letters_d[last2].contains("Q"){
            vec_diamonds.insert(1, vec_letters_d[last2]);
        }
        if vec_letters_d[last2].contains("J"){
            vec_diamonds.insert(1, vec_letters_d[last2]);
        }
        else{
            break;
        }
        last2 += 1;
    }
    
    /*
     * Convert vec_diamonds to string for output 
     */
    while i < vec_diamonds.len(){

        let (_, face) = vec_diamonds[i].split_at(1);
        diamond_str.push_str(&face);
        diamond_str.push_str(" ");

        i += 1;
    }
    diamond_str
}

/*
 * Sorts the hearts in the player's hand 
*/
pub fn sort_hearts(hearts: &mut [&str]) -> String{
    let mut vec_hearts = vec!["H"];
    let mut vec_letters_h: Vec<&str> = Vec::new();
    let mut last3 = 0;
    let mut i = 0;
    let mut hearts_str = String::from("H");

    /*
     * if clubs is not a letter saves to vec_hearts, 
     * else if clubs is a letter save it to vec_letters_h
     */
    for hearts in hearts{
        if hearts.contains("H") && !(hearts.contains("HA") || hearts.contains("HK") || hearts.contains("HQ") || hearts.contains("HJ")){
            vec_hearts.push(hearts);
        }
        else if hearts.contains("H") && (hearts.contains("HA") || hearts.contains("HK") || hearts.contains("HQ") || hearts.contains("HJ")){
            vec_letters_h.push(hearts);
        }
    }

    vec_hearts.sort();

    let mut last3 = 0;

     /*
     * Push letter cards to the head of vec_hearts 
     */
    while last3 < vec_letters_h.len(){
        if vec_letters_h[last3].contains("A"){
            vec_hearts.insert(1, vec_letters_h[last3]);
        }
        if vec_letters_h[last3].contains("K"){
            vec_hearts.insert(1, vec_letters_h[last3]);
        }
        if vec_letters_h[last3].contains("Q"){
            vec_hearts.insert(1, vec_letters_h[last3]);
        }
        if vec_letters_h[last3].contains("J"){
            vec_hearts.insert(1, vec_letters_h[last3]);
        }
        else{
            break;
        }
        last3 += 1;
    }
    
    /*
     * Convert vec_hearts to string for output 
     */
    while i < vec_hearts.len(){

        let (_, face) = vec_hearts[i].split_at(1);
        hearts_str.push_str(&face);
        hearts_str.push_str(" ");

        i += 1;
    }
    hearts_str
}

/*
 * Sorts the spades in the player's hand 
*/
pub fn sort_spades(spades: &mut [&str]) -> String{
    let mut vec_spades = vec!["S"];
    let mut vec_letters_s: Vec<&str> = Vec::new();
    let mut last4 = 0;
    let mut i = 0;
    let mut spades_str = String::from("S");

    /*
     * if clubs is not a letter saves to vec_spades, 
     * else if clubs is a letter save it to vec_letters_s
     */
    for spades in spades{
        if spades.contains("S") && !(spades.contains("SA") || spades.contains("SK") || spades.contains("SQ") || spades.contains("SJ")){
            vec_spades.push(spades);
        }
        else if spades.contains("S") && (spades.contains("SA") || spades.contains("SK") || spades.contains("SQ") || spades.contains("SJ")){
            vec_letters_s.push(spades);
        }
    }

    vec_spades.sort();

    let mut last4 = 0;

     /*
     * Push letter cards to the head of vec_spades 
     */
    while last4 < vec_letters_s.len(){
        if vec_letters_s[last4].contains("A"){
            vec_spades.insert(1, vec_letters_s[last4]);
        }
        if vec_letters_s[last4].contains("K"){
            vec_spades.insert(1, vec_letters_s[last4]);
        }
        if vec_letters_s[last4].contains("Q"){
            vec_spades.insert(1, vec_letters_s[last4]);
        }
        if vec_letters_s[last4].contains("J"){
            vec_spades.insert(1, vec_letters_s[last4]);
        }
        else{
            break;
        }
        last4 += 1;
    }
    
    /*
     * Convert vec_spades to string for output 
     */
    while i < vec_spades.len(){

        let (_, face) = vec_spades[i].split_at(1);
        spades_str.push_str(&face);
        spades_str.push_str(" ");

        i += 1;
    }
    spades_str
}

/*
 * Compares the total sum (points) of each player to determine the declarer 
*/
pub fn compare_sums<'a>(north: i32, south: i32, east: i32, west: i32) -> &'a str{
    if north > south && north > east && north > west{
        "North"
    }
    else if south > north && south > east && south > west{
        "South"
    }
    else if east > north && east > south && east > west{
        "East"
    }
    else{
        "West"
    }
}

/*
* counts how many points each player has
*/
pub fn count_points(player: &[&str]) -> i32 {
    let mut sum: i32 = 0;
    let mut count_c: i32 = 0;
    let mut count_d: i32 = 0;
    let mut count_h: i32 = 0;
    let mut count_s: i32 = 0;
    
    /*
     * Count the number of cards in each suit
     */
    for elem in player.iter(){
        if elem.contains("C"){
            count_c += 1;
        }
        if elem.contains("D"){
            count_d += 1;
        }
        if elem.contains("H"){
            count_h += 1;
        }
        if elem.contains("S"){
            count_s += 1;
        }
    }
    
    /*
     * Point accumulation happens in the for loop 
     */
    for elem2 in player.iter(){
        if elem2.contains("C"){
            if elem2.contains("A"){
                if count_c == 1{
                    sum += 6;
                }
                else if count_c == 2{
                    sum += 5;
                }
                else{
                    sum += 4;
                }
            }

            if elem2.contains("K"){
                if count_c == 1{
                    sum += 5;
                }
                else if count_c == 2{
                    sum += 4;
                }
                else{
                    sum += 3;
                }
            }

            if elem2.contains("Q"){
                if count_c == 1{
                    sum += 4;
                }
                else if count_c == 2 {
                    sum += 3;
                }
                else{
                    sum += 2;
                }
            }

            if elem2.contains("J"){
                if count_c == 1{
                    sum += 3;
                }
                else if count_c == 2{
                    sum += 2;
                }
                else{
                    sum += 1;
                }
            }
            if !(elem2.contains("A") || elem2.contains("K") || elem2.contains("Q") || elem2.contains("J")) && (count_c == 1){
                sum += 2;
            }

            if !(elem2.contains("A") || elem2.contains("K") || elem2.contains("Q") || elem2.contains("J")) && (count_c == 2){
                sum += 1;
            }
        }
        else if !(elem2.contains("C")) && count_c == 0{
            sum += 3;
        }

        if elem2.contains("D"){
            if elem2.contains("A"){
                if count_d == 1{
                    sum += 6;
                }
                else if count_d == 2{
                    sum += 5;
                }
                else{
                    sum += 4;
                }
            }

            if elem2.contains("K"){
                if count_d == 1{
                    sum += 5;
                }
                else if count_d == 2{
                    sum += 4;
                }
                else{
                    sum += 3;
                }
            }

            if elem2.contains("Q"){
                if count_d == 1{
                    sum += 4;
                }
                else if count_d == 2 {
                    sum += 3;
                }
                else{
                    sum += 2;
                }
            }

            if elem2.contains("J"){
                if count_d == 1{
                    sum += 3;
                }
                else if count_d == 2{
                    sum += 2;
                }
                else{
                    sum += 1;
                }
            }

            if !(elem2.contains("A") || elem2.contains("K") || elem2.contains("Q") || elem2.contains("J")) && (count_d == 1){
                sum += 2;
            }

            if !(elem2.contains("A") || elem2.contains("K") || elem2.contains("Q") || elem2.contains("J")) && (count_d == 2){
                sum += 1;
            }
        }
        else if !(elem2.contains("D")) && count_d == 0{
            sum += 3;
        }

        if elem2.contains("H"){
            if elem2.contains("A"){
                if count_h == 1{
                    sum += 6;
                }
                else if count_h == 2{
                    sum += 5;
                }
                else{
                    sum += 4;
                }
            }

            if elem2.contains("K"){
                if count_h == 1{
                    sum += 5;
                }
                else if count_h == 2{
                    sum += 4;
                }
                else{
                    sum += 3;
                }
            }

            if elem2.contains("Q"){
                if count_h == 1{
                    sum += 4;
                }
                else if count_h == 2 {
                    sum += 3;
                }
                else{
                    sum += 2;
                }
            }

            if elem2.contains("J"){
                if count_h == 1{
                    sum += 3;
                }
                else if count_h == 2{
                    sum += 2;
                }
                else{
                    sum += 1;
                }
            }

            if !(elem2.contains("A") || elem2.contains("K") || elem2.contains("Q") || elem2.contains("J")) && (count_h == 1){
                sum += 2;
            }

            if !(elem2.contains("A") || elem2.contains("K") || elem2.contains("Q") || elem2.contains("J")) && (count_h == 2){
                sum += 1;
            }
        }
        else if !(elem2.contains("H")) && count_h == 0{
            sum += 3;
        }

        if elem2.contains("S"){
            if elem2.contains("A"){
                if count_s == 1{
                    sum += 6;
                }
                else if count_s == 2{
                    sum += 5;
                }
                else{
                    sum += 4;
                }
            }

            if elem2.contains("K"){
                if count_s == 1{
                    sum += 5;
                }
                else if count_s == 2{
                    sum += 4;
                }
                else{
                    sum += 3;
                }
            }

            if elem2.contains("Q"){
                if count_s == 1{
                    sum += 4;
                }
                else if count_s == 2 {
                    sum += 3;
                }
                else{
                    sum += 2;
                }
            }

            if elem2.contains("J"){
                if count_s == 1{
                    sum += 3;
                }
                else if count_s == 2{
                    sum += 2;
                }
                else{
                    sum += 1;
                }
            }
            else if !(elem2.contains("S") && count_s == 0){
                sum += 3;
            }

            if !(elem2.contains("A") || elem2.contains("K") || elem2.contains("Q") || elem2.contains("J")) && (count_s == 1){
                sum += 2;
            }

            if !(elem2.contains("A") || elem2.contains("K") || elem2.contains("Q") || elem2.contains("J")) && (count_s == 2){
                sum += 1;
            }
        }
    }
    sum
}

/*
* Converts an unsigned interger vector into a vector of cards
*/
pub fn with_permutations<'a>(arr: &mut [u32]) -> Vec<&'a str>{
	let mut vec2: Vec<&str> = Vec::new();
    for x in arr.iter(){
		match x{
			1 => vec2.push("C2"),
            2 => vec2.push("C3"),
            3 => vec2.push("C4"),
            4 => vec2.push("C5"),
            5 => vec2.push("C6"),
            6 => vec2.push("C7"),
            7 => vec2.push("C8"),
            8 => vec2.push("C9"),
            9 => vec2.push("C10"),
            10 => vec2.push("CJ"),
            11 => vec2.push("CQ"),
            12 => vec2.push("CK"),
            13 => vec2.push("CA"),
            14 => vec2.push("D2"),
            15 => vec2.push("D3"),
            16 => vec2.push("D4"),
            17 => vec2.push("D5"),
            18 => vec2.push("D6"),
            19 => vec2.push("D7"),
            20 => vec2.push("D8"),
            21 => vec2.push("D9"),
            22 => vec2.push("D10"),
            23 => vec2.push("DJ"),
            24 => vec2.push("DQ"),
            25 => vec2.push("DK"),
            26 => vec2.push("DA"),
            27 => vec2.push("H2"),
            28 => vec2.push("H3"),
            29 => vec2.push("H4"),
            30 => vec2.push("H5"),
            31 => vec2.push("H6"),
            32 => vec2.push("H7"),
            33 => vec2.push("H8"),
            34 => vec2.push("H9"),
            35 => vec2.push("H10"),
            36 => vec2.push("HJ"),
            37 => vec2.push("HQ"),
            38 => vec2.push("HK"),
            39 => vec2.push("HA"),
            40 => vec2.push("S2"),
            41 => vec2.push("S3"),
            42 => vec2.push("S4"),
            43 => vec2.push("S5"),
            44 => vec2.push("S6"),
            45 => vec2.push("S7"),
            46 => vec2.push("S8"),
            47 => vec2.push("S9"),
            48 => vec2.push("S10"),
            49 => vec2.push("SJ"),
            50 => vec2.push("SQ"),
            51 => vec2.push("SK"),
            52 => vec2.push("SA"),
            _ => println!("{:?}", "Error")
		};
	}
    vec2
}

/*
* Shuffles the deck using Fisher-Yates algorithm
*/
pub fn play(arr: &mut [u32]) -> &mut [u32]{
    let mut i = arr.len() - 1;
    while i != 0{
        let testing = arr.choose(&mut rand::thread_rng());
        let rand_number: u32 = *testing.unwrap(); //unwrap unsigned int
        let rand_index = arr.iter().position(|&i| i == rand_number); //finds the index of the random number
        let rand = rand_index.unwrap(); //unwrap that index
        let temp = arr[rand as usize];
        arr[rand as usize] = arr[i as usize];
        arr[i as usize] = temp;

        i-= 1;
    }
    arr
}