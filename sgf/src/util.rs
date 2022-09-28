use crate::{ParseError, ParseResult};

/// Parse a board coordinate, such as those contained within B[] and W[] properties
pub fn parse_coords(coord: &str) -> ParseResult<(usize, usize)> {
    if coord.len() != 2 {
        return Err(ParseError::CoordinateParseError);
    }

    let mut bytes = coord.as_bytes().iter();

    // TODO: find a better way to do this
    let parsed = (
        parse_coord(*bytes.next().ok_or(ParseError::CoordinateParseError)? as char)?,
        parse_coord(*bytes.next().ok_or(ParseError::CoordinateParseError)? as char)?,
    );

    Ok(parsed)
}

fn parse_coord(c: char) -> ParseResult<usize> {
    if !c.is_ascii_alphabetic() {
        return Err(ParseError::CoordinateParseError);
    }

    let u = c as usize;

    if u >= 97 {
        return Ok(u - 97);
    }

    Ok(u - 39)
}

/// Return the (width, height) in a SZ[] property.
pub fn parse_board_size(s: &str) -> ParseResult<(usize, usize)> {
    match s.find(':') {
        None => {
            let w = s.parse::<usize>()?;

            return Ok((w, w));
        },

        Some(x) => {
            let (w_s, h_s) = s.split_at(x);

            let h_s = h_s.trim_start_matches(':');

            let w = w_s.parse::<usize>()?;
            let h = h_s.parse::<usize>()?;

            return Ok((w, h));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords() {
        assert_eq!(parse_coords("aa").unwrap(), (0, 0));
        assert_eq!(parse_coords("ZZ").unwrap(), (51, 51));
        assert_eq!(parse_coords("dE").unwrap(), (3, 30));
    }

    #[test]
    fn wrong_coords() {
        assert_eq!(parse_coords("00"), Err(ParseError::CoordinateParseError));
        assert_eq!(parse_coords("aaa"), Err(ParseError::CoordinateParseError));
        assert_eq!(parse_coords(""), Err(ParseError::CoordinateParseError));
    }

    #[test]
    pub fn board_size() {
        assert_eq!(parse_board_size("19"), Ok((19, 19)));
        assert_eq!(parse_board_size("5:9"), Ok((5, 9)));
    }
}
