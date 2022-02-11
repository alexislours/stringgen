use rand::Rng;
use std::time::Instant;

static mut MUTATION_COUNT: i64 = 0;
static mut DEATH_COUNT: i64 = 0;
static mut BIRTH_COUNT: i64 = 0;

fn main() {
    // Get time of launch
    let now = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    // Parse arguments
    match args.len() {
        3|4 => {
            let goal = match args[1].parse::<String>() {
                Ok(n) => n,
                Err(_) => {
                    println!("{} is not a string\n", args[1]);
                    help();
                    return;
                }
            };
            let pop_size = match args[2].parse::<i64>().unwrap() {
                n if n > 0 => n,
                _ => {
                    println!("{} is not a positive number\n", args[2]);
                    help();
                    return;
                }
            };
            let skip_amount;
            if args.len() == 3 {
                skip_amount = 1;
            } else {
                skip_amount = args[3].parse::<i64>().unwrap_or(1);
            }
            let gen_goal = goal.clone();
            let goal_vec: Vec<u32> = gen_goal.chars().map(|c| c as u32).collect();
            let gv = goal_vec.clone();
            let gv2 = goal_vec.clone();
            let pop = populate(gen_goal, pop_size, goal_vec);
            let mut res = generation(pop.to_vec(), pop_size, &gv);
            let mut i = 1;
            loop {
                // Print results if goal is reached
                if res[0].code == goal {
                    let nanos = now.elapsed().as_nanos();
                    let seconds = nanos as f64 / 1_000_000_000.0;
                    let ms = seconds * 1000.0;
                    println!("\n\x1b[0;32m{}\x1b[0m\n", res[0].code);
                    println!("Generations:        \x1b[0;32m{}\x1b[0m", i);
                    println!("Population:         \x1b[0;32m{}\x1b[0m", pop_size);
                    println!("Mutations:          \x1b[0;32m{}\x1b[0m", unsafe {
                        MUTATION_COUNT
                    });
                    println!("Deaths:             \x1b[0;32m{}\x1b[0m", unsafe {
                        DEATH_COUNT
                    });
                    println!("Births:             \x1b[0;32m{}\x1b[0m", unsafe {
                        BIRTH_COUNT
                    });
                    println!("Run time:           \x1b[0;32m{}s\x1b[0m", seconds);
                    println!("Generation time:    \x1b[0;32m{}ms\x1b[0m", ms / i as f64);
                    println!(
                        "Generation/s:       \x1b[0;32m{}\x1b[0m",
                        i as f64 / seconds
                    );
                    println!("Birth/s:            \x1b[0;32m{}\x1b[0m", unsafe {
                        BIRTH_COUNT as f64 / seconds
                    });
                    break;
                } 
                // Continue iteration
                else {
                    if i % skip_amount == 0 {
                        println!(
                        "iteration: \x1b[0;32m{}\x1b[0m, current best string: \x1b[0;33m{}\x1b[0m",
                        i, res[0].code
                    );
                    }
                    res = generation(res, pop_size, &gv2);
                    i += 1;
                }
            }
        }
        _ => {
            help();
        }
    }
}

fn help() {
    println!(
        "usage:
stringgen <String to generate> <Population size> [print every x generations]"
    );
}

#[derive(Clone)]
struct Chromosome {
    code: String,
    cost: u32,
}

impl Chromosome {
    fn new(code: String, cost: u32) -> Chromosome {
        Chromosome { code, cost }
    }
}

fn generation(
    mut population: Vec<Chromosome>,
    pop_size: i64,
    goal_vec: &Vec<u32>,
) -> Vec<Chromosome> {
    for i in 0..population.len() {
        if population[i].cost == 0 {
            return population;
        }
    }
    population = sort_population(population);
    population = select_population(population, pop_size);
    population = mate_population(population);
    population = mutate_population(&population, goal_vec.clone());
    return population;
}

// Create the initial population
fn populate(goal: String, population_size: i64, goal_vec: Vec<u32>) -> Vec<Chromosome> {
    let mut population: Vec<Chromosome> = Vec::new();
    for _ in 0..population_size {
        let mut chromosome = Chromosome::new(random_code(goal.len() as u64), u32::MAX);
        chromosome.cost = calc_cost(chromosome.code.clone(), goal_vec.clone());
        population.push(chromosome);
    }
    population
}

// Sort the population by cost
fn sort_population(mut population: Vec<Chromosome>) -> Vec<Chromosome> {
    population.sort_by(|a, b| a.cost.cmp(&b.cost));
    population
}

// Remove the worst half of the population
fn select_population(population: Vec<Chromosome>, pop_size: i64) -> Vec<Chromosome> {
    let mut selected_population: Vec<Chromosome> = population;
    let pre_purge_population_size = selected_population.len();
    selected_population.truncate(pop_size as usize);
    unsafe {
        DEATH_COUNT += pre_purge_population_size as i64 - selected_population.len() as i64;
    }
    selected_population
}

// Run the reproduction across the entire population
fn mate_population(population: Vec<Chromosome>) -> Vec<Chromosome> {
    let mut mate_population: Vec<Chromosome> = population.clone();
    for i in 0..population.len() / 2 {
        let mut random = i;
        while random == i {
            random = rand::thread_rng().gen_range(0..population.len());
        }
        let childs = mate(&population[i], &population[random]);
        mate_population.push(childs.0);
        mate_population.push(childs.1);
        unsafe {
            BIRTH_COUNT += 2;
        }
    }
    mate_population
}

fn mutate_population(population: &Vec<Chromosome>, goal_vec: Vec<u32>) -> Vec<Chromosome> {
    let mut mutated_population: Vec<Chromosome> = Vec::new();
    for chromosome in population {
        let code = chromosome.code.clone();
        let mutated = mutate(0.7, code);
        let mutated_str = mutated.clone();
        let calculated_cost;
        if mutated.1 {
            calculated_cost = calc_cost(mutated.0, goal_vec.clone());
        } else {
            calculated_cost = chromosome.cost
        }
        mutated_population.push(Chromosome::new(mutated_str.0, calculated_cost));
    }
    mutated_population = sort_population(mutated_population);
    mutated_population
}

// Generate a random string of length n for creating the initial population
fn random_code(length: u64) -> String {
    let mut rng = rand::thread_rng();
    let mut code = String::new();
    for _ in 0..length {
        code.push(rng.gen_range(32..125) as u8 as char);
    }
    code
}

// Calculate the cost of a code
fn calc_cost(code: String, goal_vec: Vec<u32>) -> u32 {
    let mut cost: u32 = 0;
    // Fastest way to iterate over a string
    for (c, g) in code.chars().zip(goal_vec.iter()) {
        cost += (c as u32 - g).pow(2);
    }
    cost
}

fn mate(a: &Chromosome, b: &Chromosome) -> (Chromosome, Chromosome) {
    // Children take half of the genes from each parent
    let pivot = (a.code.len() / 2) - 1;
    let child_a: String = a
        .code
        .chars()
        .take(pivot)
        .chain(b.code.chars().skip(pivot))
        .collect();
    let child_b: String = b
        .code
        .chars()
        .take(pivot)
        .chain(a.code.chars().skip(pivot))
        .collect();
    (
        // Instantiate children with max value of unsigned 32 before calculating the costs
        Chromosome::new(child_a, u32::MAX),
        Chromosome::new(child_b, u32::MAX),
    )
}

// Randomly mutate a Chromosome
fn mutate(odds: f64, code: String) -> (String, bool) {
    // Check if the mutation should occur
    if rand::thread_rng().gen_range(0.0..1.0) < odds {
        let index = rand::thread_rng().gen_range(0..code.len());
        // Randomly select codepoint direction to mutate
        let direction = if rand::thread_rng().gen_range(0.0..1.0) > 0.5 {
            1
        } else {
            -1
        };
        let mut new_code = code.clone();
        let value = new_code.chars().nth(index).unwrap();
        if (value as i64) + direction < 126 && (value as i64) + direction > 31 {
            unsafe {
                MUTATION_COUNT += 1;
            }
            new_code.remove(index);
            new_code.insert(
                index,
                (code.chars().nth(index).unwrap() as i64 + direction) as u8 as char,
            );
        }
        (new_code, true)
    } else {
        (code, false)
    }
}
