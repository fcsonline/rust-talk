let odds_squared: Vec<_> = (1..100)
    .filter(|x| x % 2 != 0)
    .map(|x| x * x)
    .collect();
