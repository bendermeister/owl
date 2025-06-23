pub fn stem(s: &str) -> String {
    let s: String = s
        .chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .map(|c| if c == 'y' { 'i' } else { c })
        .collect();

    prefix_cutter(suffix_cutter(&s)).into()
}

pub fn prefix_cutter(s: &str) -> &str {
    if let Some(s) = s.strip_prefix("anti") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("auto") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("de") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("dis") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("down") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("extra") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("hiper") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("inter") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("il") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("im") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("in") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("ir") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("mega") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("mid") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("mis") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("non") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("over") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("out") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("post") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("pre") {
        return prefix_cutter(s);
    }
    if let Some(s) = s.strip_prefix("pro") {
        return prefix_cutter(s);
    }

    return s;
}

pub fn suffix_cutter(s: &str) -> &str {
    if let Some(s) = s.strip_suffix("aci") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("al") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ance") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ence") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("dom") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("er") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("or") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("iti") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ti") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ment") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ness") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ship") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("sion") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("tion") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ate") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("en") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ifi") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("fi") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ize") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("able") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ible") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("al") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("esque") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ful") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ic") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ical") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ious") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ous") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ish") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ive") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("less") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("i") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("ing") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("en") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("e") {
        return suffix_cutter(s);
    }
    if let Some(s) = s.strip_suffix("s") {
        return suffix_cutter(s);
    }

    return s;
}
