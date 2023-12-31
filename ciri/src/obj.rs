#[derive(Debug)]
pub(crate) enum Object {
    Integer(IntObj),
    Float(FloatObj),
    String(StrObj),
    Boolean(BoolObj),
    Char(CharObj),
}

#[derive(Debug)]
pub(crate) struct IntObj(pub(crate) isize);

#[derive(Debug)]
pub(crate) struct FloatObj(pub(crate) f32);

#[derive(Debug)]
pub(crate) struct StrObj(pub(crate) String);

#[derive(Debug)]
pub(crate) struct BoolObj(pub(crate) bool);

#[derive(Debug)]
pub(crate) struct CharObj(pub(crate) char);