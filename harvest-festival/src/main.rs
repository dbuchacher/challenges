/*
    You have cultivated a plant, and after three long months, the time has come to reap the fruits (or the flowers, in this case) of your hard work. During the growth phase, you added water and fertilizer, and kept a constant temperature. It's time to check how much the plant has grown!

    A plant is represented horizontally, from the base to the left, to the end to the right:

    ---@---@---@

    The stem is made of hyphens, and the flowers are represented by symbols. A plant always starts with the stem, and always ends with flowers.

    The four given parameters are:

        seed (string) determines the type of flowers generated by the plant.
        water (integer) each unit of water extends the portion of stem between the flowers, and gives the total number of segments (stem + flowers) of the plant.
        fert (integer) each unit of fertilizer increases the amount of flowers, grouped in clusters.
        temp (integer) if the temperature recorded is between 20°C and 30°C (bounds included) the plant grows normally, otherwise all the flowers die, except for a single survivor at the end of the stem.

    Given the above parameters, implement a function that returns a string representing the plant (see the examples below for a better visualization).
    
    https://edabit.com/challenge/CmWQTvvkXSeaNGdDy
*/ 


fn plant(seed: &str, water: usize, fert: usize, temp: usize) -> String {
    let stem = "-".repeat(water);
    let flower = seed.repeat(fert);

    let mut result = String::new();

    for _i in 0..water {
        result.push_str(&stem); // grow stem
        if (20..=30).contains(&temp) { result.push_str(&flower) } // grow flowers if temp is good
    }

    if !(20..=30).contains(&temp) { result.push_str(&seed) } // grows one flower if temp is bad

    result
}

fn main() {
    assert_eq!(plant("@", 3, 3, 25), "---@@@---@@@---@@@");
    assert_eq!(plant("#", 1, 5, 30), "-#####");
    assert_eq!(plant("&", 5, 1, 20), "-----&-----&-----&-----&-----&");
    assert_eq!(plant("§", 3, 3, 15), "---------§");
}