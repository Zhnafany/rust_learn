pub use std::collections::HashMap;

#[derive(Debug)]
struct Stats {
    values: Vec<i32>
}

impl Stats {
    //average value
    pub fn mean(&self) -> i32{
        let mut sum = 0;
        for val in &self.values {
            sum += val;   
        }
        (sum as f32 / self.values.len() as f32).round() as i32
    }

    //middle position in sorted
    pub fn median(&self) -> i32{
        let mut vec = self.values.clone();
        vec.sort();
        vec[vec.len() / 2]
    }

    //the value occurs the most
    pub fn mode(&self) -> i32{
        let uniq_repeats = self.count_uniq_vals();
        let mut biggest_repeat_pair = (0,0);
        for (num, count) in &uniq_repeats {
            if biggest_repeat_pair.1 < *count {  
                biggest_repeat_pair = (*num, *count);
            } 
        }
        biggest_repeat_pair.0
    }

    fn count_uniq_vals(&self) -> HashMap<i32,i32> {
        let mut vals_repeat = HashMap::new();
        for val in &self.values {
            let count = vals_repeat.entry(*val).or_insert(0);
            *count += 1;
        }
        vals_repeat
    }
}

pub fn print_all_stats(vec: Vec<i32>) {
    let stats = Stats {
        values: vec,
    };

    println!("Average: {}", stats.mean());
    println!("Median: {}", stats.median());
    println!("Mode: {}", stats.mode());
}
