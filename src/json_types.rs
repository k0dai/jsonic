#[derive(Debug, PartialEq)]
pub enum JsonType {
    JsonNull,
    JsonBoolean,
    JsonString,
    JsonNumber,
    JsonObject,
    JsonArray,
    JsonEmpty
}

#[derive(Debug, PartialEq)]
pub enum JsonNumberType {
    JsonFloat,
    JsonInteger,
}