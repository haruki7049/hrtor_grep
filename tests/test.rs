use hrtor_grep::run;

#[test]
fn hoge_test() -> anyhow::Result<()> {
    let result = run(String::from("hogehoge\nhogehoge\nhogehoge"), "h")?;

    assert_eq!(result, "hogehoge\nhogehoge\nhogehoge\n");
    Ok(())
}

#[test]
fn foobar_test() -> anyhow::Result<()> {
    let result = run(String::from("barbar\nfoofoo\nfoobar"), "f")?;

    assert_eq!(result, "foofoo\nfoobar\n");
    Ok(())
}
