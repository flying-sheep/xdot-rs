///! Stateless parser extracting xdot operations

use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
};

use super::ops::Op;

/*
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}
  
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}
  
fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex
    )(input)
}
  
fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
  
    Ok((input, Color { red, green, blue }))
}
*/

pub(super) fn parse(spec: &str) -> IResult<&str, Vec<Op>> {
    todo!()
}
