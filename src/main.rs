use rand::Rng;
use std::io;

fn main() {
    // Original variable names
    let mut total_dead_people: u32 = 0; // D1
    let mut percent_of_starved_people_per_year: u32 = 0; // P1
    let mut current_year: u32 = 0; // Z
    let mut population: u32 = 95; // P
    let mut grain_in_store: u32 = 2800; // S
    let mut harvest: u32 = 3000; // H
    let mut grain_lost_to_rats: u32 = harvest - grain_in_store; // E
    let mut harvested_bushels_per_acre: u32 = 3; // Y
    let mut land_price: u32; // Y, reused in original code
    let mut acres_owned: u32 = 1000; // A
    let mut people_came_to_city: u32 = 5; // I
    let mut bushels_to_feed_your_people: u32 = 0; // Q, reused in original code
    let mut people_starved: u32 = 0; // D
    let mut acres_to_plant_with_seed: u32 = 0; // D, reused in original code
    let mut people_with_full_tummies: u32; // C, reused in original code
    let mut acres_per_person: u32 = 0; // L

    let mut trade_amount: u32;
    let mut skip_selling_land: bool = true;

    let mut bool_endofterm_ending: bool = false;
    let mut bool_overreactionmuch_ending = false;

    println!("HAMURABI RUST by White Mouse (2024)");
    println!("Based on original game: Hamurabi, by David H. Ahl (1978)");
    println!("Ported to Rust programming language");
    println!("");
    println!("");
    println!("TRY YOUR HAND AT GOVERNING ANCIENT SUMERIA");
    println!("FOR A TEN-YEAR TERM OF OFFICE.");
    println!("");

    fn get_land_price() -> u32 {
        let land_price: u32 = rand::thread_rng().gen_range(17..=26);
        return land_price;
    }

    fn get_hasplaguehappened(p_current_year: &u32) -> bool {
        let mut plague: bool = false;
        if *p_current_year > 1 && rand::thread_rng().gen_range(1..=100) < 16 {
            plague = true;
        }
        return plague;
    }

    fn get_hasratshappened() -> bool {
        if rand::thread_rng().gen_range(1..=5) < 3 {
            return true;
        }

        return false;
    }
    fn startofyearreport(
        p_current_year: &u32,
        p_people_starved: &u32,
        p_people_came_to_city: &u32,
        p_population: &mut u32,
        p_acres_owned: &u32,
        p_harvested_bushels_per_acre: &u32,
        p_grainlostotrats: &u32,
        p_grain_in_store: &u32,
    ) {
        println!("HAMURABI:  I BEG TO REPORT TO YOU, IN YEAR {}, {} PEOPLE STARVED, {} CAME TO THE CITY,", p_current_year, p_people_starved, p_people_came_to_city);
        *p_population += p_people_came_to_city;
        if get_hasplaguehappened(p_current_year) {
            *p_population /= 2;
            println!("A HORRIBLE PLAGUE STRUCK!  HALF THE PEOPLE DIED.")
        }
        println!("POPULATION IS NOW {}", p_population);
        println!("THE CITY NOW OWNS {} ACRES.", p_acres_owned);
        println!(
            "YOU HARVESTED {} BUSHELS PER ACRE.",
            p_harvested_bushels_per_acre
        );
        println!("RATS ATE {} BUSHELS.", p_grainlostotrats);
        println!("YOU NOW HAVE {} BUSHELS IN STORE.", p_grain_in_store);
    }

    fn endofterm(
        p_people_starvedperyear: &mut u32,
        p_total_dead_people: &mut u32,
        p_acres_owned: &mut u32,
        p_acres_per_person: &mut u32,
        p_population: &mut u32,
    ) {
        println!(
            "IN YOUR 10-YEAR TERM OF OFFICE, {} PERCENT OF THE",
            p_people_starvedperyear
        );
        println!("POPULATION STARVED PER YEAR ON THE AVERAGE, I.E. A TOTAL OF");
        println!("{}PEOPLE DIED!!", p_total_dead_people);
        *p_acres_per_person = *p_acres_owned / *p_population;
        println!("YOU STARTED WITH 10 ACRES PER PERSON AND ENDED WITH");
        println!("{} ACRES PER PERSON.", p_acres_per_person);
    }

    fn overreactionmuch() {

        println!("HAMURABI:  I CANNOT DO WHAT YOU WISH.");
        println!("GET YOURSELF ANOTHER STEWARD!!!!!");

    }

    fn notenoughgrain(p_grain_in_store: &u32) {
        println!("HAMURABI:  THINK AGAIN.  YOU HAVE ONLY {} BUSHELS OF GRAIN.  NOW THEN,", p_grain_in_store);
    }

    fn notenoughland(p_acres_owned: &u32) {

        println!("HAMURABI:  THINK AGAIN.  YOU OWN ONLY {} ACRES.  NOW THEN,", p_acres_owned);

    }

    fn notenoughpeopletowork(p_population: &u32) {

        println!("BUT YOU HAVE ONLY {} PEOPLE TO TEND THE FIELDS!  NOW THEN,", p_population);

    }

    fn impeached() {
        println!("DUE TO THIS EXTREME MISMANAGEMENT YOU HAVE NOT ONLY");
        println!("BEEN IMPEACHED AND THROWN OUT OF OFFICE BUT YOU HAVE");
        println!("ALSO BEEN DECLARED NATIONAL FINK!!!!");
    }

    fn score_nero() {
        println!("YOUR HEAVY-HANDED PERFORMANCE SMACKS OF NERO AND IVAN IV.");
        println!("THE PEOPLE (REMIANING) FIND YOU AN UNPLEASANT RULER, AND,");
        println!("FRANKLY, HATE YOUR GUTS!!");
    }

    fn score_mediocre(p_population: &mut u32) {
        println!("YOUR PERFORMANCE COULD HAVE BEEN SOMEWHAT BETTER, BUT");
        let haters: u32 = (*p_population * 8 * rand::thread_rng().gen_range(0..1)) / 10;
        println!("REALLY WASN'T TOO BAD AT ALL. {} PEOPLE", haters);
        println!("WOULD DEARLY LIKE TO SEE YOU ASSASSINATED BUT WE ALL HAVE OUR");
        println!("TRIVIAL PROBLEMS.");
    }

    fn score_fantastic() {
        println!("A FANTASTIC PERFORMANCE!!!  CHARLEMANGE, DISRAELI, AND");
        println!("JEFFERSON COMBINED COULD NOT HAVE DONE BETTER!");
    }

    loop {
        current_year += 1;

        startofyearreport(
            &current_year,
            &people_starved,
            &people_came_to_city,
            &mut population,
            &acres_owned,
            &harvested_bushels_per_acre,
            &grain_lost_to_rats,
            &grain_in_store,
        );

        if current_year > 10 {
            endofterm(
                &mut people_starved,
                &mut total_dead_people,
                &mut acres_owned,
                &mut acres_per_person,
                &mut population,
            );

            bool_endofterm_ending = true;
        };

        land_price = get_land_price();

        println!("LAND IS TRADING AT {} BUSHELS PER ACRE.", land_price);

        loop {
            if bool_endofterm_ending {
                break;
            }
            if bool_overreactionmuch_ending{

                break;
            }
            println!("HOW MANY ACRES DO YOU WISH TO BUY");

            let mut input: String = "".to_string();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => u32::MAX,
            };

            if input == u32::MAX {
                bool_overreactionmuch_ending = true;
                overreactionmuch();
                break;
            }

            trade_amount = input;

            if trade_amount == 0 {
                skip_selling_land = false;
            }

            if land_price * trade_amount <= grain_in_store {
                acres_owned += trade_amount;
                grain_in_store -= trade_amount * land_price;
                break;
            } else {
                notenoughgrain(&grain_in_store);
                continue;
            }
        }

        loop {
            if bool_endofterm_ending {
                break;
            }

            if skip_selling_land {
                break;
            }
            if bool_overreactionmuch_ending{

                break;
            }

            println!("HOW MANY ACRES DO YOU WISH TO SELL");

            let mut input: String = "".to_string();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => u32::MAX,
            };

            if input == u32::MAX {
                bool_overreactionmuch_ending = true;
                overreactionmuch();
                break;
            }

            trade_amount = input;

            if trade_amount <= acres_owned {
                acres_owned -= trade_amount;
                grain_in_store += trade_amount * land_price;
                break;
            } else {
                notenoughland(&mut acres_owned);
                continue;
            }
        }

        loop {
            if bool_endofterm_ending {
                break;
            }
            if bool_overreactionmuch_ending{

                break;
            }
            println!("HOW MANY BUSHELS DO YOU WISH TO FEED YOUR PEOPLE");

            let mut input: String = "".to_string();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => u32::MAX,
            };

            if input == u32::MAX {
                bool_overreactionmuch_ending = true;
                overreactionmuch();
                break;
            }

            bushels_to_feed_your_people = input;

            if bushels_to_feed_your_people <= grain_in_store {
                grain_in_store -= bushels_to_feed_your_people;
                break;
            } else {
                notenoughgrain(&grain_in_store);
                continue;
            }
        }

        loop {
            if bool_endofterm_ending {
                break;
            }
            if bool_overreactionmuch_ending{

                break;
            }
            println!("HOW MANY ACRES DO YOU WISH TO PLANT WITH SEED");

            let mut input: String = "".to_string();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => u32::MAX,
            };

            if input == u32::MAX {
                bool_overreactionmuch_ending = true;
                overreactionmuch();
                break;
            }

            acres_to_plant_with_seed = input;

            if acres_to_plant_with_seed <= acres_owned {
                if acres_to_plant_with_seed / 2 < grain_in_store {
                    if acres_to_plant_with_seed < 10 * population {
                        grain_in_store -= acres_to_plant_with_seed / 2;
                        break;
                    } else {
                        notenoughpeopletowork(&population);
                        continue;
                    }
                } else {
                    notenoughgrain(&grain_in_store);
                    continue;
                }
            } else {
                notenoughland(&acres_owned);
                continue;
            }
        }

        if bool_overreactionmuch_ending{

            break;

        }

        harvested_bushels_per_acre = rand::thread_rng().gen_range(1..=5);

        harvest = harvested_bushels_per_acre * acres_to_plant_with_seed;

        if get_hasratshappened() {
            grain_lost_to_rats = grain_in_store / rand::thread_rng().gen_range(1..=5);
        } else {
            grain_lost_to_rats = 0;
        }        

        grain_in_store = grain_in_store + harvest - grain_lost_to_rats;

        people_came_to_city = rand::thread_rng().gen_range(1..=5) * (20 * acres_owned + grain_in_store)
            / population
            / 100
            + 1;

        people_with_full_tummies = bushels_to_feed_your_people / 20;

        if people_with_full_tummies < population {
            people_starved = population - people_with_full_tummies;

            if people_starved > (population * 45) / 100 {
                println!("");
                println!("YOU STARVED {} PEOPLE IN ONE YEAR!!!", people_starved);
                impeached();
                break;
            }
        } else {
            people_starved = 0;
        }

        //P1=((Z-1)*P1+D*100/P)/Z
        percent_of_starved_people_per_year = ((current_year - 1) * percent_of_starved_people_per_year
            + people_starved * 100 / population)
            / current_year;

        total_dead_people += people_starved;

        population = people_with_full_tummies;
    }

    if bool_endofterm_ending {
        if percent_of_starved_people_per_year > 33 || acres_per_person < 7 {
            impeached();
        } else if percent_of_starved_people_per_year > 10 || acres_per_person < 9 {
            score_nero();
        } else if percent_of_starved_people_per_year > 3 || acres_per_person < 10 {
            score_mediocre(&mut population);
        } else {
            score_fantastic();
        }
    }

    println!("");
    println!("SO LONG FOR NOW.");
    println!("");
}
