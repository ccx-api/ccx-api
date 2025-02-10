use std::fmt;

use console::Style;
use rust_decimal::Decimal;
use smart_string::DisplayExt;
use smart_string::SmartString;

pub struct NiceNum<'a>(pub &'a Style, pub Decimal, pub usize);

impl fmt::Display for NiceNum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(style, num, left) = *self;

        let num: SmartString<62> = format_args!("{num:0.8}").to_fmt();
        let dot_pos = num.bytes().position(|c| c == b'.');
        if let Some(dot_pos) = dot_pos {
            let int_part = left.min(dot_pos).min(10);
            let left_pad = left - int_part.min(left);
            for _ in 0..left_pad {
                write!(f, " ")?;
            }
        }
        let s = num
            .trim_end_matches('0')
            .trim_end_matches('.')
            .trim_end_matches('0');

        write!(f, "{}{}", &style.apply_to(&num[..s.len()]), &num[s.len()..])
    }
}
