fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let ingredients: Vec<Vec<isize>> = s
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(", ")
                .map(|s| s.split(" ").last().unwrap().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        }).collect();
    println!("ingredients: {:?}", ingredients);
    let mut max = 0;
    let mut max_for_500 = 0;
    for q1 in 0..100 {
        for q2 in 0..(100 - q1) {
            for q3 in 0..(100 - q1 - q2) {
                let q4 = 100 - q1 - q2 - q3;
                let scores = (0..4)
                    .map(|n| {
                        q1 * ingredients[0][n]
                            + q2 * ingredients[1][n]
                            + q3 * ingredients[2][n]
                            + q4 * ingredients[3][n]
                    }).collect::<Vec<isize>>();
                let score = if scores.iter().any(|&s| s < 0) {
                    0
                } else {
                    scores.iter().product::<isize>()
                };
                if q1 * ingredients[0][4]
                    + q2 * ingredients[1][4]
                    + q3 * ingredients[2][4]
                    + q4 * ingredients[3][4]
                    == 500
                {
                    max_for_500 = max_for_500.max(score);
                }
                max = max.max(score);
            }
        }
    }
    println!("max: {}", max);
    println!("max for 500: {}", max_for_500);
}
