use localize_it::{init_locale, init_locale_with_storage};

mod init_no_labels_test {
    super::init_locale!(EN, RU);
}

mod init_with_labels_test {
    super::init_locale!(EN => "English", RU => "Русский");
}

mod init_with_storage_no_labels_test {
    super::init_locale_with_storage!(EN, RU);
}

mod init_with_storage_with_labels_test {
    super::init_locale_with_storage!(EN => "English", RU => "Русский");
}
