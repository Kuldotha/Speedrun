pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    // Calculate the square magnitude (length) of the vector
    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    // Calculate the magnitude (length) of the vector
    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    // Calculate the dot product of two vectors
    pub fn dot_product(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    // Calculate the angle (in radians) between two vectors
    pub fn angle_between(&self, other: &Vector2) -> f32 {
        let dot_product = self.dot_product(other);
        let magnitude_product = self.magnitude() * other.magnitude();

        if magnitude_product == 0.0 {
            // Prevent division by zero
            0.0
        } else {
            (dot_product / magnitude_product).acos()
        }
    }

    // Vector addition: Returns a new vector representing the sum of two vectors
    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    // Vector subtraction: Returns a new vector representing the difference of two vectors
    pub fn subtract(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
