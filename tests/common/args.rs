pub fn with_default_args<'a>(args: &'a [&'a str]) -> Vec<&'a str> {
    let mut args = args.to_vec();
    args.push("--not-check-updates");
    args
}
