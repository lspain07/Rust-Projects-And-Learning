fn main() {
    let working_days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    let working_days_num = [0; 5];

    println!("{}", working_days[3]);

    let nephews_age = vec![14, 9, 0];
    println!("Nephew's age: {:?}", nephews_age);

    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    let mut names = Vec::new();

    names.push("Lukas");
    names.push("Dalen");
    names.push("Michael");

    println!("Names: {:?}", names);
}
