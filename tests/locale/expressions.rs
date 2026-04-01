use crate::locale::expressions;

expressions!(
    TEST => {
        En: "Test",
        Ru: "Тест"
    },
    TEST_CALLABLE: fn(&str) -> String => {
        En: |argument: &str| argument.to_string(),
        Ru: |argument: &str| argument.to_string(),
    },
);
