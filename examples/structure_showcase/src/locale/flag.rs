use crate::locale::expression;

// Create expression that can be used like variable
expression!(IS_ENGLISH: bool => {
    En: true,
    Ru: false,
});
