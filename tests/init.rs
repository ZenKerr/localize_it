use localize_it::{init_locale, init_locale_with_storage};

mod init_no_labels_test {
    super::init_locale!(En, Ru);
}

mod init_with_labels_test {
    super::init_locale!(En => "English", Ru => "Русский");
}

mod init_with_storage_no_labels_test {
    super::init_locale_with_storage!(En, Ru);
}

mod init_with_storage_with_labels_test {
    super::init_locale_with_storage!(En => "English", Ru => "Русский");
}
