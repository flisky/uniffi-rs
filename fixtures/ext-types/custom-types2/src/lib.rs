use custom_types::Ouid2;

#[uniffi::export]
pub fn get_ouid2(ouid: Option<Ouid2>) -> Ouid2 {
    ouid.unwrap_or_else(|| Ouid2("Ouid".to_string()))
}

uniffi::setup_scaffolding!();
