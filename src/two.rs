pub enum Direction {
    Forward(u16),
    Up(u16),
    Down(u16)
}

pub struct Submarine {
    horizontal_position: u16,
    depth: i32,
    aim: i32,
}

impl Submarine {
    pub fn new() -> Self {
        Submarine{
            horizontal_position: 0,
            depth: 0,
            aim: 0
        }
    }

    pub fn move_submarine(&mut self, direction: &Direction) {
         match direction {
            Direction::Forward(value) => {
                self.horizontal_position += value;
                self.depth += self.aim * (*value as i32);
                if self.depth < 0 {
                    self.depth = 0
                }
            },
            Direction::Up(value) => {
                self.aim -= *value as i32;
            }
            Direction::Down(value) => {
                self.aim += *value as i32;
            }
        }
    }

    pub fn multiply_coordinates(&self) -> u32 {
        self.horizontal_position as u32 * self.depth as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let submarine = Submarine::new();
        assert_eq!(submarine.horizontal_position, 0);
        assert_eq!(submarine.depth, 0);
        assert_eq!(submarine.aim, 0);
    }

    #[test]
    fn test_move_up() {
        let mut submarine = Submarine::new();
        submarine.move_submarine(&Direction::Up(5));

        assert_eq!(submarine.aim, -5);
    }

    #[test]
    fn test_movement_down() {
        let mut submarine = Submarine::new();
        submarine.move_submarine(&Direction::Down(5));

        assert_eq!(submarine.aim, 5);
    }

    #[test]
    fn test_move_forward() {
        let mut submarine = Submarine::new();
        submarine.move_submarine(&Direction::Forward(5));
        assert_eq!(submarine.horizontal_position, 5);
        assert_eq!(submarine.depth, 0)
    }

    #[test]
    fn test_move_foward_down() {
        let mut submarine = Submarine::new();
        submarine.move_submarine(&Direction::Down(5));
        submarine.move_submarine(&Direction::Forward(10));

        assert_eq!(submarine.horizontal_position, 10);
        assert_eq!(submarine.depth, 50);
        assert_eq!(submarine.aim, 5)
    }

    #[test]
    fn test_move_foward_up() {
        let mut submarine = Submarine::new();
        submarine.move_submarine(&Direction::Up(5));
        submarine.move_submarine(&Direction::Forward(10));

        assert_eq!(submarine.horizontal_position, 10);
        assert_eq!(submarine.depth, 0);
        assert_eq!(submarine.aim, -5)
    }

    #[test]
    fn test_multiply_coordinates() {
        let mut submarine = Submarine::new();
        submarine.move_submarine(&Direction::Down(5));
        submarine.move_submarine(&Direction::Forward(10));

        assert_eq!(submarine.multiply_coordinates(), 500);
    }
}