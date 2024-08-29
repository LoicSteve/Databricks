struct Fighter {
    name: String,
    rating: f64,
}

impl Fighter {
    fn new(name: &str, rating: f64) -> Fighter {
        Fighter {
            name: name.to_string(),
            rating,
        }
    }

    fn update_rating(&mut self, opponent: &mut Fighter, result: &str) {
        let k = 50.0;

        if result == "win" {
            let expected_score = 1.0 / (1.0 + 10.0f64.powf((opponent.rating - self.rating) / 400.0));
            self.rating += k * (1.0 - expected_score);
        } else if result == "loss" {
            let expected_score = 1.0 / (1.0 + 10.0f64.powf((self.rating - opponent.rating) / 400.0));
            self.rating += k * (0.0 - expected_score);
        }
    }
}

fn main() {
    let mut fighter1 = Fighter::new("Conor McGregor", 1500.0);
    let mut fighter2 = Fighter::new("Khabib Nurmagomedov", 1500.0);

    println!("Initial Ratings before match");
    println!("Name: {}, Rating: {}", fighter1.name, fighter1.rating);
    println!("Name: {}, Rating: {}", fighter2.name, fighter2.rating);

    fighter1.update_rating(&mut fighter2, "loss");
    fighter2.update_rating(&mut fighter1, "win");
    println!("Winner: {}", fighter2.name);

    println!("Ratings after match");
    println!("Name: {}, Rating: {}", fighter1.name, fighter1.rating);
    println!("Name: {}, Rating: {}", fighter2.name, fighter2.rating);
}