use rand::seq::SliceRandom;
use rand::Rng;
use open;

struct GameState {
    // store paths to images that are soup
    soup_images: Vec<String>,
    // store paths to images that are not soup
    other_images: Vec<String>,
    // score of the game
    score: u32,
}

impl GameState {
    fn new() -> GameState {
        // Initialize the game state with paths to images
        let mut soup_images = Vec::new();
        let mut other_images = Vec::new();

        // there is a total of 10 images for soup
        for i in 1..=10 {
            let image_path = format!("assets/soup/soup{}.jpeg", i);
            soup_images.push(image_path);
        }

        // there is a total of 10 images for other photos
        for i in 1..=10 {
            let image_path = format!("assets/other/other{}.jpeg", i);
            other_images.push(image_path);
        }

        GameState {
            // our game state will have 10 soup images and 10 other images
            soup_images,
            other_images,
            score: 0,
        }
    }

    fn play_round(&mut self) {
        // Randomly choose an image
        let mut rng = rand::thread_rng();
        // boolean soup value to determine if the image is soup or not
        let is_soup = rng.gen::<bool>();
        // if the image is soup, choose a random soup image, otherwise choose a random other image
        let image_path = if is_soup {
            self.soup_images.choose(&mut rng).unwrap()
        } else {
            self.other_images.choose(&mut rng).unwrap()
        };

        // Display image using open crate
        // open the image using the default program for the image type
        if let Err(err) = open::that(&image_path) {
            eprintln!("Error opening image: {}", err);
            // Skip further processing if image opening failed
            return; 
        }

        // Prompt the user for input
        println!("Is this soup? (y/n): ");
        // Read user input
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Trim and convert to lowercase
        let guess = guess.trim().to_lowercase();

        // Check if the user's guess matches the image type
        // If the image is soup, the user's guess should be "y" or "yes"
        // If the image is not soup, the user's guess should be "n" or "no"
        let is_match = if is_soup {
            guess == "y" || guess == "yes"
        } else {
            guess == "n" || guess == "no"
        };

        // Display the result, add to score if correct
        if is_match {
            println!("Correct!");
            self.score += 1;
        // Display the result, subtract from score if incorrect
        } else {
            println!("Incorrect!");
        }
    }
}

fn main() {
    // Create a new game state
    let mut game_state = GameState::new();

    // show the user 5 images
    for _ in 0..5 {
        game_state.play_round();
    }
    
    // Display the final score
    println!("Final score: {}", game_state.score);
}