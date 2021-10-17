use csv::ReaderBuilder;
use std::time::Duration;


#[derive(Debug)]
struct Problem {
    q: String,
    a: String,
}

impl Problem {
    pub fn new(q: &str, a: &str) -> Problem {
        Self {
            q: q.to_owned(),
            a: a.to_owned(),
        }
    }
}

const FILE_NAME: &str = "../../input/problems.csv";

fn main() {
    let probs = parse_lines().expect("Could not parse CSV file");

    println!("Banana quiz about to start. Press enter when ready.");
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf).ok() {
        None => {
            println!("Error reading user input");
            std::process::exit(1)
        }
        _ => {}
    }

    let mut correct_ans = 0;

    for p in &probs {
        println!("What banana? {}", p.q);
        let (banana_s, banana_r) = std::sync::mpsc::channel();

        std::thread::spawn( move || {
            let mut banana = String::new();
            std::io::stdin().read_line(&mut banana);
            banana.pop();
            banana_s.send(banana).unwrap();
        }); 


        let mut banana = String::new();
        match banana_r.recv_timeout(Duration::from_secs(5)) {
            Ok(b) => {banana.push_str(&b)},
            Err(_) => {
                println!("Only have 5 seconds to input the answer!");
                return;
            }
        }

        println!("Your Banana: {}, Correct Banana: {}\n", banana, p.a);

        if banana != p.a {
            println!("BAD BANANA!");
            break;
        }

        println!("good banana!");
        println!("------------");
        correct_ans += 1;
    }

    println!("Correct answers: {}/{}\n", correct_ans, probs.len())
}

fn parse_lines() -> Result<Vec<Problem>, csv::Error> {
    let mut builder = ReaderBuilder::new();
    builder.has_headers(false);

    let mut reader = builder.from_path(FILE_NAME)?;
    let mut probs = Vec::new();

    for r in reader.records() {
        let rec = r?;
        probs.push(Problem::new(&rec[0], &rec[1]));
    }

    Ok(probs)
}