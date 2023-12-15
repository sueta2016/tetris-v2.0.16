#[derive(Debug, PartialEq)]
struct Pixel {
    x: usize,
    y: usize,
}
#[derive(Debug)]
pub struct Field {
    width: usize,
    height: usize,
    landscape: Vec<Pixel>,
    figure: Vec<Pixel>,
}

impl Field {
    fn new(width: usize, height: usize) -> Field {
        Field {
            width,
            height,
            landscape: vec![],
            figure: vec![],
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        for pixel in self.figure.iter() {
            if pixel.x == x && pixel.y == y {
                return 'p';
            }
        }

        for pixel in self.landscape.iter() {
            if pixel.x == x && pixel.y == y {
                return '#';
            }
        }

        '.'
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                result += &self.get(x, y).to_string()
            }
            result += "\n"
        }
        result
    }

    pub fn can_move(&self) -> bool {
        let mut can_move = true;

        for figure_pixel in self.figure.iter() {
            let new_y = figure_pixel.y + 1;

            if new_y == self.height {
                can_move = false;
                break;
            }

            for landscape_pixel in self.landscape.iter() {
                if figure_pixel.x == landscape_pixel.x && new_y == landscape_pixel.y {
                    can_move = false
                }
            }
        }
        can_move
    }

    pub fn move_figure(&mut self) {
        for figure_pixel in self.figure.iter_mut() {
            figure_pixel.y += 1
        }
    }
}

pub fn parse_into_field<'a>(input: &'a str) -> Result<Field, &'a str> {
    let lines: Vec<&str> = input.split('\n').collect();

    if lines.len() < 2 {
        return Err("Invalid input");
    }

    let dimensions: Vec<&str> = lines[0].split(' ').collect();
    let width = dimensions[0].parse().unwrap();
    let height = dimensions[1].parse().unwrap();

    let mut field = Field::new(width, height);

    if height + 1 != lines.len() {
        return Err("Specified height don't match with the actual one");
    }

    for y in 0..height {
        let line = lines[y + 1].trim();

        if line.len() != width {
            return Err("Specified width don't match with the actual one");
        }

        for x in 0..width {
            let pixel = Pixel { x, y };
            let character = line.as_bytes()[x];

            match character {
                b'p' => field.figure.push(pixel),
                b'#' => field.landscape.push(pixel),
                b'.' => {}
                _ => {
                    return Err("Unexpected char in field");
                }
            }
        }
    }
    Ok(field)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_input_into_field() {
        // Arrange
        let input = r"3 4
        .p.
        .p.
        ...
        ###";
        let expected_width = 3;
        let expected_height = 4;
        let expected_landscape: Vec<Pixel> = vec![
            Pixel { x: 0, y: 3 },
            Pixel { x: 1, y: 3 },
            Pixel { x: 2, y: 3 },
        ];
        let expected_figure: Vec<Pixel> = vec![Pixel { x: 1, y: 0 }, Pixel { x: 1, y: 1 }];

        // Act
        let field = parse_into_field(input).unwrap();

        // Assert
        assert_eq!(field.width, expected_width);
        assert_eq!(field.height, expected_height);
        assert_eq!(field.landscape, expected_landscape);
        assert_eq!(field.figure, expected_figure);
    }

    #[test]
    fn should_throw_error_on_invalid_char_in_field() {
        // Arrange
        let input = r"3 4
        .M.
        .E.
        .S.
        #I#";

        // Act
        let error_text = parse_into_field(input).unwrap_err();

        assert_eq!(error_text, "Unexpected char in field")
    }

    #[test]
    fn should_throw_error_on_unmatching_width() {
        // Arrange
        let input = r"2 4
        .p.
        ...
        ...
        ###";

        // Act
        let error_text = parse_into_field(input).unwrap_err();

        assert_eq!(
            error_text,
            "Specified width don't match with the actual one"
        )
    }

    #[test]
    fn should_throw_error_on_unmatching_height() {
        // Arrange
        let input = r"3 5
        .p.
        ...
        ...
        ###";

        // Act
        let error_text = parse_into_field(input).unwrap_err();

        assert_eq!(
            error_text,
            "Specified height don't match with the actual one"
        )
    }

    #[test]
    #[should_panic]
    fn should_throw_error_on_invalid_field_dimensions() {
        // Arrange
        let input = r"mesi -
        .p.
        ...
        ...
        ###";

        // Act
        let _ = parse_into_field(input).unwrap();
    }

    #[test]
    #[should_panic]
    fn should_panic_on_invalid_input() {
        // Arrange
        let input = r"Ronaldo better than Messi";

        // Act
        let _ = parse_into_field(input).unwrap();
    }

    #[test]
    fn should_correctly_cast_to_string() {
        let input = r"3 5
        ppp
        .p.
        ...
        #.#
        ###";
        let field = parse_into_field(input).unwrap();

        let expected_string = r"ppp
.p.
...
#.#
###
";

        assert_eq!(field.to_string(), expected_string)
    }

    #[test]
    fn should_allow_to_move_figure_one_cell_down() {
        let initial_state = r"3 5
        ppp
        .p.
        ...
        #.#
        ###";

        let field = parse_into_field(initial_state).unwrap();

        assert_eq!(field.can_move(), true)
    }

    #[test]
    fn should_not_allow_move_through_landscape() {
        let initial_state = r"3 5
        ...
        ...
        ...
        #p#
        ###";
        let field = parse_into_field(initial_state).unwrap();

        assert_eq!(field.can_move(), false)
    }

    #[test]
    fn should_not_allow_move_through_floor() {
        let initial_state = r"3 5
        ...
        ...
        ...
        ...
        .pp";
        let field = parse_into_field(initial_state).unwrap();

        assert_eq!(field.can_move(), false)
    }

    #[test]
    fn should_move_figure_down() {
        let initial_state = r"3 5
        ppp
        .p.
        ...
        #.#
        ###";

        let expected = r"...
ppp
.p.
#.#
###
";

        let mut field = parse_into_field(initial_state).unwrap();

        field.move_figure();

        assert_eq!(field.to_string(), expected)
    }
}
