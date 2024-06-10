
#[no_mangle]
pub fn jpy_to_usd(dollar: f32) -> f32 {
    dollar * (0.0063736171)
}

#[no_mangle]
pub fn usd_to_jpy(dollar: f32) -> f32 {
    dollar * (156.897)
}

#[cfg(test)]
mod tests {
}
