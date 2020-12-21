mod parts;

pub fn call_function() {
    println!("Machine code: {}", 12_35135_1);

    parts::cog::cog_id();
    parts::gear::gear_id();
}