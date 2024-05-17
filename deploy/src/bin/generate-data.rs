use rhaki_cw_plus::deploy::Deploier;
use {{crate_name}}_deploy::data::Data;

#[cfg(not(tarpaulin_include))]
fn main() {
    Data::default().generate().unwrap();
}
