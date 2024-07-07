use glob_lab::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert!(glob_match_with_brace("**/foo{bar,b*z}", "foobuzz"));
  }

  #[test]
  fn basic() {
    assert!(glob_match_with_brace("abc", "abc"));
    assert!(glob_match_with_brace("*", "abc"));
    assert!(glob_match_with_brace("*", ""));
    assert!(glob_match_with_brace("**", ""));
    assert!(glob_match_with_brace("*c", "abc"));
    assert!(!glob_match_with_brace("*b", "abc"));
    assert!(glob_match_with_brace("a*", "abc"));
    assert!(!glob_match_with_brace("b*", "abc"));
    assert!(glob_match_with_brace("a*", "a"));
    assert!(glob_match_with_brace("*a", "a"));
    assert!(glob_match_with_brace("a*b*c*d*e*", "axbxcxdxe"));
    assert!(glob_match_with_brace("a*b*c*d*e*", "axbxcxdxexxx"));
    assert!(glob_match_with_brace("a*b?c*x", "abxbbxdbxebxczzx"));
    assert!(!glob_match_with_brace("a*b?c*x", "abxbbxdbxebxczzy"));

    assert!(glob_match_with_brace("a/*/test", "a/foo/test"));
    assert!(!glob_match_with_brace("a/*/test", "a/foo/bar/test"));
    assert!(glob_match_with_brace("a/**/test", "a/foo/test"));
    assert!(glob_match_with_brace("a/**/test", "a/foo/bar/test"));
    assert!(glob_match_with_brace("a/**/b/c", "a/foo/bar/b/c"));
    assert!(glob_match_with_brace("a\\*b", "a*b"));
    assert!(!glob_match_with_brace("a\\*b", "axb"));

    assert!(glob_match_with_brace("[abc]", "a"));
    assert!(glob_match_with_brace("[abc]", "b"));
    assert!(glob_match_with_brace("[abc]", "c"));
    assert!(!glob_match_with_brace("[abc]", "d"));
    assert!(glob_match_with_brace("x[abc]x", "xax"));
    assert!(glob_match_with_brace("x[abc]x", "xbx"));
    assert!(glob_match_with_brace("x[abc]x", "xcx"));
    assert!(!glob_match_with_brace("x[abc]x", "xdx"));
    assert!(!glob_match_with_brace("x[abc]x", "xay"));
    assert!(glob_match_with_brace("[?]", "?"));
    assert!(!glob_match_with_brace("[?]", "a"));
    assert!(glob_match_with_brace("[*]", "*"));
    assert!(!glob_match_with_brace("[*]", "a"));

    assert!(glob_match_with_brace("[a-cx]", "a"));
    assert!(glob_match_with_brace("[a-cx]", "b"));
    assert!(glob_match_with_brace("[a-cx]", "c"));
    assert!(!glob_match_with_brace("[a-cx]", "d"));
    assert!(glob_match_with_brace("[a-cx]", "x"));

    assert!(!glob_match_with_brace("[^abc]", "a"));
    assert!(!glob_match_with_brace("[^abc]", "b"));
    assert!(!glob_match_with_brace("[^abc]", "c"));
    assert!(glob_match_with_brace("[^abc]", "d"));
    assert!(!glob_match_with_brace("[!abc]", "a"));
    assert!(!glob_match_with_brace("[!abc]", "b"));
    assert!(!glob_match_with_brace("[!abc]", "c"));
    assert!(glob_match_with_brace("[!abc]", "d"));
    assert!(glob_match_with_brace("[\\!]", "!"));

    assert!(glob_match_with_brace("a*b*[cy]*d*e*", "axbxcxdxexxx"));
    assert!(glob_match_with_brace("a*b*[cy]*d*e*", "axbxyxdxexxx"));
    assert!(glob_match_with_brace("a*b*[cy]*d*e*", "axbxxxyxdxexxx"));

    assert!(glob_match_with_brace("test.{jpg,png}", "test.jpg"));
    assert!(glob_match_with_brace("test.{jpg,png}", "test.png"));
    assert!(glob_match_with_brace("test.{j*g,p*g}", "test.jpg"));
    assert!(glob_match_with_brace("test.{j*g,p*g}", "test.jpxxxg"));
    assert!(glob_match_with_brace("test.{j*g,p*g}", "test.jxg"));
    assert!(!glob_match_with_brace("test.{j*g,p*g}", "test.jnt"));

    assert!(glob_match_with_brace("test.{j*g,j*c}", "test.jnc"));
    assert!(glob_match_with_brace("test.{jpg,p*g}", "test.png"));
    assert!(glob_match_with_brace("test.{jpg,p*g}", "test.pxg"));
    assert!(!glob_match_with_brace("test.{jpg,p*g}", "test.pnt"));
    assert!(glob_match_with_brace("test.{jpeg,png}", "test.jpeg"));
    assert!(!glob_match_with_brace("test.{jpeg,png}", "test.jpg"));
    assert!(glob_match_with_brace("test.{jpeg,png}", "test.png"));
    assert!(glob_match_with_brace("test.{jp\\,g,png}", "test.jp,g"));
    assert!(!glob_match_with_brace("test.{jp\\,g,png}", "test.jxg"));
    assert!(glob_match_with_brace("test/{foo,bar}/baz", "test/foo/baz"));
    assert!(glob_match_with_brace("test/{foo,bar}/baz", "test/bar/baz"));
    assert!(!glob_match_with_brace("test/{foo,bar}/baz", "test/baz/baz"));
    assert!(glob_match_with_brace(
      "test/{foo*,bar*}/baz",
      "test/foooooo/baz"
    ));
    assert!(glob_match_with_brace(
      "test/{foo*,bar*}/baz",
      "test/barrrrr/baz"
    ));
    assert!(glob_match_with_brace(
      "test/{*foo,*bar}/baz",
      "test/xxxxfoo/baz"
    ));
    assert!(glob_match_with_brace(
      "test/{*foo,*bar}/baz",
      "test/xxxxbar/baz"
    ));
    assert!(glob_match_with_brace(
      "test/{foo/**,bar}/baz",
      "test/bar/baz"
    ));
    assert!(!glob_match_with_brace(
      "test/{foo/**,bar}/baz",
      "test/bar/test/baz"
    ));

    assert!(!glob_match_with_brace(
      "*.txt",
      "some/big/path/to/the/needle.txt"
    ));
    assert!(glob_match_with_brace(
      "some/**/needle.{js,tsx,mdx,ts,jsx,txt}",
      "some/a/bigger/path/to/the/crazy/needle.txt"
    ));
    assert!(glob_match_with_brace(
      "some/**/{a,b,c}/**/needle.txt",
      "some/foo/a/bigger/path/to/the/crazy/needle.txt"
    ));
    assert!(!glob_match_with_brace(
      "some/**/{a,b,c}/**/needle.txt",
      "some/foo/d/bigger/path/to/the/crazy/needle.txt"
    ));

    assert!(glob_match_with_brace("a/{a{a,b},b}", "a/aa"));
    assert!(glob_match_with_brace("a/{a{a,b},b}", "a/ab"));
    assert!(!glob_match_with_brace("a/{a{a,b},b}", "a/ac"));
    assert!(glob_match_with_brace("a/{a{a,b},b}", "a/b"));
    assert!(!glob_match_with_brace("a/{a{a,b},b}", "a/c"));
    assert!(glob_match_with_brace("a/{b,c[}]*}", "a/b"));
    assert!(glob_match_with_brace("a/{b,c[}]*}", "a/c}xx"));

    assert!(glob_match_with_brace("/**/*a", "/a/a"));
    assert!(glob_match_with_brace("**/*.js", "a/b.c/c.js"));
    assert!(glob_match_with_brace("**/**/*.js", "a/b.c/c.js"));
    assert!(glob_match_with_brace("a/**/*.d", "a/b/c.d"));
    assert!(glob_match_with_brace("a/**/*.d", "a/.b/c.d"));

    assert!(glob_match_with_brace("**/*/**", "a/b/c"));
    assert!(glob_match_with_brace("**/*/c.js", "a/b/c.js"));
  }

  // The below tests are based on Bash and micromatch.
  // https://github.com/micromatch/picomatch/blob/master/test/bash.js
  // Converted using the following find and replace regex:
  // find: assert\(([!])?isMatch\('(.*?)', ['"](.*?)['"]\)\);
  // replace: assert!($1glob_match_with_brace("$3", "$2"));

  #[test]
  fn bash() {
    assert!(!glob_match_with_brace("a*", "*"));
    assert!(!glob_match_with_brace("a*", "**"));
    assert!(!glob_match_with_brace("a*", "\\*"));
    assert!(!glob_match_with_brace("a*", "a/*"));
    assert!(!glob_match_with_brace("a*", "b"));
    assert!(!glob_match_with_brace("a*", "bc"));
    assert!(!glob_match_with_brace("a*", "bcd"));
    assert!(!glob_match_with_brace("a*", "bdir/"));
    assert!(!glob_match_with_brace("a*", "Beware"));
    assert!(glob_match_with_brace("a*", "a"));
    assert!(glob_match_with_brace("a*", "ab"));
    assert!(glob_match_with_brace("a*", "abc"));

    assert!(!glob_match_with_brace("\\a*", "*"));
    assert!(!glob_match_with_brace("\\a*", "**"));
    assert!(!glob_match_with_brace("\\a*", "\\*"));

    assert!(glob_match_with_brace("\\a*", "a"));
    assert!(!glob_match_with_brace("\\a*", "a/*"));
    assert!(glob_match_with_brace("\\a*", "abc"));
    assert!(glob_match_with_brace("\\a*", "abd"));
    assert!(glob_match_with_brace("\\a*", "abe"));
    assert!(!glob_match_with_brace("\\a*", "b"));
    assert!(!glob_match_with_brace("\\a*", "bb"));
    assert!(!glob_match_with_brace("\\a*", "bcd"));
    assert!(!glob_match_with_brace("\\a*", "bdir/"));
    assert!(!glob_match_with_brace("\\a*", "Beware"));
    assert!(!glob_match_with_brace("\\a*", "c"));
    assert!(!glob_match_with_brace("\\a*", "ca"));
    assert!(!glob_match_with_brace("\\a*", "cb"));
    assert!(!glob_match_with_brace("\\a*", "d"));
    assert!(!glob_match_with_brace("\\a*", "dd"));
    assert!(!glob_match_with_brace("\\a*", "de"));
  }

  #[test]
  fn bash_directories() {
    assert!(!glob_match_with_brace("b*/", "*"));
    assert!(!glob_match_with_brace("b*/", "**"));
    assert!(!glob_match_with_brace("b*/", "\\*"));
    assert!(!glob_match_with_brace("b*/", "a"));
    assert!(!glob_match_with_brace("b*/", "a/*"));
    assert!(!glob_match_with_brace("b*/", "abc"));
    assert!(!glob_match_with_brace("b*/", "abd"));
    assert!(!glob_match_with_brace("b*/", "abe"));
    assert!(!glob_match_with_brace("b*/", "b"));
    assert!(!glob_match_with_brace("b*/", "bb"));
    assert!(!glob_match_with_brace("b*/", "bcd"));
    assert!(glob_match_with_brace("b*/", "bdir/"));
    assert!(!glob_match_with_brace("b*/", "Beware"));
    assert!(!glob_match_with_brace("b*/", "c"));
    assert!(!glob_match_with_brace("b*/", "ca"));
    assert!(!glob_match_with_brace("b*/", "cb"));
    assert!(!glob_match_with_brace("b*/", "d"));
    assert!(!glob_match_with_brace("b*/", "dd"));
    assert!(!glob_match_with_brace("b*/", "de"));
  }

  #[test]
  fn bash_escaping() {
    assert!(!glob_match_with_brace("\\^", "*"));
    assert!(!glob_match_with_brace("\\^", "**"));
    assert!(!glob_match_with_brace("\\^", "\\*"));
    assert!(!glob_match_with_brace("\\^", "a"));
    assert!(!glob_match_with_brace("\\^", "a/*"));
    assert!(!glob_match_with_brace("\\^", "abc"));
    assert!(!glob_match_with_brace("\\^", "abd"));
    assert!(!glob_match_with_brace("\\^", "abe"));
    assert!(!glob_match_with_brace("\\^", "b"));
    assert!(!glob_match_with_brace("\\^", "bb"));
    assert!(!glob_match_with_brace("\\^", "bcd"));
    assert!(!glob_match_with_brace("\\^", "bdir/"));
    assert!(!glob_match_with_brace("\\^", "Beware"));
    assert!(!glob_match_with_brace("\\^", "c"));
    assert!(!glob_match_with_brace("\\^", "ca"));
    assert!(!glob_match_with_brace("\\^", "cb"));
    assert!(!glob_match_with_brace("\\^", "d"));
    assert!(!glob_match_with_brace("\\^", "dd"));
    assert!(!glob_match_with_brace("\\^", "de"));

    assert!(glob_match_with_brace("\\*", "*"));
    // assert!(glob_match_with_brace("\\*", "\\*"));
    assert!(!glob_match_with_brace("\\*", "**"));
    assert!(!glob_match_with_brace("\\*", "a"));
    assert!(!glob_match_with_brace("\\*", "a/*"));
    assert!(!glob_match_with_brace("\\*", "abc"));
    assert!(!glob_match_with_brace("\\*", "abd"));
    assert!(!glob_match_with_brace("\\*", "abe"));
    assert!(!glob_match_with_brace("\\*", "b"));
    assert!(!glob_match_with_brace("\\*", "bb"));
    assert!(!glob_match_with_brace("\\*", "bcd"));
    assert!(!glob_match_with_brace("\\*", "bdir/"));
    assert!(!glob_match_with_brace("\\*", "Beware"));
    assert!(!glob_match_with_brace("\\*", "c"));
    assert!(!glob_match_with_brace("\\*", "ca"));
    assert!(!glob_match_with_brace("\\*", "cb"));
    assert!(!glob_match_with_brace("\\*", "d"));
    assert!(!glob_match_with_brace("\\*", "dd"));
    assert!(!glob_match_with_brace("\\*", "de"));

    assert!(!glob_match_with_brace("a\\*", "*"));
    assert!(!glob_match_with_brace("a\\*", "**"));
    assert!(!glob_match_with_brace("a\\*", "\\*"));
    assert!(!glob_match_with_brace("a\\*", "a"));
    assert!(!glob_match_with_brace("a\\*", "a/*"));
    assert!(!glob_match_with_brace("a\\*", "abc"));
    assert!(!glob_match_with_brace("a\\*", "abd"));
    assert!(!glob_match_with_brace("a\\*", "abe"));
    assert!(!glob_match_with_brace("a\\*", "b"));
    assert!(!glob_match_with_brace("a\\*", "bb"));
    assert!(!glob_match_with_brace("a\\*", "bcd"));
    assert!(!glob_match_with_brace("a\\*", "bdir/"));
    assert!(!glob_match_with_brace("a\\*", "Beware"));
    assert!(!glob_match_with_brace("a\\*", "c"));
    assert!(!glob_match_with_brace("a\\*", "ca"));
    assert!(!glob_match_with_brace("a\\*", "cb"));
    assert!(!glob_match_with_brace("a\\*", "d"));
    assert!(!glob_match_with_brace("a\\*", "dd"));
    assert!(!glob_match_with_brace("a\\*", "de"));

    assert!(glob_match_with_brace("*q*", "aqa"));
    assert!(glob_match_with_brace("*q*", "aaqaa"));
    assert!(!glob_match_with_brace("*q*", "*"));
    assert!(!glob_match_with_brace("*q*", "**"));
    assert!(!glob_match_with_brace("*q*", "\\*"));
    assert!(!glob_match_with_brace("*q*", "a"));
    assert!(!glob_match_with_brace("*q*", "a/*"));
    assert!(!glob_match_with_brace("*q*", "abc"));
    assert!(!glob_match_with_brace("*q*", "abd"));
    assert!(!glob_match_with_brace("*q*", "abe"));
    assert!(!glob_match_with_brace("*q*", "b"));
    assert!(!glob_match_with_brace("*q*", "bb"));
    assert!(!glob_match_with_brace("*q*", "bcd"));
    assert!(!glob_match_with_brace("*q*", "bdir/"));
    assert!(!glob_match_with_brace("*q*", "Beware"));
    assert!(!glob_match_with_brace("*q*", "c"));
    assert!(!glob_match_with_brace("*q*", "ca"));
    assert!(!glob_match_with_brace("*q*", "cb"));
    assert!(!glob_match_with_brace("*q*", "d"));
    assert!(!glob_match_with_brace("*q*", "dd"));
    assert!(!glob_match_with_brace("*q*", "de"));

    assert!(glob_match_with_brace("\\**", "*"));
    assert!(glob_match_with_brace("\\**", "**"));
    assert!(!glob_match_with_brace("\\**", "\\*"));
    assert!(!glob_match_with_brace("\\**", "a"));
    assert!(!glob_match_with_brace("\\**", "a/*"));
    assert!(!glob_match_with_brace("\\**", "abc"));
    assert!(!glob_match_with_brace("\\**", "abd"));
    assert!(!glob_match_with_brace("\\**", "abe"));
    assert!(!glob_match_with_brace("\\**", "b"));
    assert!(!glob_match_with_brace("\\**", "bb"));
    assert!(!glob_match_with_brace("\\**", "bcd"));
    assert!(!glob_match_with_brace("\\**", "bdir/"));
    assert!(!glob_match_with_brace("\\**", "Beware"));
    assert!(!glob_match_with_brace("\\**", "c"));
    assert!(!glob_match_with_brace("\\**", "ca"));
    assert!(!glob_match_with_brace("\\**", "cb"));
    assert!(!glob_match_with_brace("\\**", "d"));
    assert!(!glob_match_with_brace("\\**", "dd"));
    assert!(!glob_match_with_brace("\\**", "de"));
  }

  #[test]
  fn bash_classes() {
    assert!(!glob_match_with_brace("a*[^c]", "*"));
    assert!(!glob_match_with_brace("a*[^c]", "**"));
    assert!(!glob_match_with_brace("a*[^c]", "\\*"));
    assert!(!glob_match_with_brace("a*[^c]", "a"));
    assert!(!glob_match_with_brace("a*[^c]", "a/*"));
    assert!(!glob_match_with_brace("a*[^c]", "abc"));
    assert!(glob_match_with_brace("a*[^c]", "abd"));
    assert!(glob_match_with_brace("a*[^c]", "abe"));
    assert!(!glob_match_with_brace("a*[^c]", "b"));
    assert!(!glob_match_with_brace("a*[^c]", "bb"));
    assert!(!glob_match_with_brace("a*[^c]", "bcd"));
    assert!(!glob_match_with_brace("a*[^c]", "bdir/"));
    assert!(!glob_match_with_brace("a*[^c]", "Beware"));
    assert!(!glob_match_with_brace("a*[^c]", "c"));
    assert!(!glob_match_with_brace("a*[^c]", "ca"));
    assert!(!glob_match_with_brace("a*[^c]", "cb"));
    assert!(!glob_match_with_brace("a*[^c]", "d"));
    assert!(!glob_match_with_brace("a*[^c]", "dd"));
    assert!(!glob_match_with_brace("a*[^c]", "de"));
    assert!(!glob_match_with_brace("a*[^c]", "baz"));
    assert!(!glob_match_with_brace("a*[^c]", "bzz"));
    assert!(!glob_match_with_brace("a*[^c]", "BZZ"));
    assert!(!glob_match_with_brace("a*[^c]", "beware"));
    assert!(!glob_match_with_brace("a*[^c]", "BewAre"));

    assert!(glob_match_with_brace("a[X-]b", "a-b"));
    assert!(glob_match_with_brace("a[X-]b", "aXb"));

    assert!(!glob_match_with_brace("[a-y]*[^c]", "*"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "a*"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "**"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "\\*"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "a"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "a123b"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "a123c"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "ab"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "a/*"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "abc"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "abd"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "abe"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "b"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "bd"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "bb"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "bcd"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "bdir/"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "Beware"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "c"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "ca"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "cb"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "d"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "dd"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "dd"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "dd"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "de"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "baz"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "bzz"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "bzz"));
    // assert(!isMatch('bzz', '[a-y]*[^c]', { regex: true }));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "BZZ"));
    assert!(glob_match_with_brace("[a-y]*[^c]", "beware"));
    assert!(!glob_match_with_brace("[a-y]*[^c]", "BewAre"));

    assert!(glob_match_with_brace("a\\*b/*", "a*b/ooo"));
    assert!(glob_match_with_brace("a\\*?/*", "a*b/ooo"));

    assert!(!glob_match_with_brace("a[b]c", "*"));
    assert!(!glob_match_with_brace("a[b]c", "**"));
    assert!(!glob_match_with_brace("a[b]c", "\\*"));
    assert!(!glob_match_with_brace("a[b]c", "a"));
    assert!(!glob_match_with_brace("a[b]c", "a/*"));
    assert!(glob_match_with_brace("a[b]c", "abc"));
    assert!(!glob_match_with_brace("a[b]c", "abd"));
    assert!(!glob_match_with_brace("a[b]c", "abe"));
    assert!(!glob_match_with_brace("a[b]c", "b"));
    assert!(!glob_match_with_brace("a[b]c", "bb"));
    assert!(!glob_match_with_brace("a[b]c", "bcd"));
    assert!(!glob_match_with_brace("a[b]c", "bdir/"));
    assert!(!glob_match_with_brace("a[b]c", "Beware"));
    assert!(!glob_match_with_brace("a[b]c", "c"));
    assert!(!glob_match_with_brace("a[b]c", "ca"));
    assert!(!glob_match_with_brace("a[b]c", "cb"));
    assert!(!glob_match_with_brace("a[b]c", "d"));
    assert!(!glob_match_with_brace("a[b]c", "dd"));
    assert!(!glob_match_with_brace("a[b]c", "de"));
    assert!(!glob_match_with_brace("a[b]c", "baz"));
    assert!(!glob_match_with_brace("a[b]c", "bzz"));
    assert!(!glob_match_with_brace("a[b]c", "BZZ"));
    assert!(!glob_match_with_brace("a[b]c", "beware"));
    assert!(!glob_match_with_brace("a[b]c", "BewAre"));

    assert!(!glob_match_with_brace("a[\"b\"]c", "*"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "**"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "\\*"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "a"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "a/*"));
    assert!(glob_match_with_brace("a[\"b\"]c", "abc"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "abd"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "abe"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "b"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "bb"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "bcd"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "bdir/"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "Beware"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "c"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "ca"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "cb"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "d"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "dd"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "de"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "baz"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "bzz"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "BZZ"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "beware"));
    assert!(!glob_match_with_brace("a[\"b\"]c", "BewAre"));

    assert!(!glob_match_with_brace("a[\\\\b]c", "*"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "**"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "\\*"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "a"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "a/*"));
    assert!(glob_match_with_brace("a[\\\\b]c", "abc"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "abd"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "abe"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "b"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "bb"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "bcd"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "bdir/"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "Beware"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "c"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "ca"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "cb"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "d"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "dd"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "de"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "baz"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "bzz"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "BZZ"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "beware"));
    assert!(!glob_match_with_brace("a[\\\\b]c", "BewAre"));

    assert!(!glob_match_with_brace("a[\\b]c", "*"));
    assert!(!glob_match_with_brace("a[\\b]c", "**"));
    assert!(!glob_match_with_brace("a[\\b]c", "\\*"));
    assert!(!glob_match_with_brace("a[\\b]c", "a"));
    assert!(!glob_match_with_brace("a[\\b]c", "a/*"));
    assert!(!glob_match_with_brace("a[\\b]c", "abc"));
    assert!(!glob_match_with_brace("a[\\b]c", "abd"));
    assert!(!glob_match_with_brace("a[\\b]c", "abe"));
    assert!(!glob_match_with_brace("a[\\b]c", "b"));
    assert!(!glob_match_with_brace("a[\\b]c", "bb"));
    assert!(!glob_match_with_brace("a[\\b]c", "bcd"));
    assert!(!glob_match_with_brace("a[\\b]c", "bdir/"));
    assert!(!glob_match_with_brace("a[\\b]c", "Beware"));
    assert!(!glob_match_with_brace("a[\\b]c", "c"));
    assert!(!glob_match_with_brace("a[\\b]c", "ca"));
    assert!(!glob_match_with_brace("a[\\b]c", "cb"));
    assert!(!glob_match_with_brace("a[\\b]c", "d"));
    assert!(!glob_match_with_brace("a[\\b]c", "dd"));
    assert!(!glob_match_with_brace("a[\\b]c", "de"));
    assert!(!glob_match_with_brace("a[\\b]c", "baz"));
    assert!(!glob_match_with_brace("a[\\b]c", "bzz"));
    assert!(!glob_match_with_brace("a[\\b]c", "BZZ"));
    assert!(!glob_match_with_brace("a[\\b]c", "beware"));
    assert!(!glob_match_with_brace("a[\\b]c", "BewAre"));

    assert!(!glob_match_with_brace("a[b-d]c", "*"));
    assert!(!glob_match_with_brace("a[b-d]c", "**"));
    assert!(!glob_match_with_brace("a[b-d]c", "\\*"));
    assert!(!glob_match_with_brace("a[b-d]c", "a"));
    assert!(!glob_match_with_brace("a[b-d]c", "a/*"));
    assert!(glob_match_with_brace("a[b-d]c", "abc"));
    assert!(!glob_match_with_brace("a[b-d]c", "abd"));
    assert!(!glob_match_with_brace("a[b-d]c", "abe"));
    assert!(!glob_match_with_brace("a[b-d]c", "b"));
    assert!(!glob_match_with_brace("a[b-d]c", "bb"));
    assert!(!glob_match_with_brace("a[b-d]c", "bcd"));
    assert!(!glob_match_with_brace("a[b-d]c", "bdir/"));
    assert!(!glob_match_with_brace("a[b-d]c", "Beware"));
    assert!(!glob_match_with_brace("a[b-d]c", "c"));
    assert!(!glob_match_with_brace("a[b-d]c", "ca"));
    assert!(!glob_match_with_brace("a[b-d]c", "cb"));
    assert!(!glob_match_with_brace("a[b-d]c", "d"));
    assert!(!glob_match_with_brace("a[b-d]c", "dd"));
    assert!(!glob_match_with_brace("a[b-d]c", "de"));
    assert!(!glob_match_with_brace("a[b-d]c", "baz"));
    assert!(!glob_match_with_brace("a[b-d]c", "bzz"));
    assert!(!glob_match_with_brace("a[b-d]c", "BZZ"));
    assert!(!glob_match_with_brace("a[b-d]c", "beware"));
    assert!(!glob_match_with_brace("a[b-d]c", "BewAre"));

    assert!(!glob_match_with_brace("a?c", "*"));
    assert!(!glob_match_with_brace("a?c", "**"));
    assert!(!glob_match_with_brace("a?c", "\\*"));
    assert!(!glob_match_with_brace("a?c", "a"));
    assert!(!glob_match_with_brace("a?c", "a/*"));
    assert!(glob_match_with_brace("a?c", "abc"));
    assert!(!glob_match_with_brace("a?c", "abd"));
    assert!(!glob_match_with_brace("a?c", "abe"));
    assert!(!glob_match_with_brace("a?c", "b"));
    assert!(!glob_match_with_brace("a?c", "bb"));
    assert!(!glob_match_with_brace("a?c", "bcd"));
    assert!(!glob_match_with_brace("a?c", "bdir/"));
    assert!(!glob_match_with_brace("a?c", "Beware"));
    assert!(!glob_match_with_brace("a?c", "c"));
    assert!(!glob_match_with_brace("a?c", "ca"));
    assert!(!glob_match_with_brace("a?c", "cb"));
    assert!(!glob_match_with_brace("a?c", "d"));
    assert!(!glob_match_with_brace("a?c", "dd"));
    assert!(!glob_match_with_brace("a?c", "de"));
    assert!(!glob_match_with_brace("a?c", "baz"));
    assert!(!glob_match_with_brace("a?c", "bzz"));
    assert!(!glob_match_with_brace("a?c", "BZZ"));
    assert!(!glob_match_with_brace("a?c", "beware"));
    assert!(!glob_match_with_brace("a?c", "BewAre"));

    assert!(glob_match_with_brace("*/man*/bash.*", "man/man1/bash.1"));

    assert!(glob_match_with_brace("[^a-c]*", "*"));
    assert!(glob_match_with_brace("[^a-c]*", "**"));
    assert!(!glob_match_with_brace("[^a-c]*", "a"));
    assert!(!glob_match_with_brace("[^a-c]*", "a/*"));
    assert!(!glob_match_with_brace("[^a-c]*", "abc"));
    assert!(!glob_match_with_brace("[^a-c]*", "abd"));
    assert!(!glob_match_with_brace("[^a-c]*", "abe"));
    assert!(!glob_match_with_brace("[^a-c]*", "b"));
    assert!(!glob_match_with_brace("[^a-c]*", "bb"));
    assert!(!glob_match_with_brace("[^a-c]*", "bcd"));
    assert!(!glob_match_with_brace("[^a-c]*", "bdir/"));
    assert!(glob_match_with_brace("[^a-c]*", "Beware"));
    assert!(glob_match_with_brace("[^a-c]*", "Beware"));
    assert!(!glob_match_with_brace("[^a-c]*", "c"));
    assert!(!glob_match_with_brace("[^a-c]*", "ca"));
    assert!(!glob_match_with_brace("[^a-c]*", "cb"));
    assert!(glob_match_with_brace("[^a-c]*", "d"));
    assert!(glob_match_with_brace("[^a-c]*", "dd"));
    assert!(glob_match_with_brace("[^a-c]*", "de"));
    assert!(!glob_match_with_brace("[^a-c]*", "baz"));
    assert!(!glob_match_with_brace("[^a-c]*", "bzz"));
    assert!(glob_match_with_brace("[^a-c]*", "BZZ"));
    assert!(!glob_match_with_brace("[^a-c]*", "beware"));
    assert!(glob_match_with_brace("[^a-c]*", "BewAre"));
  }

  #[test]
  fn bash_wildmatch() {
    assert!(!glob_match_with_brace("a[]-]b", "aab"));
    assert!(!glob_match_with_brace("[ten]", "ten"));
    assert!(glob_match_with_brace("]", "]"));
    assert!(glob_match_with_brace("a[]-]b", "a-b"));
    assert!(glob_match_with_brace("a[]-]b", "a]b"));
    assert!(glob_match_with_brace("a[]]b", "a]b"));
    assert!(glob_match_with_brace("a[\\]a\\-]b", "aab"));
    assert!(glob_match_with_brace("t[a-g]n", "ten"));
    assert!(glob_match_with_brace("t[^a-g]n", "ton"));
  }

  #[test]
  fn bash_slashmatch() {
    // assert!(!glob_match_with_brace("f[^eiu][^eiu][^eiu][^eiu][^eiu]r", "foo/bar"));
    assert!(glob_match_with_brace("foo[/]bar", "foo/bar"));
    assert!(glob_match_with_brace(
      "f[^eiu][^eiu][^eiu][^eiu][^eiu]r",
      "foo-bar"
    ));
  }

  #[test]
  fn bash_extra_stars() {
    assert!(!glob_match_with_brace("a**c", "bbc"));
    assert!(glob_match_with_brace("a**c", "abc"));
    assert!(!glob_match_with_brace("a**c", "bbd"));

    assert!(!glob_match_with_brace("a***c", "bbc"));
    assert!(glob_match_with_brace("a***c", "abc"));
    assert!(!glob_match_with_brace("a***c", "bbd"));

    assert!(!glob_match_with_brace("a*****?c", "bbc"));
    assert!(glob_match_with_brace("a*****?c", "abc"));
    assert!(!glob_match_with_brace("a*****?c", "bbc"));

    assert!(glob_match_with_brace("?*****??", "bbc"));
    assert!(glob_match_with_brace("?*****??", "abc"));

    assert!(glob_match_with_brace("*****??", "bbc"));
    assert!(glob_match_with_brace("*****??", "abc"));

    assert!(glob_match_with_brace("?*****?c", "bbc"));
    assert!(glob_match_with_brace("?*****?c", "abc"));

    assert!(glob_match_with_brace("?***?****c", "bbc"));
    assert!(glob_match_with_brace("?***?****c", "abc"));
    assert!(!glob_match_with_brace("?***?****c", "bbd"));

    assert!(glob_match_with_brace("?***?****?", "bbc"));
    assert!(glob_match_with_brace("?***?****?", "abc"));

    assert!(glob_match_with_brace("?***?****", "bbc"));
    assert!(glob_match_with_brace("?***?****", "abc"));

    assert!(glob_match_with_brace("*******c", "bbc"));
    assert!(glob_match_with_brace("*******c", "abc"));

    assert!(glob_match_with_brace("*******?", "bbc"));
    assert!(glob_match_with_brace("*******?", "abc"));

    assert!(glob_match_with_brace("a*cd**?**??k", "abcdecdhjk"));
    assert!(glob_match_with_brace("a**?**cd**?**??k", "abcdecdhjk"));
    assert!(glob_match_with_brace("a**?**cd**?**??k***", "abcdecdhjk"));
    assert!(glob_match_with_brace("a**?**cd**?**??***k", "abcdecdhjk"));
    assert!(glob_match_with_brace("a**?**cd**?**??***k**", "abcdecdhjk"));
    assert!(glob_match_with_brace("a****c**?**??*****", "abcdecdhjk"));
  }

  #[test]
  fn stars() {
    assert!(!glob_match_with_brace("*.js", "a/b/c/z.js"));
    assert!(!glob_match_with_brace("*.js", "a/b/z.js"));
    assert!(!glob_match_with_brace("*.js", "a/z.js"));
    assert!(glob_match_with_brace("*.js", "z.js"));

    // assert!(!glob_match_with_brace("*/*", "a/.ab"));
    // assert!(!glob_match_with_brace("*", ".ab"));

    assert!(glob_match_with_brace("z*.js", "z.js"));
    assert!(glob_match_with_brace("*/*", "a/z"));
    assert!(glob_match_with_brace("*/z*.js", "a/z.js"));
    assert!(glob_match_with_brace("a/z*.js", "a/z.js"));

    assert!(glob_match_with_brace("*", "ab"));
    assert!(glob_match_with_brace("*", "abc"));

    assert!(!glob_match_with_brace("f*", "bar"));
    assert!(!glob_match_with_brace("*r", "foo"));
    assert!(!glob_match_with_brace("b*", "foo"));
    assert!(!glob_match_with_brace("*", "foo/bar"));
    assert!(glob_match_with_brace("*c", "abc"));
    assert!(glob_match_with_brace("a*", "abc"));
    assert!(glob_match_with_brace("a*c", "abc"));
    assert!(glob_match_with_brace("*r", "bar"));
    assert!(glob_match_with_brace("b*", "bar"));
    assert!(glob_match_with_brace("f*", "foo"));

    assert!(glob_match_with_brace("*abc*", "one abc two"));
    assert!(glob_match_with_brace("a*b", "a         b"));

    assert!(!glob_match_with_brace("*a*", "foo"));
    assert!(glob_match_with_brace("*a*", "bar"));
    assert!(glob_match_with_brace("*abc*", "oneabctwo"));
    assert!(!glob_match_with_brace("*-bc-*", "a-b.c-d"));
    assert!(glob_match_with_brace("*-*.*-*", "a-b.c-d"));
    assert!(glob_match_with_brace("*-b*c-*", "a-b.c-d"));
    assert!(glob_match_with_brace("*-b.c-*", "a-b.c-d"));
    assert!(glob_match_with_brace("*.*", "a-b.c-d"));
    assert!(glob_match_with_brace("*.*-*", "a-b.c-d"));
    assert!(glob_match_with_brace("*.*-d", "a-b.c-d"));
    assert!(glob_match_with_brace("*.c-*", "a-b.c-d"));
    assert!(glob_match_with_brace("*b.*d", "a-b.c-d"));
    assert!(glob_match_with_brace("a*.c*", "a-b.c-d"));
    assert!(glob_match_with_brace("a-*.*-d", "a-b.c-d"));
    assert!(glob_match_with_brace("*.*", "a.b"));
    assert!(glob_match_with_brace("*.b", "a.b"));
    assert!(glob_match_with_brace("a.*", "a.b"));
    assert!(glob_match_with_brace("a.b", "a.b"));

    assert!(!glob_match_with_brace("**-bc-**", "a-b.c-d"));
    assert!(glob_match_with_brace("**-**.**-**", "a-b.c-d"));
    assert!(glob_match_with_brace("**-b**c-**", "a-b.c-d"));
    assert!(glob_match_with_brace("**-b.c-**", "a-b.c-d"));
    assert!(glob_match_with_brace("**.**", "a-b.c-d"));
    assert!(glob_match_with_brace("**.**-**", "a-b.c-d"));
    assert!(glob_match_with_brace("**.**-d", "a-b.c-d"));
    assert!(glob_match_with_brace("**.c-**", "a-b.c-d"));
    assert!(glob_match_with_brace("**b.**d", "a-b.c-d"));
    assert!(glob_match_with_brace("a**.c**", "a-b.c-d"));
    assert!(glob_match_with_brace("a-**.**-d", "a-b.c-d"));
    assert!(glob_match_with_brace("**.**", "a.b"));
    assert!(glob_match_with_brace("**.b", "a.b"));
    assert!(glob_match_with_brace("a.**", "a.b"));
    assert!(glob_match_with_brace("a.b", "a.b"));

    assert!(glob_match_with_brace("*/*", "/ab"));
    assert!(glob_match_with_brace(".", "."));
    assert!(!glob_match_with_brace("a/", "a/.b"));
    assert!(glob_match_with_brace("/*", "/ab"));
    assert!(glob_match_with_brace("/??", "/ab"));
    assert!(glob_match_with_brace("/?b", "/ab"));
    assert!(glob_match_with_brace("/*", "/cd"));
    assert!(glob_match_with_brace("a", "a"));
    assert!(glob_match_with_brace("a/.*", "a/.b"));
    assert!(glob_match_with_brace("?/?", "a/b"));
    assert!(glob_match_with_brace(
      "a/**/j/**/z/*.md",
      "a/b/c/d/e/j/n/p/o/z/c.md"
    ));
    assert!(glob_match_with_brace("a/**/z/*.md", "a/b/c/d/e/z/c.md"));
    assert!(glob_match_with_brace("a/b/c/*.md", "a/b/c/xyz.md"));
    assert!(glob_match_with_brace("a/b/c/*.md", "a/b/c/xyz.md"));
    assert!(glob_match_with_brace("a/*/z/.a", "a/b/z/.a"));
    assert!(!glob_match_with_brace("bz", "a/b/z/.a"));
    assert!(glob_match_with_brace(
      "a/**/c/*.md",
      "a/bb.bb/aa/b.b/aa/c/xyz.md"
    ));
    assert!(glob_match_with_brace(
      "a/**/c/*.md",
      "a/bb.bb/aa/bb/aa/c/xyz.md"
    ));
    assert!(glob_match_with_brace("a/*/c/*.md", "a/bb.bb/c/xyz.md"));
    assert!(glob_match_with_brace("a/*/c/*.md", "a/bb/c/xyz.md"));
    assert!(glob_match_with_brace("a/*/c/*.md", "a/bbbb/c/xyz.md"));
    assert!(glob_match_with_brace("*", "aaa"));
    assert!(glob_match_with_brace("*", "ab"));
    assert!(glob_match_with_brace("ab", "ab"));

    assert!(!glob_match_with_brace("*/*/*", "aaa"));
    assert!(!glob_match_with_brace("*/*/*", "aaa/bb/aa/rr"));
    assert!(!glob_match_with_brace("aaa*", "aaa/bba/ccc"));
    // assert!(!glob_match_with_brace("aaa**", "aaa/bba/ccc"));
    assert!(!glob_match_with_brace("aaa/*", "aaa/bba/ccc"));
    assert!(!glob_match_with_brace("aaa/*ccc", "aaa/bba/ccc"));
    assert!(!glob_match_with_brace("aaa/*z", "aaa/bba/ccc"));
    assert!(!glob_match_with_brace("*/*/*", "aaa/bbb"));
    assert!(!glob_match_with_brace("*/*jk*/*i", "ab/zzz/ejkl/hi"));
    assert!(glob_match_with_brace("*/*/*", "aaa/bba/ccc"));
    assert!(glob_match_with_brace("aaa/**", "aaa/bba/ccc"));
    assert!(glob_match_with_brace("aaa/*", "aaa/bbb"));
    assert!(glob_match_with_brace("*/*z*/*/*i", "ab/zzz/ejkl/hi"));
    assert!(glob_match_with_brace("*j*i", "abzzzejklhi"));

    assert!(glob_match_with_brace("*", "a"));
    assert!(glob_match_with_brace("*", "b"));
    assert!(!glob_match_with_brace("*", "a/a"));
    assert!(!glob_match_with_brace("*", "a/a/a"));
    assert!(!glob_match_with_brace("*", "a/a/b"));
    assert!(!glob_match_with_brace("*", "a/a/a/a"));
    assert!(!glob_match_with_brace("*", "a/a/a/a/a"));

    assert!(!glob_match_with_brace("*/*", "a"));
    assert!(glob_match_with_brace("*/*", "a/a"));
    assert!(!glob_match_with_brace("*/*", "a/a/a"));

    assert!(!glob_match_with_brace("*/*/*", "a"));
    assert!(!glob_match_with_brace("*/*/*", "a/a"));
    assert!(glob_match_with_brace("*/*/*", "a/a/a"));
    assert!(!glob_match_with_brace("*/*/*", "a/a/a/a"));

    assert!(!glob_match_with_brace("*/*/*/*", "a"));
    assert!(!glob_match_with_brace("*/*/*/*", "a/a"));
    assert!(!glob_match_with_brace("*/*/*/*", "a/a/a"));
    assert!(glob_match_with_brace("*/*/*/*", "a/a/a/a"));
    assert!(!glob_match_with_brace("*/*/*/*", "a/a/a/a/a"));

    assert!(!glob_match_with_brace("*/*/*/*/*", "a"));
    assert!(!glob_match_with_brace("*/*/*/*/*", "a/a"));
    assert!(!glob_match_with_brace("*/*/*/*/*", "a/a/a"));
    assert!(!glob_match_with_brace("*/*/*/*/*", "a/a/b"));
    assert!(!glob_match_with_brace("*/*/*/*/*", "a/a/a/a"));
    assert!(glob_match_with_brace("*/*/*/*/*", "a/a/a/a/a"));
    assert!(!glob_match_with_brace("*/*/*/*/*", "a/a/a/a/a/a"));

    assert!(!glob_match_with_brace("a/*", "a"));
    assert!(glob_match_with_brace("a/*", "a/a"));
    assert!(!glob_match_with_brace("a/*", "a/a/a"));
    assert!(!glob_match_with_brace("a/*", "a/a/a/a"));
    assert!(!glob_match_with_brace("a/*", "a/a/a/a/a"));

    assert!(!glob_match_with_brace("a/*/*", "a"));
    assert!(!glob_match_with_brace("a/*/*", "a/a"));
    assert!(glob_match_with_brace("a/*/*", "a/a/a"));
    assert!(!glob_match_with_brace("a/*/*", "b/a/a"));
    assert!(!glob_match_with_brace("a/*/*", "a/a/a/a"));
    assert!(!glob_match_with_brace("a/*/*", "a/a/a/a/a"));

    assert!(!glob_match_with_brace("a/*/*/*", "a"));
    assert!(!glob_match_with_brace("a/*/*/*", "a/a"));
    assert!(!glob_match_with_brace("a/*/*/*", "a/a/a"));
    assert!(glob_match_with_brace("a/*/*/*", "a/a/a/a"));
    assert!(!glob_match_with_brace("a/*/*/*", "a/a/a/a/a"));

    assert!(!glob_match_with_brace("a/*/*/*/*", "a"));
    assert!(!glob_match_with_brace("a/*/*/*/*", "a/a"));
    assert!(!glob_match_with_brace("a/*/*/*/*", "a/a/a"));
    assert!(!glob_match_with_brace("a/*/*/*/*", "a/a/b"));
    assert!(!glob_match_with_brace("a/*/*/*/*", "a/a/a/a"));
    assert!(glob_match_with_brace("a/*/*/*/*", "a/a/a/a/a"));

    assert!(!glob_match_with_brace("a/*/a", "a"));
    assert!(!glob_match_with_brace("a/*/a", "a/a"));
    assert!(glob_match_with_brace("a/*/a", "a/a/a"));
    assert!(!glob_match_with_brace("a/*/a", "a/a/b"));
    assert!(!glob_match_with_brace("a/*/a", "a/a/a/a"));
    assert!(!glob_match_with_brace("a/*/a", "a/a/a/a/a"));

    assert!(!glob_match_with_brace("a/*/b", "a"));
    assert!(!glob_match_with_brace("a/*/b", "a/a"));
    assert!(!glob_match_with_brace("a/*/b", "a/a/a"));
    assert!(glob_match_with_brace("a/*/b", "a/a/b"));
    assert!(!glob_match_with_brace("a/*/b", "a/a/a/a"));
    assert!(!glob_match_with_brace("a/*/b", "a/a/a/a/a"));

    assert!(!glob_match_with_brace("*/**/a", "a"));
    assert!(!glob_match_with_brace("*/**/a", "a/a/b"));
    assert!(glob_match_with_brace("*/**/a", "a/a"));
    assert!(glob_match_with_brace("*/**/a", "a/a/a"));
    assert!(glob_match_with_brace("*/**/a", "a/a/a/a"));
    assert!(glob_match_with_brace("*/**/a", "a/a/a/a/a"));

    assert!(!glob_match_with_brace("*/", "a"));
    assert!(!glob_match_with_brace("*/*", "a"));
    assert!(!glob_match_with_brace("a/*", "a"));
    // assert!(!glob_match_with_brace("*/*", "a/"));
    // assert!(!glob_match_with_brace("a/*", "a/"));
    assert!(!glob_match_with_brace("*", "a/a"));
    assert!(!glob_match_with_brace("*/", "a/a"));
    assert!(!glob_match_with_brace("*/", "a/x/y"));
    assert!(!glob_match_with_brace("*/*", "a/x/y"));
    assert!(!glob_match_with_brace("a/*", "a/x/y"));
    // assert!(glob_match_with_brace("*", "a/"));
    assert!(glob_match_with_brace("*", "a"));
    assert!(glob_match_with_brace("*/", "a/"));
    assert!(glob_match_with_brace("*{,/}", "a/"));
    assert!(glob_match_with_brace("*/*", "a/a"));
    assert!(glob_match_with_brace("a/*", "a/a"));

    assert!(!glob_match_with_brace("a/**/*.txt", "a.txt"));
    assert!(glob_match_with_brace("a/**/*.txt", "a/x/y.txt"));
    assert!(!glob_match_with_brace("a/**/*.txt", "a/x/y/z"));

    assert!(!glob_match_with_brace("a/*.txt", "a.txt"));
    assert!(glob_match_with_brace("a/*.txt", "a/b.txt"));
    assert!(!glob_match_with_brace("a/*.txt", "a/x/y.txt"));
    assert!(!glob_match_with_brace("a/*.txt", "a/x/y/z"));

    assert!(glob_match_with_brace("a*.txt", "a.txt"));
    assert!(!glob_match_with_brace("a*.txt", "a/b.txt"));
    assert!(!glob_match_with_brace("a*.txt", "a/x/y.txt"));
    assert!(!glob_match_with_brace("a*.txt", "a/x/y/z"));

    assert!(glob_match_with_brace("*.txt", "a.txt"));
    assert!(!glob_match_with_brace("*.txt", "a/b.txt"));
    assert!(!glob_match_with_brace("*.txt", "a/x/y.txt"));
    assert!(!glob_match_with_brace("*.txt", "a/x/y/z"));

    assert!(!glob_match_with_brace("a*", "a/b"));
    assert!(!glob_match_with_brace("a/**/b", "a/a/bb"));
    assert!(!glob_match_with_brace("a/**/b", "a/bb"));

    assert!(!glob_match_with_brace("*/**", "foo"));
    assert!(!glob_match_with_brace("**/", "foo/bar"));
    assert!(!glob_match_with_brace("**/*/", "foo/bar"));
    assert!(!glob_match_with_brace("*/*/", "foo/bar"));

    assert!(glob_match_with_brace("**/..", "/home/foo/.."));
    assert!(glob_match_with_brace("**/a", "a"));
    assert!(glob_match_with_brace("**", "a/a"));
    assert!(glob_match_with_brace("a/**", "a/a"));
    assert!(glob_match_with_brace("a/**", "a/"));
    // assert!(glob_match_with_brace("a/**", "a"));
    assert!(!glob_match_with_brace("**/", "a/a"));
    // assert!(glob_match_with_brace("**/a/**", "a"));
    // assert!(glob_match_with_brace("a/**", "a"));
    assert!(!glob_match_with_brace("**/", "a/a"));
    assert!(glob_match_with_brace("*/**/a", "a/a"));
    // assert!(glob_match_with_brace("a/**", "a"));
    assert!(glob_match_with_brace("*/**", "foo/"));
    assert!(glob_match_with_brace("**/*", "foo/bar"));
    assert!(glob_match_with_brace("*/*", "foo/bar"));
    assert!(glob_match_with_brace("*/**", "foo/bar"));
    assert!(glob_match_with_brace("**/", "foo/bar/"));
    // assert!(glob_match_with_brace("**/*", "foo/bar/"));
    assert!(glob_match_with_brace("**/*/", "foo/bar/"));
    assert!(glob_match_with_brace("*/**", "foo/bar/"));
    assert!(glob_match_with_brace("*/*/", "foo/bar/"));

    assert!(!glob_match_with_brace("*/foo", "bar/baz/foo"));
    assert!(!glob_match_with_brace("**/bar/*", "deep/foo/bar"));
    assert!(!glob_match_with_brace("*/bar/**", "deep/foo/bar/baz/x"));
    assert!(!glob_match_with_brace("/*", "ef"));
    assert!(!glob_match_with_brace("foo?bar", "foo/bar"));
    assert!(!glob_match_with_brace("**/bar*", "foo/bar/baz"));
    // assert!(!glob_match_with_brace("**/bar**", "foo/bar/baz"));
    assert!(!glob_match_with_brace("foo**bar", "foo/baz/bar"));
    assert!(!glob_match_with_brace("foo*bar", "foo/baz/bar"));
    // assert!(glob_match_with_brace("foo/**", "foo"));
    assert!(glob_match_with_brace("/*", "/ab"));
    assert!(glob_match_with_brace("/*", "/cd"));
    assert!(glob_match_with_brace("/*", "/ef"));
    assert!(glob_match_with_brace("a/**/j/**/z/*.md", "a/b/j/c/z/x.md"));
    assert!(glob_match_with_brace("a/**/j/**/z/*.md", "a/j/z/x.md"));

    assert!(glob_match_with_brace("**/foo", "bar/baz/foo"));
    assert!(glob_match_with_brace("**/bar/*", "deep/foo/bar/baz"));
    assert!(glob_match_with_brace("**/bar/**", "deep/foo/bar/baz/"));
    assert!(glob_match_with_brace("**/bar/*/*", "deep/foo/bar/baz/x"));
    assert!(glob_match_with_brace("foo/**/**/bar", "foo/b/a/z/bar"));
    assert!(glob_match_with_brace("foo/**/bar", "foo/b/a/z/bar"));
    assert!(glob_match_with_brace("foo/**/**/bar", "foo/bar"));
    assert!(glob_match_with_brace("foo/**/bar", "foo/bar"));
    assert!(glob_match_with_brace("*/bar/**", "foo/bar/baz/x"));
    assert!(glob_match_with_brace("foo/**/**/bar", "foo/baz/bar"));
    assert!(glob_match_with_brace("foo/**/bar", "foo/baz/bar"));
    assert!(glob_match_with_brace("**/foo", "XXX/foo"));
  }

  #[test]
  fn globstars() {
    assert!(glob_match_with_brace("**/*.js", "a/b/c/d.js"));
    assert!(glob_match_with_brace("**/*.js", "a/b/c.js"));
    assert!(glob_match_with_brace("**/*.js", "a/b.js"));
    assert!(glob_match_with_brace("a/b/**/*.js", "a/b/c/d/e/f.js"));
    assert!(glob_match_with_brace("a/b/**/*.js", "a/b/c/d/e.js"));
    assert!(glob_match_with_brace("a/b/c/**/*.js", "a/b/c/d.js"));
    assert!(glob_match_with_brace("a/b/**/*.js", "a/b/c/d.js"));
    assert!(glob_match_with_brace("a/b/**/*.js", "a/b/d.js"));
    assert!(!glob_match_with_brace("a/b/**/*.js", "a/d.js"));
    assert!(!glob_match_with_brace("a/b/**/*.js", "d.js"));

    assert!(!glob_match_with_brace("**c", "a/b/c"));
    assert!(!glob_match_with_brace("a/**c", "a/b/c"));
    assert!(!glob_match_with_brace("a/**z", "a/b/c"));
    assert!(!glob_match_with_brace("a/**b**/c", "a/b/c/b/c"));
    assert!(!glob_match_with_brace("a/b/c**/*.js", "a/b/c/d/e.js"));
    assert!(glob_match_with_brace("a/**/b/**/c", "a/b/c/b/c"));
    assert!(glob_match_with_brace("a/**b**/c", "a/aba/c"));
    assert!(glob_match_with_brace("a/**b**/c", "a/b/c"));
    assert!(glob_match_with_brace("a/b/c**/*.js", "a/b/c/d.js"));

    assert!(!glob_match_with_brace("a/**/*", "a"));
    assert!(!glob_match_with_brace("a/**/**/*", "a"));
    assert!(!glob_match_with_brace("a/**/**/**/*", "a"));
    assert!(!glob_match_with_brace("**/a", "a/"));
    assert!(glob_match_with_brace("a/**/*", "a/"));
    assert!(glob_match_with_brace("a/**/**/*", "a/"));
    assert!(glob_match_with_brace("a/**/**/**/*", "a/"));
    assert!(!glob_match_with_brace("**/a", "a/b"));
    assert!(!glob_match_with_brace(
      "a/**/j/**/z/*.md",
      "a/b/c/j/e/z/c.txt"
    ));
    assert!(!glob_match_with_brace("a/**/b", "a/bb"));
    assert!(!glob_match_with_brace("**/a", "a/c"));
    assert!(!glob_match_with_brace("**/a", "a/b"));
    assert!(!glob_match_with_brace("**/a", "a/x/y"));
    assert!(!glob_match_with_brace("**/a", "a/b/c/d"));
    assert!(glob_match_with_brace("**", "a"));
    assert!(glob_match_with_brace("**/a", "a"));
    // assert!(glob_match_with_brace("a/**", "a"));
    assert!(glob_match_with_brace("**", "a/"));
    assert!(glob_match_with_brace("**/a/**", "a/"));
    assert!(glob_match_with_brace("a/**", "a/"));
    assert!(glob_match_with_brace("a/**/**", "a/"));
    assert!(glob_match_with_brace("**/a", "a/a"));
    assert!(glob_match_with_brace("**", "a/b"));
    assert!(glob_match_with_brace("*/*", "a/b"));
    assert!(glob_match_with_brace("a/**", "a/b"));
    assert!(glob_match_with_brace("a/**/*", "a/b"));
    assert!(glob_match_with_brace("a/**/**/*", "a/b"));
    assert!(glob_match_with_brace("a/**/**/**/*", "a/b"));
    assert!(glob_match_with_brace("a/**/b", "a/b"));
    assert!(glob_match_with_brace("**", "a/b/c"));
    assert!(glob_match_with_brace("**/*", "a/b/c"));
    assert!(glob_match_with_brace("**/**", "a/b/c"));
    assert!(glob_match_with_brace("*/**", "a/b/c"));
    assert!(glob_match_with_brace("a/**", "a/b/c"));
    assert!(glob_match_with_brace("a/**/*", "a/b/c"));
    assert!(glob_match_with_brace("a/**/**/*", "a/b/c"));
    assert!(glob_match_with_brace("a/**/**/**/*", "a/b/c"));
    assert!(glob_match_with_brace("**", "a/b/c/d"));
    assert!(glob_match_with_brace("a/**", "a/b/c/d"));
    assert!(glob_match_with_brace("a/**/*", "a/b/c/d"));
    assert!(glob_match_with_brace("a/**/**/*", "a/b/c/d"));
    assert!(glob_match_with_brace("a/**/**/**/*", "a/b/c/d"));
    assert!(glob_match_with_brace("a/b/**/c/**/*.*", "a/b/c/d.e"));
    assert!(glob_match_with_brace("a/**/f/*.md", "a/b/c/d/e/f/g.md"));
    assert!(glob_match_with_brace(
      "a/**/f/**/k/*.md",
      "a/b/c/d/e/f/g/h/i/j/k/l.md"
    ));
    assert!(glob_match_with_brace("a/b/c/*.md", "a/b/c/def.md"));
    assert!(glob_match_with_brace("a/*/c/*.md", "a/bb.bb/c/ddd.md"));
    assert!(glob_match_with_brace(
      "a/**/f/*.md",
      "a/bb.bb/cc/d.d/ee/f/ggg.md"
    ));
    assert!(glob_match_with_brace(
      "a/**/f/*.md",
      "a/bb.bb/cc/dd/ee/f/ggg.md"
    ));
    assert!(glob_match_with_brace("a/*/c/*.md", "a/bb/c/ddd.md"));
    assert!(glob_match_with_brace("a/*/c/*.md", "a/bbbb/c/ddd.md"));

    assert!(glob_match_with_brace(
      "foo/bar/**/one/**/*.*",
      "foo/bar/baz/one/image.png"
    ));
    assert!(glob_match_with_brace(
      "foo/bar/**/one/**/*.*",
      "foo/bar/baz/one/two/image.png"
    ));
    assert!(glob_match_with_brace(
      "foo/bar/**/one/**/*.*",
      "foo/bar/baz/one/two/three/image.png"
    ));
    assert!(!glob_match_with_brace("a/b/**/f", "a/b/c/d/"));
    // assert!(glob_match_with_brace("a/**", "a"));
    assert!(glob_match_with_brace("**", "a"));
    // assert!(glob_match_with_brace("a{,/**}", "a"));
    assert!(glob_match_with_brace("**", "a/"));
    assert!(glob_match_with_brace("a/**", "a/"));
    assert!(glob_match_with_brace("**", "a/b/c/d"));
    assert!(glob_match_with_brace("**", "a/b/c/d/"));
    assert!(glob_match_with_brace("**/**", "a/b/c/d/"));
    assert!(glob_match_with_brace("**/b/**", "a/b/c/d/"));
    assert!(glob_match_with_brace("a/b/**", "a/b/c/d/"));
    assert!(glob_match_with_brace("a/b/**/", "a/b/c/d/"));
    assert!(glob_match_with_brace("a/b/**/c/**/", "a/b/c/d/"));
    assert!(glob_match_with_brace("a/b/**/c/**/d/", "a/b/c/d/"));
    assert!(glob_match_with_brace("a/b/**/**/*.*", "a/b/c/d/e.f"));
    assert!(glob_match_with_brace("a/b/**/*.*", "a/b/c/d/e.f"));
    assert!(glob_match_with_brace("a/b/**/c/**/d/*.*", "a/b/c/d/e.f"));
    assert!(glob_match_with_brace("a/b/**/d/**/*.*", "a/b/c/d/e.f"));
    assert!(glob_match_with_brace("a/b/**/d/**/*.*", "a/b/c/d/g/e.f"));
    assert!(glob_match_with_brace("a/b/**/d/**/*.*", "a/b/c/d/g/g/e.f"));
    assert!(glob_match_with_brace("a/b-*/**/z.js", "a/b-c/z.js"));
    assert!(glob_match_with_brace("a/b-*/**/z.js", "a/b-c/d/e/z.js"));

    assert!(glob_match_with_brace("*/*", "a/b"));
    assert!(glob_match_with_brace("a/b/c/*.md", "a/b/c/xyz.md"));
    assert!(glob_match_with_brace("a/*/c/*.md", "a/bb.bb/c/xyz.md"));
    assert!(glob_match_with_brace("a/*/c/*.md", "a/bb/c/xyz.md"));
    assert!(glob_match_with_brace("a/*/c/*.md", "a/bbbb/c/xyz.md"));

    assert!(glob_match_with_brace("**/*", "a/b/c"));
    assert!(glob_match_with_brace("**/**", "a/b/c"));
    assert!(glob_match_with_brace("*/**", "a/b/c"));
    assert!(glob_match_with_brace(
      "a/**/j/**/z/*.md",
      "a/b/c/d/e/j/n/p/o/z/c.md"
    ));
    assert!(glob_match_with_brace("a/**/z/*.md", "a/b/c/d/e/z/c.md"));
    assert!(glob_match_with_brace(
      "a/**/c/*.md",
      "a/bb.bb/aa/b.b/aa/c/xyz.md"
    ));
    assert!(glob_match_with_brace(
      "a/**/c/*.md",
      "a/bb.bb/aa/bb/aa/c/xyz.md"
    ));
    assert!(!glob_match_with_brace(
      "a/**/j/**/z/*.md",
      "a/b/c/j/e/z/c.txt"
    ));
    assert!(!glob_match_with_brace(
      "a/b/**/c{d,e}/**/xyz.md",
      "a/b/c/xyz.md"
    ));
    assert!(!glob_match_with_brace(
      "a/b/**/c{d,e}/**/xyz.md",
      "a/b/d/xyz.md"
    ));
    assert!(!glob_match_with_brace("a/**/", "a/b"));
    // assert!(!glob_match_with_brace("**/*", "a/b/.js/c.txt"));
    assert!(!glob_match_with_brace("a/**/", "a/b/c/d"));
    assert!(!glob_match_with_brace("a/**/", "a/bb"));
    assert!(!glob_match_with_brace("a/**/", "a/cb"));
    assert!(glob_match_with_brace("/**", "/a/b"));
    assert!(glob_match_with_brace("**/*", "a.b"));
    assert!(glob_match_with_brace("**/*", "a.js"));
    assert!(glob_match_with_brace("**/*.js", "a.js"));
    // assert!(glob_match_with_brace("a/**/", "a/"));
    assert!(glob_match_with_brace("**/*.js", "a/a.js"));
    assert!(glob_match_with_brace("**/*.js", "a/a/b.js"));
    assert!(glob_match_with_brace("a/**/b", "a/b"));
    assert!(glob_match_with_brace("a/**b", "a/b"));
    assert!(glob_match_with_brace("**/*.md", "a/b.md"));
    assert!(glob_match_with_brace("**/*", "a/b/c.js"));
    assert!(glob_match_with_brace("**/*", "a/b/c.txt"));
    assert!(glob_match_with_brace("a/**/", "a/b/c/d/"));
    assert!(glob_match_with_brace("**/*", "a/b/c/d/a.js"));
    assert!(glob_match_with_brace("a/b/**/*.js", "a/b/c/z.js"));
    assert!(glob_match_with_brace("a/b/**/*.js", "a/b/z.js"));
    assert!(glob_match_with_brace("**/*", "ab"));
    assert!(glob_match_with_brace("**/*", "ab/c"));
    assert!(glob_match_with_brace("**/*", "ab/c/d"));
    assert!(glob_match_with_brace("**/*", "abc.js"));

    assert!(!glob_match_with_brace("**/", "a"));
    assert!(!glob_match_with_brace("**/a/*", "a"));
    assert!(!glob_match_with_brace("**/a/*/*", "a"));
    assert!(!glob_match_with_brace("*/a/**", "a"));
    assert!(!glob_match_with_brace("a/**/*", "a"));
    assert!(!glob_match_with_brace("a/**/**/*", "a"));
    assert!(!glob_match_with_brace("**/", "a/b"));
    assert!(!glob_match_with_brace("**/b/*", "a/b"));
    assert!(!glob_match_with_brace("**/b/*/*", "a/b"));
    assert!(!glob_match_with_brace("b/**", "a/b"));
    assert!(!glob_match_with_brace("**/", "a/b/c"));
    assert!(!glob_match_with_brace("**/**/b", "a/b/c"));
    assert!(!glob_match_with_brace("**/b", "a/b/c"));
    assert!(!glob_match_with_brace("**/b/*/*", "a/b/c"));
    assert!(!glob_match_with_brace("b/**", "a/b/c"));
    assert!(!glob_match_with_brace("**/", "a/b/c/d"));
    assert!(!glob_match_with_brace("**/d/*", "a/b/c/d"));
    assert!(!glob_match_with_brace("b/**", "a/b/c/d"));
    assert!(glob_match_with_brace("**", "a"));
    assert!(glob_match_with_brace("**/**", "a"));
    assert!(glob_match_with_brace("**/**/*", "a"));
    assert!(glob_match_with_brace("**/**/a", "a"));
    assert!(glob_match_with_brace("**/a", "a"));
    // assert!(glob_match_with_brace("**/a/**", "a"));
    // assert!(glob_match_with_brace("a/**", "a"));
    assert!(glob_match_with_brace("**", "a/b"));
    assert!(glob_match_with_brace("**/**", "a/b"));
    assert!(glob_match_with_brace("**/**/*", "a/b"));
    assert!(glob_match_with_brace("**/**/b", "a/b"));
    assert!(glob_match_with_brace("**/b", "a/b"));
    // assert!(glob_match_with_brace("**/b/**", "a/b"));
    // assert!(glob_match_with_brace("*/b/**", "a/b"));
    assert!(glob_match_with_brace("a/**", "a/b"));
    assert!(glob_match_with_brace("a/**/*", "a/b"));
    assert!(glob_match_with_brace("a/**/**/*", "a/b"));
    assert!(glob_match_with_brace("**", "a/b/c"));
    assert!(glob_match_with_brace("**/**", "a/b/c"));
    assert!(glob_match_with_brace("**/**/*", "a/b/c"));
    assert!(glob_match_with_brace("**/b/*", "a/b/c"));
    assert!(glob_match_with_brace("**/b/**", "a/b/c"));
    assert!(glob_match_with_brace("*/b/**", "a/b/c"));
    assert!(glob_match_with_brace("a/**", "a/b/c"));
    assert!(glob_match_with_brace("a/**/*", "a/b/c"));
    assert!(glob_match_with_brace("a/**/**/*", "a/b/c"));
    assert!(glob_match_with_brace("**", "a/b/c/d"));
    assert!(glob_match_with_brace("**/**", "a/b/c/d"));
    assert!(glob_match_with_brace("**/**/*", "a/b/c/d"));
    assert!(glob_match_with_brace("**/**/d", "a/b/c/d"));
    assert!(glob_match_with_brace("**/b/**", "a/b/c/d"));
    assert!(glob_match_with_brace("**/b/*/*", "a/b/c/d"));
    assert!(glob_match_with_brace("**/d", "a/b/c/d"));
    assert!(glob_match_with_brace("*/b/**", "a/b/c/d"));
    assert!(glob_match_with_brace("a/**", "a/b/c/d"));
    assert!(glob_match_with_brace("a/**/*", "a/b/c/d"));
    assert!(glob_match_with_brace("a/**/**/*", "a/b/c/d"));

    assert!(glob_match_with_brace("**/**.txt.js", "/foo/bar.txt.js"));
  }

  #[test]
  fn utf8() {
    assert!(glob_match_with_brace("フ*/**/*", "フォルダ/aaa.js"));
    assert!(glob_match_with_brace("フォ*/**/*", "フォルダ/aaa.js"));
    assert!(glob_match_with_brace("フォル*/**/*", "フォルダ/aaa.js"));
    assert!(glob_match_with_brace("フ*ル*/**/*", "フォルダ/aaa.js"));
    assert!(glob_match_with_brace("フォルダ/**/*", "フォルダ/aaa.js"));
  }

  #[test]
  fn negation() {
    assert!(!glob_match_with_brace("!*", "abc"));
    assert!(!glob_match_with_brace("!abc", "abc"));
    assert!(!glob_match_with_brace("*!.md", "bar.md"));
    assert!(!glob_match_with_brace("foo!.md", "bar.md"));
    assert!(!glob_match_with_brace("\\!*!*.md", "foo!.md"));
    assert!(!glob_match_with_brace("\\!*!*.md", "foo!bar.md"));
    assert!(glob_match_with_brace("*!*.md", "!foo!.md"));
    assert!(glob_match_with_brace("\\!*!*.md", "!foo!.md"));
    assert!(glob_match_with_brace("!*foo", "abc"));
    assert!(glob_match_with_brace("!foo*", "abc"));
    assert!(glob_match_with_brace("!xyz", "abc"));
    assert!(glob_match_with_brace("*!*.*", "ba!r.js"));
    assert!(glob_match_with_brace("*.md", "bar.md"));
    assert!(glob_match_with_brace("*!*.*", "foo!.md"));
    assert!(glob_match_with_brace("*!*.md", "foo!.md"));
    assert!(glob_match_with_brace("*!.md", "foo!.md"));
    assert!(glob_match_with_brace("*.md", "foo!.md"));
    assert!(glob_match_with_brace("foo!.md", "foo!.md"));
    assert!(glob_match_with_brace("*!*.md", "foo!bar.md"));
    assert!(glob_match_with_brace("*b*.md", "foobar.md"));

    assert!(!glob_match_with_brace("a!!b", "a"));
    assert!(!glob_match_with_brace("a!!b", "aa"));
    assert!(!glob_match_with_brace("a!!b", "a/b"));
    assert!(!glob_match_with_brace("a!!b", "a!b"));
    assert!(glob_match_with_brace("a!!b", "a!!b"));
    assert!(!glob_match_with_brace("a!!b", "a/!!/b"));

    assert!(!glob_match_with_brace("!a/b", "a/b"));
    assert!(glob_match_with_brace("!a/b", "a"));
    assert!(glob_match_with_brace("!a/b", "a.b"));
    assert!(glob_match_with_brace("!a/b", "a/a"));
    assert!(glob_match_with_brace("!a/b", "a/c"));
    assert!(glob_match_with_brace("!a/b", "b/a"));
    assert!(glob_match_with_brace("!a/b", "b/b"));
    assert!(glob_match_with_brace("!a/b", "b/c"));

    assert!(!glob_match_with_brace("!abc", "abc"));
    assert!(glob_match_with_brace("!!abc", "abc"));
    assert!(!glob_match_with_brace("!!!abc", "abc"));
    assert!(glob_match_with_brace("!!!!abc", "abc"));
    assert!(!glob_match_with_brace("!!!!!abc", "abc"));
    assert!(glob_match_with_brace("!!!!!!abc", "abc"));
    assert!(!glob_match_with_brace("!!!!!!!abc", "abc"));
    assert!(glob_match_with_brace("!!!!!!!!abc", "abc"));

    // assert!(!glob_match_with_brace("!(*/*)", "a/a"));
    // assert!(!glob_match_with_brace("!(*/*)", "a/b"));
    // assert!(!glob_match_with_brace("!(*/*)", "a/c"));
    // assert!(!glob_match_with_brace("!(*/*)", "b/a"));
    // assert!(!glob_match_with_brace("!(*/*)", "b/b"));
    // assert!(!glob_match_with_brace("!(*/*)", "b/c"));
    // assert!(!glob_match_with_brace("!(*/b)", "a/b"));
    // assert!(!glob_match_with_brace("!(*/b)", "b/b"));
    // assert!(!glob_match_with_brace("!(a/b)", "a/b"));
    assert!(!glob_match_with_brace("!*", "a"));
    assert!(!glob_match_with_brace("!*", "a.b"));
    assert!(!glob_match_with_brace("!*/*", "a/a"));
    assert!(!glob_match_with_brace("!*/*", "a/b"));
    assert!(!glob_match_with_brace("!*/*", "a/c"));
    assert!(!glob_match_with_brace("!*/*", "b/a"));
    assert!(!glob_match_with_brace("!*/*", "b/b"));
    assert!(!glob_match_with_brace("!*/*", "b/c"));
    assert!(!glob_match_with_brace("!*/b", "a/b"));
    assert!(!glob_match_with_brace("!*/b", "b/b"));
    assert!(!glob_match_with_brace("!*/c", "a/c"));
    assert!(!glob_match_with_brace("!*/c", "a/c"));
    assert!(!glob_match_with_brace("!*/c", "b/c"));
    assert!(!glob_match_with_brace("!*/c", "b/c"));
    assert!(!glob_match_with_brace("!*a*", "bar"));
    assert!(!glob_match_with_brace("!*a*", "fab"));
    // assert!(!glob_match_with_brace("!a/(*)", "a/a"));
    // assert!(!glob_match_with_brace("!a/(*)", "a/b"));
    // assert!(!glob_match_with_brace("!a/(*)", "a/c"));
    // assert!(!glob_match_with_brace("!a/(b)", "a/b"));
    assert!(!glob_match_with_brace("!a/*", "a/a"));
    assert!(!glob_match_with_brace("!a/*", "a/b"));
    assert!(!glob_match_with_brace("!a/*", "a/c"));
    assert!(!glob_match_with_brace("!f*b", "fab"));
    // assert!(glob_match_with_brace("!(*/*)", "a"));
    // assert!(glob_match_with_brace("!(*/*)", "a.b"));
    // assert!(glob_match_with_brace("!(*/b)", "a"));
    // assert!(glob_match_with_brace("!(*/b)", "a.b"));
    // assert!(glob_match_with_brace("!(*/b)", "a/a"));
    // assert!(glob_match_with_brace("!(*/b)", "a/c"));
    // assert!(glob_match_with_brace("!(*/b)", "b/a"));
    // assert!(glob_match_with_brace("!(*/b)", "b/c"));
    // assert!(glob_match_with_brace("!(a/b)", "a"));
    // assert!(glob_match_with_brace("!(a/b)", "a.b"));
    // assert!(glob_match_with_brace("!(a/b)", "a/a"));
    // assert!(glob_match_with_brace("!(a/b)", "a/c"));
    // assert!(glob_match_with_brace("!(a/b)", "b/a"));
    // assert!(glob_match_with_brace("!(a/b)", "b/b"));
    // assert!(glob_match_with_brace("!(a/b)", "b/c"));
    assert!(glob_match_with_brace("!*", "a/a"));
    assert!(glob_match_with_brace("!*", "a/b"));
    assert!(glob_match_with_brace("!*", "a/c"));
    assert!(glob_match_with_brace("!*", "b/a"));
    assert!(glob_match_with_brace("!*", "b/b"));
    assert!(glob_match_with_brace("!*", "b/c"));
    assert!(glob_match_with_brace("!*/*", "a"));
    assert!(glob_match_with_brace("!*/*", "a.b"));
    assert!(glob_match_with_brace("!*/b", "a"));
    assert!(glob_match_with_brace("!*/b", "a.b"));
    assert!(glob_match_with_brace("!*/b", "a/a"));
    assert!(glob_match_with_brace("!*/b", "a/c"));
    assert!(glob_match_with_brace("!*/b", "b/a"));
    assert!(glob_match_with_brace("!*/b", "b/c"));
    assert!(glob_match_with_brace("!*/c", "a"));
    assert!(glob_match_with_brace("!*/c", "a.b"));
    assert!(glob_match_with_brace("!*/c", "a/a"));
    assert!(glob_match_with_brace("!*/c", "a/b"));
    assert!(glob_match_with_brace("!*/c", "b/a"));
    assert!(glob_match_with_brace("!*/c", "b/b"));
    assert!(glob_match_with_brace("!*a*", "foo"));
    // assert!(glob_match_with_brace("!a/(*)", "a"));
    // assert!(glob_match_with_brace("!a/(*)", "a.b"));
    // assert!(glob_match_with_brace("!a/(*)", "b/a"));
    // assert!(glob_match_with_brace("!a/(*)", "b/b"));
    // assert!(glob_match_with_brace("!a/(*)", "b/c"));
    // assert!(glob_match_with_brace("!a/(b)", "a"));
    // assert!(glob_match_with_brace("!a/(b)", "a.b"));
    // assert!(glob_match_with_brace("!a/(b)", "a/a"));
    // assert!(glob_match_with_brace("!a/(b)", "a/c"));
    // assert!(glob_match_with_brace("!a/(b)", "b/a"));
    // assert!(glob_match_with_brace("!a/(b)", "b/b"));
    // assert!(glob_match_with_brace("!a/(b)", "b/c"));
    assert!(glob_match_with_brace("!a/*", "a"));
    assert!(glob_match_with_brace("!a/*", "a.b"));
    assert!(glob_match_with_brace("!a/*", "b/a"));
    assert!(glob_match_with_brace("!a/*", "b/b"));
    assert!(glob_match_with_brace("!a/*", "b/c"));
    assert!(glob_match_with_brace("!f*b", "bar"));
    assert!(glob_match_with_brace("!f*b", "foo"));

    assert!(!glob_match_with_brace("!.md", ".md"));
    assert!(glob_match_with_brace("!**/*.md", "a.js"));
    // assert!(!glob_match_with_brace("!**/*.md", "b.md"));
    assert!(glob_match_with_brace("!**/*.md", "c.txt"));
    assert!(glob_match_with_brace("!*.md", "a.js"));
    assert!(!glob_match_with_brace("!*.md", "b.md"));
    assert!(glob_match_with_brace("!*.md", "c.txt"));
    assert!(!glob_match_with_brace("!*.md", "abc.md"));
    assert!(glob_match_with_brace("!*.md", "abc.txt"));
    assert!(!glob_match_with_brace("!*.md", "foo.md"));
    assert!(glob_match_with_brace("!.md", "foo.md"));

    assert!(glob_match_with_brace("!*.md", "a.js"));
    assert!(glob_match_with_brace("!*.md", "b.txt"));
    assert!(!glob_match_with_brace("!*.md", "c.md"));
    assert!(!glob_match_with_brace("!a/*/a.js", "a/a/a.js"));
    assert!(!glob_match_with_brace("!a/*/a.js", "a/b/a.js"));
    assert!(!glob_match_with_brace("!a/*/a.js", "a/c/a.js"));
    assert!(!glob_match_with_brace("!a/*/*/a.js", "a/a/a/a.js"));
    assert!(glob_match_with_brace("!a/*/*/a.js", "b/a/b/a.js"));
    assert!(glob_match_with_brace("!a/*/*/a.js", "c/a/c/a.js"));
    assert!(!glob_match_with_brace("!a/a*.txt", "a/a.txt"));
    assert!(glob_match_with_brace("!a/a*.txt", "a/b.txt"));
    assert!(glob_match_with_brace("!a/a*.txt", "a/c.txt"));
    assert!(!glob_match_with_brace("!a.a*.txt", "a.a.txt"));
    assert!(glob_match_with_brace("!a.a*.txt", "a.b.txt"));
    assert!(glob_match_with_brace("!a.a*.txt", "a.c.txt"));
    assert!(!glob_match_with_brace("!a/*.txt", "a/a.txt"));
    assert!(!glob_match_with_brace("!a/*.txt", "a/b.txt"));
    assert!(!glob_match_with_brace("!a/*.txt", "a/c.txt"));

    assert!(glob_match_with_brace("!*.md", "a.js"));
    assert!(glob_match_with_brace("!*.md", "b.txt"));
    assert!(!glob_match_with_brace("!*.md", "c.md"));
    // assert!(!glob_match_with_brace("!**/a.js", "a/a/a.js"));
    // assert!(!glob_match_with_brace("!**/a.js", "a/b/a.js"));
    // assert!(!glob_match_with_brace("!**/a.js", "a/c/a.js"));
    assert!(glob_match_with_brace("!**/a.js", "a/a/b.js"));
    assert!(!glob_match_with_brace("!a/**/a.js", "a/a/a/a.js"));
    assert!(glob_match_with_brace("!a/**/a.js", "b/a/b/a.js"));
    assert!(glob_match_with_brace("!a/**/a.js", "c/a/c/a.js"));
    assert!(glob_match_with_brace("!**/*.md", "a/b.js"));
    assert!(glob_match_with_brace("!**/*.md", "a.js"));
    assert!(!glob_match_with_brace("!**/*.md", "a/b.md"));
    // assert!(!glob_match_with_brace("!**/*.md", "a.md"));
    assert!(!glob_match_with_brace("**/*.md", "a/b.js"));
    assert!(!glob_match_with_brace("**/*.md", "a.js"));
    assert!(glob_match_with_brace("**/*.md", "a/b.md"));
    assert!(glob_match_with_brace("**/*.md", "a.md"));
    assert!(glob_match_with_brace("!**/*.md", "a/b.js"));
    assert!(glob_match_with_brace("!**/*.md", "a.js"));
    assert!(!glob_match_with_brace("!**/*.md", "a/b.md"));
    // assert!(!glob_match_with_brace("!**/*.md", "a.md"));
    assert!(glob_match_with_brace("!*.md", "a/b.js"));
    assert!(glob_match_with_brace("!*.md", "a.js"));
    assert!(glob_match_with_brace("!*.md", "a/b.md"));
    assert!(!glob_match_with_brace("!*.md", "a.md"));
    assert!(glob_match_with_brace("!**/*.md", "a.js"));
    // assert!(!glob_match_with_brace("!**/*.md", "b.md"));
    assert!(glob_match_with_brace("!**/*.md", "c.txt"));
  }

  #[test]
  fn question_mark() {
    assert!(glob_match_with_brace("?", "a"));
    assert!(!glob_match_with_brace("?", "aa"));
    assert!(!glob_match_with_brace("?", "ab"));
    assert!(!glob_match_with_brace("?", "aaa"));
    assert!(!glob_match_with_brace("?", "abcdefg"));

    assert!(!glob_match_with_brace("??", "a"));
    assert!(glob_match_with_brace("??", "aa"));
    assert!(glob_match_with_brace("??", "ab"));
    assert!(!glob_match_with_brace("??", "aaa"));
    assert!(!glob_match_with_brace("??", "abcdefg"));

    assert!(!glob_match_with_brace("???", "a"));
    assert!(!glob_match_with_brace("???", "aa"));
    assert!(!glob_match_with_brace("???", "ab"));
    assert!(glob_match_with_brace("???", "aaa"));
    assert!(!glob_match_with_brace("???", "abcdefg"));

    assert!(!glob_match_with_brace("a?c", "aaa"));
    assert!(glob_match_with_brace("a?c", "aac"));
    assert!(glob_match_with_brace("a?c", "abc"));
    assert!(!glob_match_with_brace("ab?", "a"));
    assert!(!glob_match_with_brace("ab?", "aa"));
    assert!(!glob_match_with_brace("ab?", "ab"));
    assert!(!glob_match_with_brace("ab?", "ac"));
    assert!(!glob_match_with_brace("ab?", "abcd"));
    assert!(!glob_match_with_brace("ab?", "abbb"));
    assert!(glob_match_with_brace("a?b", "acb"));

    assert!(!glob_match_with_brace("a/?/c/?/e.md", "a/bb/c/dd/e.md"));
    assert!(glob_match_with_brace("a/??/c/??/e.md", "a/bb/c/dd/e.md"));
    assert!(!glob_match_with_brace("a/??/c.md", "a/bbb/c.md"));
    assert!(glob_match_with_brace("a/?/c.md", "a/b/c.md"));
    assert!(glob_match_with_brace("a/?/c/?/e.md", "a/b/c/d/e.md"));
    assert!(!glob_match_with_brace("a/?/c/???/e.md", "a/b/c/d/e.md"));
    assert!(glob_match_with_brace("a/?/c/???/e.md", "a/b/c/zzz/e.md"));
    assert!(!glob_match_with_brace("a/?/c.md", "a/bb/c.md"));
    assert!(glob_match_with_brace("a/??/c.md", "a/bb/c.md"));
    assert!(glob_match_with_brace("a/???/c.md", "a/bbb/c.md"));
    assert!(glob_match_with_brace("a/????/c.md", "a/bbbb/c.md"));
  }

  #[test]
  fn braces() {
    assert!(glob_match_with_brace("{a,b,c}", "a"));
    assert!(glob_match_with_brace("{a,b,c}", "b"));
    assert!(glob_match_with_brace("{a,b,c}", "c"));
    assert!(!glob_match_with_brace("{a,b,c}", "aa"));
    assert!(!glob_match_with_brace("{a,b,c}", "bb"));
    assert!(!glob_match_with_brace("{a,b,c}", "cc"));

    assert!(glob_match_with_brace("a/{a,b}", "a/a"));
    assert!(glob_match_with_brace("a/{a,b}", "a/b"));
    assert!(!glob_match_with_brace("a/{a,b}", "a/c"));
    assert!(!glob_match_with_brace("a/{a,b}", "b/b"));
    assert!(!glob_match_with_brace("a/{a,b,c}", "b/b"));
    assert!(glob_match_with_brace("a/{a,b,c}", "a/c"));
    assert!(glob_match_with_brace("a{b,bc}.txt", "abc.txt"));

    assert!(glob_match_with_brace("foo[{a,b}]baz", "foo{baz"));

    assert!(!glob_match_with_brace("a{,b}.txt", "abc.txt"));
    assert!(!glob_match_with_brace("a{a,b,}.txt", "abc.txt"));
    assert!(!glob_match_with_brace("a{b,}.txt", "abc.txt"));
    assert!(glob_match_with_brace("a{,b}.txt", "a.txt"));
    assert!(glob_match_with_brace("a{b,}.txt", "a.txt"));
    assert!(glob_match_with_brace("a{a,b,}.txt", "aa.txt"));
    assert!(glob_match_with_brace("a{a,b,}.txt", "aa.txt"));
    assert!(glob_match_with_brace("a{,b}.txt", "ab.txt"));
    assert!(glob_match_with_brace("a{b,}.txt", "ab.txt"));

    // assert!(glob_match_with_brace("{a/,}a/**", "a"));
    assert!(glob_match_with_brace("a{a,b/}*.txt", "aa.txt"));
    assert!(glob_match_with_brace("a{a,b/}*.txt", "ab/.txt"));
    assert!(glob_match_with_brace("a{a,b/}*.txt", "ab/a.txt"));
    // assert!(glob_match_with_brace("{a/,}a/**", "a/"));
    assert!(glob_match_with_brace("{a/,}a/**", "a/a/"));
    // assert!(glob_match_with_brace("{a/,}a/**", "a/a"));
    assert!(glob_match_with_brace("{a/,}a/**", "a/a/a"));
    assert!(glob_match_with_brace("{a/,}a/**", "a/a/"));
    assert!(glob_match_with_brace("{a/,}a/**", "a/a/a/"));
    assert!(glob_match_with_brace("{a/,}b/**", "a/b/a/"));
    assert!(glob_match_with_brace("{a/,}b/**", "b/a/"));
    assert!(glob_match_with_brace("a{,/}*.txt", "a.txt"));
    assert!(glob_match_with_brace("a{,/}*.txt", "ab.txt"));
    assert!(glob_match_with_brace("a{,/}*.txt", "a/b.txt"));
    assert!(glob_match_with_brace("a{,/}*.txt", "a/ab.txt"));

    assert!(glob_match_with_brace(
      "a{,.*{foo,db},\\(bar\\)}.txt",
      "a.txt"
    ));
    assert!(!glob_match_with_brace(
      "a{,.*{foo,db},\\(bar\\)}.txt",
      "adb.txt"
    ));
    assert!(glob_match_with_brace(
      "a{,.*{foo,db},\\(bar\\)}.txt",
      "a.db.txt"
    ));

    assert!(glob_match_with_brace(
      "a{,*.{foo,db},\\(bar\\)}.txt",
      "a.txt"
    ));
    assert!(!glob_match_with_brace(
      "a{,*.{foo,db},\\(bar\\)}.txt",
      "adb.txt"
    ));
    assert!(glob_match_with_brace(
      "a{,*.{foo,db},\\(bar\\)}.txt",
      "a.db.txt"
    ));

    // assert!(glob_match_with_brace("a{,.*{foo,db},\\(bar\\)}", "a"));
    assert!(!glob_match_with_brace("a{,.*{foo,db},\\(bar\\)}", "adb"));
    assert!(glob_match_with_brace("a{,.*{foo,db},\\(bar\\)}", "a.db"));

    // assert!(glob_match_with_brace("a{,*.{foo,db},\\(bar\\)}", "a"));
    assert!(!glob_match_with_brace("a{,*.{foo,db},\\(bar\\)}", "adb"));
    assert!(glob_match_with_brace("a{,*.{foo,db},\\(bar\\)}", "a.db"));

    assert!(!glob_match_with_brace("{,.*{foo,db},\\(bar\\)}", "a"));
    assert!(!glob_match_with_brace("{,.*{foo,db},\\(bar\\)}", "adb"));
    assert!(!glob_match_with_brace("{,.*{foo,db},\\(bar\\)}", "a.db"));
    assert!(glob_match_with_brace("{,.*{foo,db},\\(bar\\)}", ".db"));

    assert!(!glob_match_with_brace("{,*.{foo,db},\\(bar\\)}", "a"));
    assert!(glob_match_with_brace("{*,*.{foo,db},\\(bar\\)}", "a"));
    assert!(!glob_match_with_brace("{,*.{foo,db},\\(bar\\)}", "adb"));
    assert!(glob_match_with_brace("{,*.{foo,db},\\(bar\\)}", "a.db"));

    assert!(!glob_match_with_brace(
      "a/b/**/c{d,e}/**/xyz.md",
      "a/b/c/xyz.md"
    ));
    assert!(!glob_match_with_brace(
      "a/b/**/c{d,e}/**/xyz.md",
      "a/b/d/xyz.md"
    ));
    assert!(glob_match_with_brace(
      "a/b/**/c{d,e}/**/xyz.md",
      "a/b/cd/xyz.md"
    ));
    assert!(glob_match_with_brace(
      "a/b/**/{c,d,e}/**/xyz.md",
      "a/b/c/xyz.md"
    ));
    assert!(glob_match_with_brace(
      "a/b/**/{c,d,e}/**/xyz.md",
      "a/b/d/xyz.md"
    ));
    assert!(glob_match_with_brace(
      "a/b/**/{c,d,e}/**/xyz.md",
      "a/b/e/xyz.md"
    ));

    assert!(glob_match_with_brace("*{a,b}*", "xax"));
    assert!(glob_match_with_brace("*{a,b}*", "xxax"));
    assert!(glob_match_with_brace("*{a,b}*", "xbx"));

    assert!(glob_match_with_brace("*{*a,b}", "xba"));
    assert!(glob_match_with_brace("*{*a,b}", "xb"));

    assert!(!glob_match_with_brace("*??", "a"));
    assert!(!glob_match_with_brace("*???", "aa"));
    assert!(glob_match_with_brace("*???", "aaa"));
    assert!(!glob_match_with_brace("*****??", "a"));
    assert!(!glob_match_with_brace("*****???", "aa"));
    assert!(glob_match_with_brace("*****???", "aaa"));

    assert!(!glob_match_with_brace("a*?c", "aaa"));
    assert!(glob_match_with_brace("a*?c", "aac"));
    assert!(glob_match_with_brace("a*?c", "abc"));

    assert!(glob_match_with_brace("a**?c", "abc"));
    assert!(!glob_match_with_brace("a**?c", "abb"));
    assert!(glob_match_with_brace("a**?c", "acc"));
    assert!(glob_match_with_brace("a*****?c", "abc"));

    assert!(glob_match_with_brace("*****?", "a"));
    assert!(glob_match_with_brace("*****?", "aa"));
    assert!(glob_match_with_brace("*****?", "abc"));
    assert!(glob_match_with_brace("*****?", "zzz"));
    assert!(glob_match_with_brace("*****?", "bbb"));
    assert!(glob_match_with_brace("*****?", "aaaa"));

    assert!(!glob_match_with_brace("*****??", "a"));
    assert!(glob_match_with_brace("*****??", "aa"));
    assert!(glob_match_with_brace("*****??", "abc"));
    assert!(glob_match_with_brace("*****??", "zzz"));
    assert!(glob_match_with_brace("*****??", "bbb"));
    assert!(glob_match_with_brace("*****??", "aaaa"));

    assert!(!glob_match_with_brace("?*****??", "a"));
    assert!(!glob_match_with_brace("?*****??", "aa"));
    assert!(glob_match_with_brace("?*****??", "abc"));
    assert!(glob_match_with_brace("?*****??", "zzz"));
    assert!(glob_match_with_brace("?*****??", "bbb"));
    assert!(glob_match_with_brace("?*****??", "aaaa"));

    assert!(glob_match_with_brace("?*****?c", "abc"));
    assert!(!glob_match_with_brace("?*****?c", "abb"));
    assert!(!glob_match_with_brace("?*****?c", "zzz"));

    assert!(glob_match_with_brace("?***?****c", "abc"));
    assert!(!glob_match_with_brace("?***?****c", "bbb"));
    assert!(!glob_match_with_brace("?***?****c", "zzz"));

    assert!(glob_match_with_brace("?***?****?", "abc"));
    assert!(glob_match_with_brace("?***?****?", "bbb"));
    assert!(glob_match_with_brace("?***?****?", "zzz"));

    assert!(glob_match_with_brace("?***?****", "abc"));
    assert!(glob_match_with_brace("*******c", "abc"));
    assert!(glob_match_with_brace("*******?", "abc"));
    assert!(glob_match_with_brace("a*cd**?**??k", "abcdecdhjk"));
    assert!(glob_match_with_brace("a**?**cd**?**??k", "abcdecdhjk"));
    assert!(glob_match_with_brace("a**?**cd**?**??k***", "abcdecdhjk"));
    assert!(glob_match_with_brace("a**?**cd**?**??***k", "abcdecdhjk"));
    assert!(glob_match_with_brace("a**?**cd**?**??***k**", "abcdecdhjk"));
    assert!(glob_match_with_brace("a****c**?**??*****", "abcdecdhjk"));

    assert!(!glob_match_with_brace("a/?/c/?/*/e.md", "a/b/c/d/e.md"));
    assert!(glob_match_with_brace("a/?/c/?/*/e.md", "a/b/c/d/e/e.md"));
    assert!(glob_match_with_brace(
      "a/?/c/?/*/e.md",
      "a/b/c/d/efghijk/e.md"
    ));
    assert!(glob_match_with_brace("a/?/**/e.md", "a/b/c/d/efghijk/e.md"));
    assert!(!glob_match_with_brace("a/?/e.md", "a/bb/e.md"));
    assert!(glob_match_with_brace("a/??/e.md", "a/bb/e.md"));
    assert!(!glob_match_with_brace("a/?/**/e.md", "a/bb/e.md"));
    assert!(glob_match_with_brace("a/?/**/e.md", "a/b/ccc/e.md"));
    assert!(glob_match_with_brace(
      "a/*/?/**/e.md",
      "a/b/c/d/efghijk/e.md"
    ));
    assert!(glob_match_with_brace(
      "a/*/?/**/e.md",
      "a/b/c/d/efgh.ijk/e.md"
    ));
    assert!(glob_match_with_brace(
      "a/*/?/**/e.md",
      "a/b.bb/c/d/efgh.ijk/e.md"
    ));
    assert!(glob_match_with_brace(
      "a/*/?/**/e.md",
      "a/bbb/c/d/efgh.ijk/e.md"
    ));

    assert!(glob_match_with_brace("a/*/ab??.md", "a/bbb/abcd.md"));
    assert!(glob_match_with_brace("a/bbb/ab??.md", "a/bbb/abcd.md"));
    assert!(glob_match_with_brace("a/bbb/ab???md", "a/bbb/abcd.md"));
  }

  #[test]
  fn fuzz_tests() {
    // https://github.com/devongovett/glob-match/issues/1
    let s = "{*{??*{??**,Uz*zz}w**{*{**a,z***b*[!}w??*azzzzzzzz*!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!z[za,z&zz}w**z*z*}";
    assert!(!glob_match_with_brace(s, s));
    let s = "**** *{*{??*{??***\u{5} *{*{??*{??***\u{5},\0U\0}]*****\u{1},\0***\0,\0\0}w****,\0U\0}]*****\u{1},\0***\0,\0\0}w*****\u{1}***{}*.*\0\0*\0";
    assert!(!glob_match_with_brace(s, s));
  }
}
