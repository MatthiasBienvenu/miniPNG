#[cfg(test)]
mod tests {
    use crate::error::MiniPngError;
    use crate::mininpng::MiniPNG;

    #[test]
    fn test_bw_from_string_all_white() {
        let input = "XXX\nXXX\nXXX";
        let image = MiniPNG::bw_from_string(input).unwrap();

        let expected = "\
Mini-PNG Image
Width: 3
Height: 3
Pixel Type: 0 (1 bit black and white)
Data size: 2 bytes

XXX
XXX
XXX";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn test_bw_from_string_all_black() {
        let input = "   \n   \n   ";
        let image = MiniPNG::bw_from_string(input).unwrap();

        let expected = "\
Mini-PNG Image
Width: 3
Height: 3
Pixel Type: 0 (1 bit black and white)
Data size: 2 bytes



";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn test_bw_from_string_letter_m() {
        let input =
            " X       X\n XX     XX\n X X   X X\n X  X X  X\n X   X   X\n X       X\n X       X";
        let image = MiniPNG::bw_from_string(input).unwrap();

        let expected = "\
Mini-PNG Image
Width: 10
Height: 7
Pixel Type: 0 (1 bit black and white)
Data size: 9 bytes

 X       X
 XX     XX
 X X   X X
 X  X X  X
 X   X   X
 X       X
 X       X";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn test_bw_from_string_width_not_multiple_of_8() {
        let input = "XXXXXXXXXX\n          \nXXXXXXXXXX";
        let image = MiniPNG::bw_from_string(input).unwrap();

        let expected = "\
Mini-PNG Image
Width: 10
Height: 3
Pixel Type: 0 (1 bit black and white)
Data size: 4 bytes

XXXXXXXXXX

XXXXXXXXXX";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn test_bw_from_string_diagonal() {
        let input = "X      \n X     \n  X    \n   X   \n    X  \n     X \n      X";
        let image = MiniPNG::bw_from_string(input).unwrap();

        let expected = "\
Mini-PNG Image
Width: 7
Height: 7
Pixel Type: 0 (1 bit black and white)
Data size: 7 bytes

X
 X
  X
   X
    X
     X
      X";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn test_bw_from_string_single_line() {
        let input = "X X X X X";
        let image = MiniPNG::bw_from_string(input).unwrap();

        let expected = "\
Mini-PNG Image
Width: 9
Height: 1
Pixel Type: 0 (1 bit black and white)
Data size: 2 bytes

X X X X X";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn test_bw_from_string_uneven_lines() {
        let input = "X\nXX\nXXX";
        let image = MiniPNG::bw_from_string(input).unwrap();

        // Shorter lines get padded with spaces to match width
        let expected = "\
Mini-PNG Image
Width: 3
Height: 3
Pixel Type: 0 (1 bit black and white)
Data size: 2 bytes

X
XX
XXX";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn test_bw_from_string_width_17() {
        // Width 17 is tricky - crosses two byte boundaries
        let input = "XXXXXXXXXXXXXXXXX\n                 \nXXXXXXXXXXXXXXXXX";
        let image = MiniPNG::bw_from_string(input).unwrap();

        let expected = "\
Mini-PNG Image
Width: 17
Height: 3
Pixel Type: 0 (1 bit black and white)
Data size: 7 bytes

XXXXXXXXXXXXXXXXX

XXXXXXXXXXXXXXXXX";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn test_bw_from_string_invalid_character() {
        let input = "X X\n#X \nX X";
        let result = MiniPNG::bw_from_string(input);

        assert!(result.is_err());
        match result {
            Err(MiniPngError::IllegalCharacter(c)) => assert_eq!(c, '#'),
            _ => panic!("Expected IllegalCharacter error"),
        }
    }

    #[test]
    fn test_bw_from_string_empty() {
        let input = "";
        let image = MiniPNG::bw_from_string(input).unwrap();

        let expected = "\
Mini-PNG Image
Width: 0
Height: 0
Pixel Type: 0 (1 bit black and white)
Data size: 0 bytes";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn test_bw_from_string_cross() {
        let input = "  X  \n  X  \nXXXXX\n  X  \n  X  ";
        let image = MiniPNG::bw_from_string(input).unwrap();

        let expected = "\
Mini-PNG Image
Width: 5
Height: 5
Pixel Type: 0 (1 bit black and white)
Data size: 4 bytes

  X
  X
XXXXX
  X
  X";

        assert_eq!(
            image
                .display()
                .unwrap()
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }
}
