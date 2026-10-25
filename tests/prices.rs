use fee_better_somniel::labels::price;

#[test]
fn thousands_use_the_same_exact_short_notation_in_every_language() {
  for (amount, expected) in [
    (1000, "1k"),
    (1200, "1.2k"),
    (1230, "1.23k"),
    (1234, "1.234k"),
    (10001, "10.001k"),
    (30000, "30k"),
    (99900, "99.9k"),
    (100000, "100k"),
    (i32::MAX, "2147483.647k"),
  ] {
    assert_eq!(price(amount).as_deref(), Some(expected));
  }
}

#[test]
fn small_or_invalid_prices_keep_the_native_display() {
  for amount in [i32::MIN, -1, 0, 100, 200, 999] {
    assert_eq!(price(amount), None);
  }
}

#[test]
fn every_supported_count_has_a_short_exact_price_at_the_vanilla_cost() {
  for count in 2..=1000 {
    let amount = count * 100;
    let Some(label) = price(amount) else { continue };
    assert!(label.len() <= 5, "{label}");
    let mut parts = label.strip_suffix('k').unwrap().split('.');
    let whole: i32 = parts.next().unwrap().parse().unwrap();
    let fraction = parts.next().unwrap_or("");
    let fraction: i32 = format!("{fraction:0<3}").parse().unwrap();
    assert_eq!(whole * 1000 + fraction, amount);
  }
}
