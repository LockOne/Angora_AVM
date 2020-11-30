#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchMethod {
    Gd,
    Random,
    Cbh,
    Mb,
    Avm,
}

pub fn parse_search_method(m: &str) -> SearchMethod {
    match m {
        "gd" => SearchMethod::Gd,
        "random" => SearchMethod::Random,
        "cbh" => SearchMethod::Cbh,
        "mb" => SearchMethod::Mb,
        "avm" => SearchMethod::Avm,
        _ => SearchMethod::Gd,
    }
}
