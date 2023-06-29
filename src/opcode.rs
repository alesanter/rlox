pub type Byte = u8;

#[derive(Debug)]
#[repr(u8)]
pub enum Op {
    Return = 0,
    Constant = 1,
    Negate = 2,
    Add = 3,
    Subtract = 4,
    Multiply = 5,
    Divide = 6,
}

impl From<Op> for Byte {
    fn from(value: Op) -> Self {
        return value as Byte;
    }
}

impl TryInto<Op> for Byte {
    type Error = Byte;

    fn try_into(self) -> Result<Op, Self::Error> {
        match self {
            0 => Ok(Op::Return),
            1 => Ok(Op::Constant),
            2 => Ok(Op::Negate),
            3 => Ok(Op::Add),
            4 => Ok(Op::Subtract),
            5 => Ok(Op::Multiply),
            6 => Ok(Op::Divide),
            _ => Err(self),
        }
    }
}
