struct BmiRange{
    min: f64,
    max: f64,
    label: &'static str,
}

fn main(){
    let height_cm = input("How tall are you?");
    let weight = input("How much do you weigh?");

    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);

    let bmi_list =vec![
        BmiRange{min: 0.0, max: 18.5, label: "Low"},
        BmiRange{min: 18.5, max: 25.0, label: "Normal"},
        BmiRange{min: 25.0, max: 30.0, label: "High 1"},
        BmiRange{min: 30.0, max: 35.0, label: "High 2"},
        BmiRange{min: 35.0, max: 40.0, label: "high 3"},
        BmiRange{min: 40.0, max: 99.0, label: "high 4"},
    ];

    let mut result = "Unknown";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }
    println!("BMI={:.1}, Result={}", bmi, result);
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Input error");
    s.trim().parse().expect("Failed numerical conversion.")
}