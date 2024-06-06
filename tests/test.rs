use glob_lab::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert!(glob_match("**/*/abc", "/abc"));
  }

  #[test]
  fn basic() {
    assert!(glob_match("abc", "abc"));
    assert!(glob_match("*", "abc"));
    assert!(glob_match("*", ""));
    assert!(glob_match("**", ""));
    assert!(glob_match("*c", "abc"));
    assert!(!glob_match("*b", "abc"));
    assert!(glob_match("a*", "abc"));
    assert!(!glob_match("b*", "abc"));
    assert!(glob_match("a*", "a"));
    assert!(glob_match("*a", "a"));
    assert!(glob_match("a*b*c*d*e*", "axbxcxdxe"));
    assert!(glob_match("a*b*c*d*e*", "axbxcxdxexxx"));
    assert!(glob_match("a*b?c*x", "abxbbxdbxebxczzx"));
    assert!(!glob_match("a*b?c*x", "abxbbxdbxebxczzy"));

    assert!(glob_match("a/*/test", "a/foo/test"));
    assert!(!glob_match("a/*/test", "a/foo/bar/test"));
    assert!(glob_match("a/**/test", "a/foo/test"));
    assert!(glob_match("a/**/test", "a/foo/bar/test"));
    assert!(glob_match("a/**/b/c", "a/foo/bar/b/c"));
    assert!(glob_match("a\\*b", "a*b"));
    assert!(!glob_match("a\\*b", "axb"));

    assert!(glob_match("[abc]", "a"));
    assert!(glob_match("[abc]", "b"));
    assert!(glob_match("[abc]", "c"));
    assert!(!glob_match("[abc]", "d"));
    assert!(glob_match("x[abc]x", "xax"));
    assert!(glob_match("x[abc]x", "xbx"));
    assert!(glob_match("x[abc]x", "xcx"));
    assert!(!glob_match("x[abc]x", "xdx"));
    assert!(!glob_match("x[abc]x", "xay"));
    assert!(glob_match("[?]", "?"));
    assert!(!glob_match("[?]", "a"));
    assert!(glob_match("[*]", "*"));
    assert!(!glob_match("[*]", "a"));

    assert!(glob_match("[a-cx]", "a"));
    assert!(glob_match("[a-cx]", "b"));
    assert!(glob_match("[a-cx]", "c"));
    assert!(!glob_match("[a-cx]", "d"));
    assert!(glob_match("[a-cx]", "x"));

    assert!(!glob_match("[^abc]", "a"));
    assert!(!glob_match("[^abc]", "b"));
    assert!(!glob_match("[^abc]", "c"));
    assert!(glob_match("[^abc]", "d"));
    assert!(!glob_match("[!abc]", "a"));
    assert!(!glob_match("[!abc]", "b"));
    assert!(!glob_match("[!abc]", "c"));
    assert!(glob_match("[!abc]", "d"));
    assert!(glob_match("[\\!]", "!"));

    assert!(glob_match("a*b*[cy]*d*e*", "axbxcxdxexxx"));
    assert!(glob_match("a*b*[cy]*d*e*", "axbxyxdxexxx"));
    assert!(glob_match("a*b*[cy]*d*e*", "axbxxxyxdxexxx"));

    assert!(!glob_match("*.txt", "some/big/path/to/the/needle.txt"));

    assert!(glob_match("/**/*a", "/a/a"));
    assert!(glob_match("**/*.js", "a/b.c/c.js"));
    assert!(glob_match("**/**/*.js", "a/b.c/c.js"));
    assert!(glob_match("a/**/*.d", "a/b/c.d"));
    assert!(glob_match("a/**/*.d", "a/.b/c.d"));

    assert!(glob_match("**/*/**", "a/b/c"));
    assert!(glob_match("**/*/c.js", "a/b/c.js"));
  }

  #[test]
  fn bash() {
    assert!(!glob_match("a*", "*"));
    assert!(!glob_match("a*", "**"));
    assert!(!glob_match("a*", "\\*"));
    assert!(!glob_match("a*", "a/*"));
    assert!(!glob_match("a*", "b"));
    assert!(!glob_match("a*", "bc"));
    assert!(!glob_match("a*", "bcd"));
    assert!(!glob_match("a*", "bdir/"));
    assert!(!glob_match("a*", "Beware"));
    assert!(glob_match("a*", "a"));
    assert!(glob_match("a*", "ab"));
    assert!(glob_match("a*", "abc"));

    assert!(!glob_match("\\a*", "*"));
    assert!(!glob_match("\\a*", "**"));
    assert!(!glob_match("\\a*", "\\*"));

    assert!(glob_match("\\a*", "a"));
    assert!(!glob_match("\\a*", "a/*"));
    assert!(glob_match("\\a*", "abc"));
    assert!(glob_match("\\a*", "abd"));
    assert!(glob_match("\\a*", "abe"));
    assert!(!glob_match("\\a*", "b"));
    assert!(!glob_match("\\a*", "bb"));
    assert!(!glob_match("\\a*", "bcd"));
    assert!(!glob_match("\\a*", "bdir/"));
    assert!(!glob_match("\\a*", "Beware"));
    assert!(!glob_match("\\a*", "c"));
    assert!(!glob_match("\\a*", "ca"));
    assert!(!glob_match("\\a*", "cb"));
    assert!(!glob_match("\\a*", "d"));
    assert!(!glob_match("\\a*", "dd"));
    assert!(!glob_match("\\a*", "de"));
  }

  #[test]
  fn bash_directories() {
    assert!(!glob_match("b*/", "*"));
    assert!(!glob_match("b*/", "**"));
    assert!(!glob_match("b*/", "\\*"));
    assert!(!glob_match("b*/", "a"));
    assert!(!glob_match("b*/", "a/*"));
    assert!(!glob_match("b*/", "abc"));
    assert!(!glob_match("b*/", "abd"));
    assert!(!glob_match("b*/", "abe"));
    assert!(!glob_match("b*/", "b"));
    assert!(!glob_match("b*/", "bb"));
    assert!(!glob_match("b*/", "bcd"));
    assert!(glob_match("b*/", "bdir/"));
    assert!(!glob_match("b*/", "Beware"));
    assert!(!glob_match("b*/", "c"));
    assert!(!glob_match("b*/", "ca"));
    assert!(!glob_match("b*/", "cb"));
    assert!(!glob_match("b*/", "d"));
    assert!(!glob_match("b*/", "dd"));
    assert!(!glob_match("b*/", "de"));
  }

  #[test]
  fn bash_escaping() {
    assert!(!glob_match("\\^", "*"));
    assert!(!glob_match("\\^", "**"));
    assert!(!glob_match("\\^", "\\*"));
    assert!(!glob_match("\\^", "a"));
    assert!(!glob_match("\\^", "a/*"));
    assert!(!glob_match("\\^", "abc"));
    assert!(!glob_match("\\^", "abd"));
    assert!(!glob_match("\\^", "abe"));
    assert!(!glob_match("\\^", "b"));
    assert!(!glob_match("\\^", "bb"));
    assert!(!glob_match("\\^", "bcd"));
    assert!(!glob_match("\\^", "bdir/"));
    assert!(!glob_match("\\^", "Beware"));
    assert!(!glob_match("\\^", "c"));
    assert!(!glob_match("\\^", "ca"));
    assert!(!glob_match("\\^", "cb"));
    assert!(!glob_match("\\^", "d"));
    assert!(!glob_match("\\^", "dd"));
    assert!(!glob_match("\\^", "de"));

    assert!(glob_match("\\*", "*"));
    assert!(!glob_match("\\*", "**"));
    assert!(!glob_match("\\*", "a"));
    assert!(!glob_match("\\*", "a/*"));
    assert!(!glob_match("\\*", "abc"));
    assert!(!glob_match("\\*", "abd"));
    assert!(!glob_match("\\*", "abe"));
    assert!(!glob_match("\\*", "b"));
    assert!(!glob_match("\\*", "bb"));
    assert!(!glob_match("\\*", "bcd"));
    assert!(!glob_match("\\*", "bdir/"));
    assert!(!glob_match("\\*", "Beware"));
    assert!(!glob_match("\\*", "c"));
    assert!(!glob_match("\\*", "ca"));
    assert!(!glob_match("\\*", "cb"));
    assert!(!glob_match("\\*", "d"));
    assert!(!glob_match("\\*", "dd"));
    assert!(!glob_match("\\*", "de"));

    assert!(!glob_match("a\\*", "*"));
    assert!(!glob_match("a\\*", "**"));
    assert!(!glob_match("a\\*", "\\*"));
    assert!(!glob_match("a\\*", "a"));
    assert!(!glob_match("a\\*", "a/*"));
    assert!(!glob_match("a\\*", "abc"));
    assert!(!glob_match("a\\*", "abd"));
    assert!(!glob_match("a\\*", "abe"));
    assert!(!glob_match("a\\*", "b"));
    assert!(!glob_match("a\\*", "bb"));
    assert!(!glob_match("a\\*", "bcd"));
    assert!(!glob_match("a\\*", "bdir/"));
    assert!(!glob_match("a\\*", "Beware"));
    assert!(!glob_match("a\\*", "c"));
    assert!(!glob_match("a\\*", "ca"));
    assert!(!glob_match("a\\*", "cb"));
    assert!(!glob_match("a\\*", "d"));
    assert!(!glob_match("a\\*", "dd"));
    assert!(!glob_match("a\\*", "de"));

    assert!(glob_match("*q*", "aqa"));
    assert!(glob_match("*q*", "aaqaa"));
    assert!(!glob_match("*q*", "*"));
    assert!(!glob_match("*q*", "**"));
    assert!(!glob_match("*q*", "\\*"));
    assert!(!glob_match("*q*", "a"));
    assert!(!glob_match("*q*", "a/*"));
    assert!(!glob_match("*q*", "abc"));
    assert!(!glob_match("*q*", "abd"));
    assert!(!glob_match("*q*", "abe"));
    assert!(!glob_match("*q*", "b"));
    assert!(!glob_match("*q*", "bb"));
    assert!(!glob_match("*q*", "bcd"));
    assert!(!glob_match("*q*", "bdir/"));
    assert!(!glob_match("*q*", "Beware"));
    assert!(!glob_match("*q*", "c"));
    assert!(!glob_match("*q*", "ca"));
    assert!(!glob_match("*q*", "cb"));
    assert!(!glob_match("*q*", "d"));
    assert!(!glob_match("*q*", "dd"));
    assert!(!glob_match("*q*", "de"));

    assert!(glob_match("\\**", "*"));
    assert!(glob_match("\\**", "**"));
    assert!(!glob_match("\\**", "\\*"));
    assert!(!glob_match("\\**", "a"));
    assert!(!glob_match("\\**", "a/*"));
    assert!(!glob_match("\\**", "abc"));
    assert!(!glob_match("\\**", "abd"));
    assert!(!glob_match("\\**", "abe"));
    assert!(!glob_match("\\**", "b"));
    assert!(!glob_match("\\**", "bb"));
    assert!(!glob_match("\\**", "bcd"));
    assert!(!glob_match("\\**", "bdir/"));
    assert!(!glob_match("\\**", "Beware"));
    assert!(!glob_match("\\**", "c"));
    assert!(!glob_match("\\**", "ca"));
    assert!(!glob_match("\\**", "cb"));
    assert!(!glob_match("\\**", "d"));
    assert!(!glob_match("\\**", "dd"));
    assert!(!glob_match("\\**", "de"));
  }

  #[test]
  fn bash_classes() {
    assert!(!glob_match("a*[^c]", "*"));
    assert!(!glob_match("a*[^c]", "**"));
    assert!(!glob_match("a*[^c]", "\\*"));
    assert!(!glob_match("a*[^c]", "a"));
    assert!(!glob_match("a*[^c]", "a/*"));
    assert!(!glob_match("a*[^c]", "abc"));
    assert!(glob_match("a*[^c]", "abd"));
    assert!(glob_match("a*[^c]", "abe"));
    assert!(!glob_match("a*[^c]", "b"));
    assert!(!glob_match("a*[^c]", "bb"));
    assert!(!glob_match("a*[^c]", "bcd"));
    assert!(!glob_match("a*[^c]", "bdir/"));
    assert!(!glob_match("a*[^c]", "Beware"));
    assert!(!glob_match("a*[^c]", "c"));
    assert!(!glob_match("a*[^c]", "ca"));
    assert!(!glob_match("a*[^c]", "cb"));
    assert!(!glob_match("a*[^c]", "d"));
    assert!(!glob_match("a*[^c]", "dd"));
    assert!(!glob_match("a*[^c]", "de"));
    assert!(!glob_match("a*[^c]", "baz"));
    assert!(!glob_match("a*[^c]", "bzz"));
    assert!(!glob_match("a*[^c]", "BZZ"));
    assert!(!glob_match("a*[^c]", "beware"));
    assert!(!glob_match("a*[^c]", "BewAre"));

    assert!(glob_match("a[X-]b", "a-b"));
    assert!(glob_match("a[X-]b", "aXb"));

    assert!(!glob_match("[a-y]*[^c]", "*"));
    assert!(glob_match("[a-y]*[^c]", "a*"));
    assert!(!glob_match("[a-y]*[^c]", "**"));
    assert!(!glob_match("[a-y]*[^c]", "\\*"));
    assert!(!glob_match("[a-y]*[^c]", "a"));
    assert!(glob_match("[a-y]*[^c]", "a123b"));
    assert!(!glob_match("[a-y]*[^c]", "a123c"));
    assert!(glob_match("[a-y]*[^c]", "ab"));
    assert!(!glob_match("[a-y]*[^c]", "a/*"));
    assert!(!glob_match("[a-y]*[^c]", "abc"));
    assert!(glob_match("[a-y]*[^c]", "abd"));
    assert!(glob_match("[a-y]*[^c]", "abe"));
    assert!(!glob_match("[a-y]*[^c]", "b"));
    assert!(glob_match("[a-y]*[^c]", "bd"));
    assert!(glob_match("[a-y]*[^c]", "bb"));
    assert!(glob_match("[a-y]*[^c]", "bcd"));
    assert!(glob_match("[a-y]*[^c]", "bdir/"));
    assert!(!glob_match("[a-y]*[^c]", "Beware"));
    assert!(!glob_match("[a-y]*[^c]", "c"));
    assert!(glob_match("[a-y]*[^c]", "ca"));
    assert!(glob_match("[a-y]*[^c]", "cb"));
    assert!(!glob_match("[a-y]*[^c]", "d"));
    assert!(glob_match("[a-y]*[^c]", "dd"));
    assert!(glob_match("[a-y]*[^c]", "dd"));
    assert!(glob_match("[a-y]*[^c]", "dd"));
    assert!(glob_match("[a-y]*[^c]", "de"));
    assert!(glob_match("[a-y]*[^c]", "baz"));
    assert!(glob_match("[a-y]*[^c]", "bzz"));
    assert!(glob_match("[a-y]*[^c]", "bzz"));
    assert!(!glob_match("[a-y]*[^c]", "BZZ"));
    assert!(glob_match("[a-y]*[^c]", "beware"));
    assert!(!glob_match("[a-y]*[^c]", "BewAre"));

    assert!(glob_match("a\\*b/*", "a*b/ooo"));
    assert!(glob_match("a\\*?/*", "a*b/ooo"));

    assert!(!glob_match("a[b]c", "*"));
    assert!(!glob_match("a[b]c", "**"));
    assert!(!glob_match("a[b]c", "\\*"));
    assert!(!glob_match("a[b]c", "a"));
    assert!(!glob_match("a[b]c", "a/*"));
    assert!(glob_match("a[b]c", "abc"));
    assert!(!glob_match("a[b]c", "abd"));
    assert!(!glob_match("a[b]c", "abe"));
    assert!(!glob_match("a[b]c", "b"));
    assert!(!glob_match("a[b]c", "bb"));
    assert!(!glob_match("a[b]c", "bcd"));
    assert!(!glob_match("a[b]c", "bdir/"));
    assert!(!glob_match("a[b]c", "Beware"));
    assert!(!glob_match("a[b]c", "c"));
    assert!(!glob_match("a[b]c", "ca"));
    assert!(!glob_match("a[b]c", "cb"));
    assert!(!glob_match("a[b]c", "d"));
    assert!(!glob_match("a[b]c", "dd"));
    assert!(!glob_match("a[b]c", "de"));
    assert!(!glob_match("a[b]c", "baz"));
    assert!(!glob_match("a[b]c", "bzz"));
    assert!(!glob_match("a[b]c", "BZZ"));
    assert!(!glob_match("a[b]c", "beware"));
    assert!(!glob_match("a[b]c", "BewAre"));

    assert!(!glob_match("a[\"b\"]c", "*"));
    assert!(!glob_match("a[\"b\"]c", "**"));
    assert!(!glob_match("a[\"b\"]c", "\\*"));
    assert!(!glob_match("a[\"b\"]c", "a"));
    assert!(!glob_match("a[\"b\"]c", "a/*"));
    assert!(glob_match("a[\"b\"]c", "abc"));
    assert!(!glob_match("a[\"b\"]c", "abd"));
    assert!(!glob_match("a[\"b\"]c", "abe"));
    assert!(!glob_match("a[\"b\"]c", "b"));
    assert!(!glob_match("a[\"b\"]c", "bb"));
    assert!(!glob_match("a[\"b\"]c", "bcd"));
    assert!(!glob_match("a[\"b\"]c", "bdir/"));
    assert!(!glob_match("a[\"b\"]c", "Beware"));
    assert!(!glob_match("a[\"b\"]c", "c"));
    assert!(!glob_match("a[\"b\"]c", "ca"));
    assert!(!glob_match("a[\"b\"]c", "cb"));
    assert!(!glob_match("a[\"b\"]c", "d"));
    assert!(!glob_match("a[\"b\"]c", "dd"));
    assert!(!glob_match("a[\"b\"]c", "de"));
    assert!(!glob_match("a[\"b\"]c", "baz"));
    assert!(!glob_match("a[\"b\"]c", "bzz"));
    assert!(!glob_match("a[\"b\"]c", "BZZ"));
    assert!(!glob_match("a[\"b\"]c", "beware"));
    assert!(!glob_match("a[\"b\"]c", "BewAre"));

    assert!(!glob_match("a[\\\\b]c", "*"));
    assert!(!glob_match("a[\\\\b]c", "**"));
    assert!(!glob_match("a[\\\\b]c", "\\*"));
    assert!(!glob_match("a[\\\\b]c", "a"));
    assert!(!glob_match("a[\\\\b]c", "a/*"));
    assert!(glob_match("a[\\\\b]c", "abc"));
    assert!(!glob_match("a[\\\\b]c", "abd"));
    assert!(!glob_match("a[\\\\b]c", "abe"));
    assert!(!glob_match("a[\\\\b]c", "b"));
    assert!(!glob_match("a[\\\\b]c", "bb"));
    assert!(!glob_match("a[\\\\b]c", "bcd"));
    assert!(!glob_match("a[\\\\b]c", "bdir/"));
    assert!(!glob_match("a[\\\\b]c", "Beware"));
    assert!(!glob_match("a[\\\\b]c", "c"));
    assert!(!glob_match("a[\\\\b]c", "ca"));
    assert!(!glob_match("a[\\\\b]c", "cb"));
    assert!(!glob_match("a[\\\\b]c", "d"));
    assert!(!glob_match("a[\\\\b]c", "dd"));
    assert!(!glob_match("a[\\\\b]c", "de"));
    assert!(!glob_match("a[\\\\b]c", "baz"));
    assert!(!glob_match("a[\\\\b]c", "bzz"));
    assert!(!glob_match("a[\\\\b]c", "BZZ"));
    assert!(!glob_match("a[\\\\b]c", "beware"));
    assert!(!glob_match("a[\\\\b]c", "BewAre"));

    assert!(!glob_match("a[\\b]c", "*"));
    assert!(!glob_match("a[\\b]c", "**"));
    assert!(!glob_match("a[\\b]c", "\\*"));
    assert!(!glob_match("a[\\b]c", "a"));
    assert!(!glob_match("a[\\b]c", "a/*"));
    assert!(!glob_match("a[\\b]c", "abc"));
    assert!(!glob_match("a[\\b]c", "abd"));
    assert!(!glob_match("a[\\b]c", "abe"));
    assert!(!glob_match("a[\\b]c", "b"));
    assert!(!glob_match("a[\\b]c", "bb"));
    assert!(!glob_match("a[\\b]c", "bcd"));
    assert!(!glob_match("a[\\b]c", "bdir/"));
    assert!(!glob_match("a[\\b]c", "Beware"));
    assert!(!glob_match("a[\\b]c", "c"));
    assert!(!glob_match("a[\\b]c", "ca"));
    assert!(!glob_match("a[\\b]c", "cb"));
    assert!(!glob_match("a[\\b]c", "d"));
    assert!(!glob_match("a[\\b]c", "dd"));
    assert!(!glob_match("a[\\b]c", "de"));
    assert!(!glob_match("a[\\b]c", "baz"));
    assert!(!glob_match("a[\\b]c", "bzz"));
    assert!(!glob_match("a[\\b]c", "BZZ"));
    assert!(!glob_match("a[\\b]c", "beware"));
    assert!(!glob_match("a[\\b]c", "BewAre"));

    assert!(!glob_match("a[b-d]c", "*"));
    assert!(!glob_match("a[b-d]c", "**"));
    assert!(!glob_match("a[b-d]c", "\\*"));
    assert!(!glob_match("a[b-d]c", "a"));
    assert!(!glob_match("a[b-d]c", "a/*"));
    assert!(glob_match("a[b-d]c", "abc"));
    assert!(!glob_match("a[b-d]c", "abd"));
    assert!(!glob_match("a[b-d]c", "abe"));
    assert!(!glob_match("a[b-d]c", "b"));
    assert!(!glob_match("a[b-d]c", "bb"));
    assert!(!glob_match("a[b-d]c", "bcd"));
    assert!(!glob_match("a[b-d]c", "bdir/"));
    assert!(!glob_match("a[b-d]c", "Beware"));
    assert!(!glob_match("a[b-d]c", "c"));
    assert!(!glob_match("a[b-d]c", "ca"));
    assert!(!glob_match("a[b-d]c", "cb"));
    assert!(!glob_match("a[b-d]c", "d"));
    assert!(!glob_match("a[b-d]c", "dd"));
    assert!(!glob_match("a[b-d]c", "de"));
    assert!(!glob_match("a[b-d]c", "baz"));
    assert!(!glob_match("a[b-d]c", "bzz"));
    assert!(!glob_match("a[b-d]c", "BZZ"));
    assert!(!glob_match("a[b-d]c", "beware"));
    assert!(!glob_match("a[b-d]c", "BewAre"));

    assert!(!glob_match("a?c", "*"));
    assert!(!glob_match("a?c", "**"));
    assert!(!glob_match("a?c", "\\*"));
    assert!(!glob_match("a?c", "a"));
    assert!(!glob_match("a?c", "a/*"));
    assert!(glob_match("a?c", "abc"));
    assert!(!glob_match("a?c", "abd"));
    assert!(!glob_match("a?c", "abe"));
    assert!(!glob_match("a?c", "b"));
    assert!(!glob_match("a?c", "bb"));
    assert!(!glob_match("a?c", "bcd"));
    assert!(!glob_match("a?c", "bdir/"));
    assert!(!glob_match("a?c", "Beware"));
    assert!(!glob_match("a?c", "c"));
    assert!(!glob_match("a?c", "ca"));
    assert!(!glob_match("a?c", "cb"));
    assert!(!glob_match("a?c", "d"));
    assert!(!glob_match("a?c", "dd"));
    assert!(!glob_match("a?c", "de"));
    assert!(!glob_match("a?c", "baz"));
    assert!(!glob_match("a?c", "bzz"));
    assert!(!glob_match("a?c", "BZZ"));
    assert!(!glob_match("a?c", "beware"));
    assert!(!glob_match("a?c", "BewAre"));

    assert!(glob_match("*/man*/bash.*", "man/man1/bash.1"));

    assert!(glob_match("[^a-c]*", "*"));
    assert!(glob_match("[^a-c]*", "**"));
    assert!(!glob_match("[^a-c]*", "a"));
    assert!(!glob_match("[^a-c]*", "a/*"));
    assert!(!glob_match("[^a-c]*", "abc"));
    assert!(!glob_match("[^a-c]*", "abd"));
    assert!(!glob_match("[^a-c]*", "abe"));
    assert!(!glob_match("[^a-c]*", "b"));
    assert!(!glob_match("[^a-c]*", "bb"));
    assert!(!glob_match("[^a-c]*", "bcd"));
    assert!(!glob_match("[^a-c]*", "bdir/"));
    assert!(glob_match("[^a-c]*", "Beware"));
    assert!(glob_match("[^a-c]*", "Beware"));
    assert!(!glob_match("[^a-c]*", "c"));
    assert!(!glob_match("[^a-c]*", "ca"));
    assert!(!glob_match("[^a-c]*", "cb"));
    assert!(glob_match("[^a-c]*", "d"));
    assert!(glob_match("[^a-c]*", "dd"));
    assert!(glob_match("[^a-c]*", "de"));
    assert!(!glob_match("[^a-c]*", "baz"));
    assert!(!glob_match("[^a-c]*", "bzz"));
    assert!(glob_match("[^a-c]*", "BZZ"));
    assert!(!glob_match("[^a-c]*", "beware"));
    assert!(glob_match("[^a-c]*", "BewAre"));
  }

  #[test]
  fn bash_wildmatch() {
    assert!(!glob_match("a[]-]b", "aab"));
    assert!(!glob_match("[ten]", "ten"));
    assert!(glob_match("]", "]"));
    assert!(glob_match("a[]-]b", "a-b"));
    assert!(glob_match("a[]-]b", "a]b"));
    assert!(glob_match("a[]]b", "a]b"));
    assert!(glob_match("a[\\]a\\-]b", "aab"));
    assert!(glob_match("t[a-g]n", "ten"));
    assert!(glob_match("t[^a-g]n", "ton"));
  }

  #[test]
  fn bash_slashmatch() {
    assert!(glob_match("foo[/]bar", "foo/bar"));
    assert!(glob_match("f[^eiu][^eiu][^eiu][^eiu][^eiu]r", "foo-bar"));
  }

  #[test]
  fn bash_extra_stars() {
    assert!(!glob_match("a**c", "bbc"));
    assert!(glob_match("a**c", "abc"));
    assert!(!glob_match("a**c", "bbd"));

    assert!(!glob_match("a***c", "bbc"));
    assert!(glob_match("a***c", "abc"));
    assert!(!glob_match("a***c", "bbd"));

    assert!(!glob_match("a*****?c", "bbc"));
    assert!(glob_match("a*****?c", "abc"));
    assert!(!glob_match("a*****?c", "bbc"));

    assert!(glob_match("?*****??", "bbc"));
    assert!(glob_match("?*****??", "abc"));

    assert!(glob_match("*****??", "bbc"));
    assert!(glob_match("*****??", "abc"));

    assert!(glob_match("?*****?c", "bbc"));
    assert!(glob_match("?*****?c", "abc"));

    assert!(glob_match("?***?****c", "bbc"));
    assert!(glob_match("?***?****c", "abc"));
    assert!(!glob_match("?***?****c", "bbd"));

    assert!(glob_match("?***?****?", "bbc"));
    assert!(glob_match("?***?****?", "abc"));

    assert!(glob_match("?***?****", "bbc"));
    assert!(glob_match("?***?****", "abc"));

    assert!(glob_match("*******c", "bbc"));
    assert!(glob_match("*******c", "abc"));

    assert!(glob_match("*******?", "bbc"));
    assert!(glob_match("*******?", "abc"));

    assert!(glob_match("a*cd**?**??k", "abcdecdhjk"));
    assert!(glob_match("a**?**cd**?**??k", "abcdecdhjk"));
    assert!(glob_match("a**?**cd**?**??k***", "abcdecdhjk"));
    assert!(glob_match("a**?**cd**?**??***k", "abcdecdhjk"));
    assert!(glob_match("a**?**cd**?**??***k**", "abcdecdhjk"));
    assert!(glob_match("a****c**?**??*****", "abcdecdhjk"));
  }

  #[test]
  fn stars() {
    assert!(!glob_match("*.js", "a/b/c/z.js"));
    assert!(!glob_match("*.js", "a/b/z.js"));
    assert!(!glob_match("*.js", "a/z.js"));
    assert!(glob_match("*.js", "z.js"));

    assert!(glob_match("z*.js", "z.js"));
    assert!(glob_match("*/*", "a/z"));
    assert!(glob_match("*/z*.js", "a/z.js"));
    assert!(glob_match("a/z*.js", "a/z.js"));

    assert!(glob_match("*", "ab"));
    assert!(glob_match("*", "abc"));

    assert!(!glob_match("f*", "bar"));
    assert!(!glob_match("*r", "foo"));
    assert!(!glob_match("b*", "foo"));
    assert!(!glob_match("*", "foo/bar"));
    assert!(glob_match("*c", "abc"));
    assert!(glob_match("a*", "abc"));
    assert!(glob_match("a*c", "abc"));
    assert!(glob_match("*r", "bar"));
    assert!(glob_match("b*", "bar"));
    assert!(glob_match("f*", "foo"));

    assert!(glob_match("*abc*", "one abc two"));
    assert!(glob_match("a*b", "a         b"));

    assert!(!glob_match("*a*", "foo"));
    assert!(glob_match("*a*", "bar"));
    assert!(glob_match("*abc*", "oneabctwo"));
    assert!(!glob_match("*-bc-*", "a-b.c-d"));
    assert!(glob_match("*-*.*-*", "a-b.c-d"));
    assert!(glob_match("*-b*c-*", "a-b.c-d"));
    assert!(glob_match("*-b.c-*", "a-b.c-d"));
    assert!(glob_match("*.*", "a-b.c-d"));
    assert!(glob_match("*.*-*", "a-b.c-d"));
    assert!(glob_match("*.*-d", "a-b.c-d"));
    assert!(glob_match("*.c-*", "a-b.c-d"));
    assert!(glob_match("*b.*d", "a-b.c-d"));
    assert!(glob_match("a*.c*", "a-b.c-d"));
    assert!(glob_match("a-*.*-d", "a-b.c-d"));
    assert!(glob_match("*.*", "a.b"));
    assert!(glob_match("*.b", "a.b"));
    assert!(glob_match("a.*", "a.b"));
    assert!(glob_match("a.b", "a.b"));

    assert!(!glob_match("**-bc-**", "a-b.c-d"));
    assert!(glob_match("**-**.**-**", "a-b.c-d"));
    assert!(glob_match("**-b**c-**", "a-b.c-d"));
    assert!(glob_match("**-b.c-**", "a-b.c-d"));
    assert!(glob_match("**.**", "a-b.c-d"));
    assert!(glob_match("**.**-**", "a-b.c-d"));
    assert!(glob_match("**.**-d", "a-b.c-d"));
    assert!(glob_match("**.c-**", "a-b.c-d"));
    assert!(glob_match("**b.**d", "a-b.c-d"));
    assert!(glob_match("a**.c**", "a-b.c-d"));
    assert!(glob_match("a-**.**-d", "a-b.c-d"));
    assert!(glob_match("**.**", "a.b"));
    assert!(glob_match("**.b", "a.b"));
    assert!(glob_match("a.**", "a.b"));
    assert!(glob_match("a.b", "a.b"));

    assert!(glob_match("*/*", "/ab"));
    assert!(glob_match(".", "."));
    assert!(!glob_match("a/", "a/.b"));
    assert!(glob_match("/*", "/ab"));
    assert!(glob_match("/??", "/ab"));
    assert!(glob_match("/?b", "/ab"));
    assert!(glob_match("/*", "/cd"));
    assert!(glob_match("a", "a"));
    assert!(glob_match("a/.*", "a/.b"));
    assert!(glob_match("?/?", "a/b"));
    assert!(glob_match("a/**/j/**/z/*.md", "a/b/c/d/e/j/n/p/o/z/c.md"));
    assert!(glob_match("a/**/z/*.md", "a/b/c/d/e/z/c.md"));
    assert!(glob_match("a/b/c/*.md", "a/b/c/xyz.md"));
    assert!(glob_match("a/b/c/*.md", "a/b/c/xyz.md"));
    assert!(glob_match("a/*/z/.a", "a/b/z/.a"));
    assert!(!glob_match("bz", "a/b/z/.a"));
    assert!(glob_match("a/**/c/*.md", "a/bb.bb/aa/b.b/aa/c/xyz.md"));
    assert!(glob_match("a/**/c/*.md", "a/bb.bb/aa/bb/aa/c/xyz.md"));
    assert!(glob_match("a/*/c/*.md", "a/bb.bb/c/xyz.md"));
    assert!(glob_match("a/*/c/*.md", "a/bb/c/xyz.md"));
    assert!(glob_match("a/*/c/*.md", "a/bbbb/c/xyz.md"));
    assert!(glob_match("*", "aaa"));
    assert!(glob_match("*", "ab"));
    assert!(glob_match("ab", "ab"));

    assert!(!glob_match("*/*/*", "aaa"));
    assert!(!glob_match("*/*/*", "aaa/bb/aa/rr"));
    assert!(!glob_match("aaa*", "aaa/bba/ccc"));
    // assert!(!glob_match("aaa**", "aaa/bba/ccc"));
    assert!(!glob_match("aaa/*", "aaa/bba/ccc"));
    assert!(!glob_match("aaa/*ccc", "aaa/bba/ccc"));
    assert!(!glob_match("aaa/*z", "aaa/bba/ccc"));
    assert!(!glob_match("*/*/*", "aaa/bbb"));
    assert!(!glob_match("*/*jk*/*i", "ab/zzz/ejkl/hi"));
    assert!(glob_match("*/*/*", "aaa/bba/ccc"));
    assert!(glob_match("aaa/**", "aaa/bba/ccc"));
    assert!(glob_match("aaa/*", "aaa/bbb"));
    assert!(glob_match("*/*z*/*/*i", "ab/zzz/ejkl/hi"));
    assert!(glob_match("*j*i", "abzzzejklhi"));

    assert!(glob_match("*", "a"));
    assert!(glob_match("*", "b"));
    assert!(!glob_match("*", "a/a"));
    assert!(!glob_match("*", "a/a/a"));
    assert!(!glob_match("*", "a/a/b"));
    assert!(!glob_match("*", "a/a/a/a"));
    assert!(!glob_match("*", "a/a/a/a/a"));

    assert!(!glob_match("*/*", "a"));
    assert!(glob_match("*/*", "a/a"));
    assert!(!glob_match("*/*", "a/a/a"));

    assert!(!glob_match("*/*/*", "a"));
    assert!(!glob_match("*/*/*", "a/a"));
    assert!(glob_match("*/*/*", "a/a/a"));
    assert!(!glob_match("*/*/*", "a/a/a/a"));

    assert!(!glob_match("*/*/*/*", "a"));
    assert!(!glob_match("*/*/*/*", "a/a"));
    assert!(!glob_match("*/*/*/*", "a/a/a"));
    assert!(glob_match("*/*/*/*", "a/a/a/a"));
    assert!(!glob_match("*/*/*/*", "a/a/a/a/a"));

    assert!(!glob_match("*/*/*/*/*", "a"));
    assert!(!glob_match("*/*/*/*/*", "a/a"));
    assert!(!glob_match("*/*/*/*/*", "a/a/a"));
    assert!(!glob_match("*/*/*/*/*", "a/a/b"));
    assert!(!glob_match("*/*/*/*/*", "a/a/a/a"));
    assert!(glob_match("*/*/*/*/*", "a/a/a/a/a"));
    assert!(!glob_match("*/*/*/*/*", "a/a/a/a/a/a"));

    assert!(!glob_match("a/*", "a"));
    assert!(glob_match("a/*", "a/a"));
    assert!(!glob_match("a/*", "a/a/a"));
    assert!(!glob_match("a/*", "a/a/a/a"));
    assert!(!glob_match("a/*", "a/a/a/a/a"));

    assert!(!glob_match("a/*/*", "a"));
    assert!(!glob_match("a/*/*", "a/a"));
    assert!(glob_match("a/*/*", "a/a/a"));
    assert!(!glob_match("a/*/*", "b/a/a"));
    assert!(!glob_match("a/*/*", "a/a/a/a"));
    assert!(!glob_match("a/*/*", "a/a/a/a/a"));

    assert!(!glob_match("a/*/*/*", "a"));
    assert!(!glob_match("a/*/*/*", "a/a"));
    assert!(!glob_match("a/*/*/*", "a/a/a"));
    assert!(glob_match("a/*/*/*", "a/a/a/a"));
    assert!(!glob_match("a/*/*/*", "a/a/a/a/a"));

    assert!(!glob_match("a/*/*/*/*", "a"));
    assert!(!glob_match("a/*/*/*/*", "a/a"));
    assert!(!glob_match("a/*/*/*/*", "a/a/a"));
    assert!(!glob_match("a/*/*/*/*", "a/a/b"));
    assert!(!glob_match("a/*/*/*/*", "a/a/a/a"));
    assert!(glob_match("a/*/*/*/*", "a/a/a/a/a"));

    assert!(!glob_match("a/*/a", "a"));
    assert!(!glob_match("a/*/a", "a/a"));
    assert!(glob_match("a/*/a", "a/a/a"));
    assert!(!glob_match("a/*/a", "a/a/b"));
    assert!(!glob_match("a/*/a", "a/a/a/a"));
    assert!(!glob_match("a/*/a", "a/a/a/a/a"));

    assert!(!glob_match("a/*/b", "a"));
    assert!(!glob_match("a/*/b", "a/a"));
    assert!(!glob_match("a/*/b", "a/a/a"));
    assert!(glob_match("a/*/b", "a/a/b"));
    assert!(!glob_match("a/*/b", "a/a/a/a"));
    assert!(!glob_match("a/*/b", "a/a/a/a/a"));

    assert!(!glob_match("*/**/a", "a"));
    assert!(!glob_match("*/**/a", "a/a/b"));
    assert!(glob_match("*/**/a", "a/a"));
    assert!(glob_match("*/**/a", "a/a/a"));
    assert!(glob_match("*/**/a", "a/a/a/a"));
    assert!(glob_match("*/**/a", "a/a/a/a/a"));

    assert!(!glob_match("*/", "a"));
    assert!(!glob_match("*/*", "a"));
    assert!(!glob_match("a/*", "a"));
    // assert!(!glob_match("*/*", "a/"));
    // assert!(!glob_match("a/*", "a/"));
    assert!(!glob_match("*", "a/a"));
    assert!(!glob_match("*/", "a/a"));
    assert!(!glob_match("*/", "a/x/y"));
    assert!(!glob_match("*/*", "a/x/y"));
    assert!(!glob_match("a/*", "a/x/y"));
    // assert!(glob_match("*", "a/"));
    assert!(glob_match("*", "a"));
    assert!(glob_match("*/", "a/"));
    assert!(glob_match("*/*", "a/a"));
    assert!(glob_match("a/*", "a/a"));

    assert!(!glob_match("a/**/*.txt", "a.txt"));
    assert!(glob_match("a/**/*.txt", "a/x/y.txt"));
    assert!(!glob_match("a/**/*.txt", "a/x/y/z"));

    assert!(!glob_match("a/*.txt", "a.txt"));
    assert!(glob_match("a/*.txt", "a/b.txt"));
    assert!(!glob_match("a/*.txt", "a/x/y.txt"));
    assert!(!glob_match("a/*.txt", "a/x/y/z"));

    assert!(glob_match("a*.txt", "a.txt"));
    assert!(!glob_match("a*.txt", "a/b.txt"));
    assert!(!glob_match("a*.txt", "a/x/y.txt"));
    assert!(!glob_match("a*.txt", "a/x/y/z"));

    assert!(glob_match("*.txt", "a.txt"));
    assert!(!glob_match("*.txt", "a/b.txt"));
    assert!(!glob_match("*.txt", "a/x/y.txt"));
    assert!(!glob_match("*.txt", "a/x/y/z"));

    assert!(!glob_match("a*", "a/b"));
    assert!(!glob_match("a/**/b", "a/a/bb"));
    assert!(!glob_match("a/**/b", "a/bb"));

    assert!(!glob_match("*/**", "foo"));
    assert!(!glob_match("**/", "foo/bar"));
    assert!(!glob_match("**/*/", "foo/bar"));
    assert!(!glob_match("*/*/", "foo/bar"));

    assert!(glob_match("**/..", "/home/foo/.."));
    assert!(glob_match("**/a", "a"));
    assert!(glob_match("**", "a/a"));
    assert!(glob_match("a/**", "a/a"));
    assert!(glob_match("a/**", "a/"));
    // assert!(glob_match("a/**", "a"));
    assert!(!glob_match("**/", "a/a"));
    // assert!(glob_match("**/a/**", "a"));
    // assert!(glob_match("a/**", "a"));
    assert!(!glob_match("**/", "a/a"));
    assert!(glob_match("*/**/a", "a/a"));
    // assert!(glob_match("a/**", "a"));
    assert!(glob_match("*/**", "foo/"));
    assert!(glob_match("**/*", "foo/bar"));
    assert!(glob_match("*/*", "foo/bar"));
    assert!(glob_match("*/**", "foo/bar"));
    assert!(glob_match("**/", "foo/bar/"));
    assert!(glob_match("**/*/", "foo/bar/"));
    assert!(glob_match("*/**", "foo/bar/"));
    assert!(glob_match("*/*/", "foo/bar/"));

    assert!(!glob_match("*/foo", "bar/baz/foo"));
    assert!(!glob_match("**/bar/*", "deep/foo/bar"));
    assert!(!glob_match("*/bar/**", "deep/foo/bar/baz/x"));
    assert!(!glob_match("/*", "ef"));
    assert!(!glob_match("foo?bar", "foo/bar"));
    assert!(!glob_match("**/bar*", "foo/bar/baz"));
    assert!(!glob_match("foo**bar", "foo/baz/bar"));
    assert!(!glob_match("foo*bar", "foo/baz/bar"));
    assert!(glob_match("/*", "/ab"));
    assert!(glob_match("/*", "/cd"));
    assert!(glob_match("/*", "/ef"));
    assert!(glob_match("a/**/j/**/z/*.md", "a/b/j/c/z/x.md"));
    assert!(glob_match("a/**/j/**/z/*.md", "a/j/z/x.md"));

    assert!(glob_match("**/foo", "bar/baz/foo"));
    assert!(glob_match("**/bar/*", "deep/foo/bar/baz"));
    assert!(glob_match("**/bar/**", "deep/foo/bar/baz/"));
    assert!(glob_match("**/bar/*/*", "deep/foo/bar/baz/x"));
    assert!(glob_match("foo/**/**/bar", "foo/b/a/z/bar"));
    assert!(glob_match("foo/**/bar", "foo/b/a/z/bar"));
    assert!(glob_match("foo/**/**/bar", "foo/bar"));
    assert!(glob_match("foo/**/bar", "foo/bar"));
    assert!(glob_match("*/bar/**", "foo/bar/baz/x"));
    assert!(glob_match("foo/**/**/bar", "foo/baz/bar"));
    assert!(glob_match("foo/**/bar", "foo/baz/bar"));
    assert!(glob_match("**/foo", "XXX/foo"));
  }

  #[test]
  fn globstars() {
    assert!(glob_match("**/*.js", "a/b/c/d.js"));
    assert!(glob_match("**/*.js", "a/b/c.js"));
    assert!(glob_match("**/*.js", "a/b.js"));
    assert!(glob_match("a/b/**/*.js", "a/b/c/d/e/f.js"));
    assert!(glob_match("a/b/**/*.js", "a/b/c/d/e.js"));
    assert!(glob_match("a/b/c/**/*.js", "a/b/c/d.js"));
    assert!(glob_match("a/b/**/*.js", "a/b/c/d.js"));
    assert!(glob_match("a/b/**/*.js", "a/b/d.js"));
    assert!(!glob_match("a/b/**/*.js", "a/d.js"));
    assert!(!glob_match("a/b/**/*.js", "d.js"));

    assert!(!glob_match("**c", "a/b/c"));
    assert!(!glob_match("a/**c", "a/b/c"));
    assert!(!glob_match("a/**z", "a/b/c"));
    assert!(!glob_match("a/**b**/c", "a/b/c/b/c"));
    assert!(!glob_match("a/b/c**/*.js", "a/b/c/d/e.js"));
    assert!(glob_match("a/**/b/**/c", "a/b/c/b/c"));
    assert!(glob_match("a/**b**/c", "a/aba/c"));
    assert!(glob_match("a/**b**/c", "a/b/c"));
    assert!(glob_match("a/b/c**/*.js", "a/b/c/d.js"));

    assert!(!glob_match("a/**/*", "a"));
    assert!(!glob_match("a/**/**/*", "a"));
    assert!(!glob_match("a/**/**/**/*", "a"));
    assert!(!glob_match("**/a", "a/"));
    assert!(glob_match("a/**/*", "a/"));
    assert!(glob_match("a/**/**/*", "a/"));
    assert!(glob_match("a/**/**/**/*", "a/"));
    assert!(!glob_match("**/a", "a/b"));
    assert!(!glob_match("a/**/j/**/z/*.md", "a/b/c/j/e/z/c.txt"));
    assert!(!glob_match("a/**/b", "a/bb"));
    assert!(!glob_match("**/a", "a/c"));
    assert!(!glob_match("**/a", "a/b"));
    assert!(!glob_match("**/a", "a/x/y"));
    assert!(!glob_match("**/a", "a/b/c/d"));
    assert!(glob_match("**", "a"));
    assert!(glob_match("**/a", "a"));
    assert!(glob_match("**", "a/"));
    assert!(glob_match("**/a/**", "a/"));
    assert!(glob_match("a/**", "a/"));
    assert!(glob_match("a/**/**", "a/"));
    assert!(glob_match("**/a", "a/a"));
    assert!(glob_match("**", "a/b"));
    assert!(glob_match("*/*", "a/b"));
    assert!(glob_match("a/**", "a/b"));
    assert!(glob_match("a/**/*", "a/b"));
    assert!(glob_match("a/**/**/*", "a/b"));
    assert!(glob_match("a/**/**/**/*", "a/b"));
    assert!(glob_match("a/**/b", "a/b"));
    assert!(glob_match("**", "a/b/c"));
    assert!(glob_match("**/*", "a/b/c"));
    assert!(glob_match("**/**", "a/b/c"));
    assert!(glob_match("*/**", "a/b/c"));
    assert!(glob_match("a/**", "a/b/c"));
    assert!(glob_match("a/**/*", "a/b/c"));
    assert!(glob_match("a/**/**/*", "a/b/c"));
    assert!(glob_match("a/**/**/**/*", "a/b/c"));
    assert!(glob_match("**", "a/b/c/d"));
    assert!(glob_match("a/**", "a/b/c/d"));
    assert!(glob_match("a/**/*", "a/b/c/d"));
    assert!(glob_match("a/**/**/*", "a/b/c/d"));
    assert!(glob_match("a/**/**/**/*", "a/b/c/d"));
    assert!(glob_match("a/b/**/c/**/*.*", "a/b/c/d.e"));
    assert!(glob_match("a/**/f/*.md", "a/b/c/d/e/f/g.md"));
    assert!(glob_match("a/**/f/**/k/*.md", "a/b/c/d/e/f/g/h/i/j/k/l.md"));
    assert!(glob_match("a/b/c/*.md", "a/b/c/def.md"));
    assert!(glob_match("a/*/c/*.md", "a/bb.bb/c/ddd.md"));
    assert!(glob_match("a/**/f/*.md", "a/bb.bb/cc/d.d/ee/f/ggg.md"));
    assert!(glob_match("a/**/f/*.md", "a/bb.bb/cc/dd/ee/f/ggg.md"));
    assert!(glob_match("a/*/c/*.md", "a/bb/c/ddd.md"));
    assert!(glob_match("a/*/c/*.md", "a/bbbb/c/ddd.md"));

    assert!(glob_match(
      "foo/bar/**/one/**/*.*",
      "foo/bar/baz/one/image.png"
    ));
    assert!(glob_match(
      "foo/bar/**/one/**/*.*",
      "foo/bar/baz/one/two/image.png"
    ));
    assert!(glob_match(
      "foo/bar/**/one/**/*.*",
      "foo/bar/baz/one/two/three/image.png"
    ));
    assert!(!glob_match("a/b/**/f", "a/b/c/d/"));
    // assert!(glob_match("a/**", "a"));
    assert!(glob_match("**", "a"));
    // assert!(glob_match("a{,/**}", "a"));
    assert!(glob_match("**", "a/"));
    assert!(glob_match("a/**", "a/"));
    assert!(glob_match("**", "a/b/c/d"));
    assert!(glob_match("**", "a/b/c/d/"));
    assert!(glob_match("**/**", "a/b/c/d/"));
    assert!(glob_match("**/b/**", "a/b/c/d/"));
    assert!(glob_match("a/b/**", "a/b/c/d/"));
    assert!(glob_match("a/b/**/", "a/b/c/d/"));
    assert!(glob_match("a/b/**/c/**/", "a/b/c/d/"));
    assert!(glob_match("a/b/**/c/**/d/", "a/b/c/d/"));
    assert!(glob_match("a/b/**/**/*.*", "a/b/c/d/e.f"));
    assert!(glob_match("a/b/**/*.*", "a/b/c/d/e.f"));
    assert!(glob_match("a/b/**/c/**/d/*.*", "a/b/c/d/e.f"));
    assert!(glob_match("a/b/**/d/**/*.*", "a/b/c/d/e.f"));
    assert!(glob_match("a/b/**/d/**/*.*", "a/b/c/d/g/e.f"));
    assert!(glob_match("a/b/**/d/**/*.*", "a/b/c/d/g/g/e.f"));
    assert!(glob_match("a/b-*/**/z.js", "a/b-c/z.js"));
    assert!(glob_match("a/b-*/**/z.js", "a/b-c/d/e/z.js"));

    assert!(glob_match("*/*", "a/b"));
    assert!(glob_match("a/b/c/*.md", "a/b/c/xyz.md"));
    assert!(glob_match("a/*/c/*.md", "a/bb.bb/c/xyz.md"));
    assert!(glob_match("a/*/c/*.md", "a/bb/c/xyz.md"));
    assert!(glob_match("a/*/c/*.md", "a/bbbb/c/xyz.md"));

    assert!(glob_match("**/*", "a/b/c"));
    assert!(glob_match("**/**", "a/b/c"));
    assert!(glob_match("*/**", "a/b/c"));
    assert!(glob_match("a/**/j/**/z/*.md", "a/b/c/d/e/j/n/p/o/z/c.md"));
    assert!(glob_match("a/**/z/*.md", "a/b/c/d/e/z/c.md"));
    assert!(glob_match("a/**/c/*.md", "a/bb.bb/aa/b.b/aa/c/xyz.md"));
    assert!(glob_match("a/**/c/*.md", "a/bb.bb/aa/bb/aa/c/xyz.md"));
    assert!(!glob_match("a/**/j/**/z/*.md", "a/b/c/j/e/z/c.txt"));
    assert!(!glob_match("a/**/", "a/b"));
    assert!(!glob_match("a/**/", "a/b/c/d"));
    assert!(!glob_match("a/**/", "a/bb"));
    assert!(!glob_match("a/**/", "a/cb"));
    assert!(glob_match("/**", "/a/b"));
    assert!(glob_match("**/*", "a.b"));
    assert!(glob_match("**/*", "a.js"));
    assert!(glob_match("**/*.js", "a.js"));
    assert!(glob_match("**/*.js", "a/a.js"));
    assert!(glob_match("**/*.js", "a/a/b.js"));
    assert!(glob_match("a/**/b", "a/b"));
    assert!(glob_match("a/**b", "a/b"));
    assert!(glob_match("**/*.md", "a/b.md"));
    assert!(glob_match("**/*", "a/b/c.js"));
    assert!(glob_match("**/*", "a/b/c.txt"));
    assert!(glob_match("a/**/", "a/b/c/d/"));
    assert!(glob_match("**/*", "a/b/c/d/a.js"));
    assert!(glob_match("a/b/**/*.js", "a/b/c/z.js"));
    assert!(glob_match("a/b/**/*.js", "a/b/z.js"));
    assert!(glob_match("**/*", "ab"));
    assert!(glob_match("**/*", "ab/c"));
    assert!(glob_match("**/*", "ab/c/d"));
    assert!(glob_match("**/*", "abc.js"));

    assert!(!glob_match("**/", "a"));
    assert!(!glob_match("**/a/*", "a"));
    assert!(!glob_match("**/a/*/*", "a"));
    assert!(!glob_match("*/a/**", "a"));
    assert!(!glob_match("a/**/*", "a"));
    assert!(!glob_match("a/**/**/*", "a"));
    assert!(!glob_match("**/", "a/b"));
    assert!(!glob_match("**/b/*", "a/b"));
    assert!(!glob_match("**/b/*/*", "a/b"));
    assert!(!glob_match("b/**", "a/b"));
    assert!(!glob_match("**/", "a/b/c"));
    assert!(!glob_match("**/**/b", "a/b/c"));
    assert!(!glob_match("**/b", "a/b/c"));
    assert!(!glob_match("**/b/*/*", "a/b/c"));
    assert!(!glob_match("b/**", "a/b/c"));
    assert!(!glob_match("**/", "a/b/c/d"));
    assert!(!glob_match("**/d/*", "a/b/c/d"));
    assert!(!glob_match("b/**", "a/b/c/d"));
    assert!(glob_match("**", "a"));
    assert!(glob_match("**/**", "a"));
    assert!(glob_match("**/**/*", "a"));
    assert!(glob_match("**/**/a", "a"));
    assert!(glob_match("**/a", "a"));
    assert!(glob_match("**", "a/b"));
    assert!(glob_match("**/**", "a/b"));
    assert!(glob_match("**/**/*", "a/b"));
    assert!(glob_match("**/**/b", "a/b"));
    assert!(glob_match("**/b", "a/b"));
    assert!(glob_match("a/**", "a/b"));
    assert!(glob_match("a/**/*", "a/b"));
    assert!(glob_match("a/**/**/*", "a/b"));
    assert!(glob_match("**", "a/b/c"));
    assert!(glob_match("**/**", "a/b/c"));
    assert!(glob_match("**/**/*", "a/b/c"));
    assert!(glob_match("**/b/*", "a/b/c"));
    assert!(glob_match("**/b/**", "a/b/c"));
    assert!(glob_match("*/b/**", "a/b/c"));
    assert!(glob_match("a/**", "a/b/c"));
    assert!(glob_match("a/**/*", "a/b/c"));
    assert!(glob_match("a/**/**/*", "a/b/c"));
    assert!(glob_match("**", "a/b/c/d"));
    assert!(glob_match("**/**", "a/b/c/d"));
    assert!(glob_match("**/**/*", "a/b/c/d"));
    assert!(glob_match("**/**/d", "a/b/c/d"));
    assert!(glob_match("**/b/**", "a/b/c/d"));
    assert!(glob_match("**/b/*/*", "a/b/c/d"));
    assert!(glob_match("**/d", "a/b/c/d"));
    assert!(glob_match("*/b/**", "a/b/c/d"));
    assert!(glob_match("a/**", "a/b/c/d"));
    assert!(glob_match("a/**/*", "a/b/c/d"));
    assert!(glob_match("a/**/**/*", "a/b/c/d"));

    assert!(glob_match("**/**.txt.js", "/foo/bar.txt.js"));
  }

  #[test]
  fn utf8() {
    assert!(glob_match("フ*/**/*", "フォルダ/aaa.js"));
    assert!(glob_match("フォ*/**/*", "フォルダ/aaa.js"));
    assert!(glob_match("フォル*/**/*", "フォルダ/aaa.js"));
    assert!(glob_match("フ*ル*/**/*", "フォルダ/aaa.js"));
    assert!(glob_match("フォルダ/**/*", "フォルダ/aaa.js"));
  }

  #[test]
  fn negation() {
    assert!(!glob_match("!*", "abc"));
    assert!(!glob_match("!abc", "abc"));
    assert!(!glob_match("*!.md", "bar.md"));
    assert!(!glob_match("foo!.md", "bar.md"));
    assert!(!glob_match("\\!*!*.md", "foo!.md"));
    assert!(!glob_match("\\!*!*.md", "foo!bar.md"));
    assert!(glob_match("*!*.md", "!foo!.md"));
    assert!(glob_match("\\!*!*.md", "!foo!.md"));
    assert!(glob_match("!*foo", "abc"));
    assert!(glob_match("!foo*", "abc"));
    assert!(glob_match("!xyz", "abc"));
    assert!(glob_match("*!*.*", "ba!r.js"));
    assert!(glob_match("*.md", "bar.md"));
    assert!(glob_match("*!*.*", "foo!.md"));
    assert!(glob_match("*!*.md", "foo!.md"));
    assert!(glob_match("*!.md", "foo!.md"));
    assert!(glob_match("*.md", "foo!.md"));
    assert!(glob_match("foo!.md", "foo!.md"));
    assert!(glob_match("*!*.md", "foo!bar.md"));
    assert!(glob_match("*b*.md", "foobar.md"));

    assert!(!glob_match("a!!b", "a"));
    assert!(!glob_match("a!!b", "aa"));
    assert!(!glob_match("a!!b", "a/b"));
    assert!(!glob_match("a!!b", "a!b"));
    assert!(glob_match("a!!b", "a!!b"));
    assert!(!glob_match("a!!b", "a/!!/b"));

    assert!(!glob_match("!a/b", "a/b"));
    assert!(glob_match("!a/b", "a"));
    assert!(glob_match("!a/b", "a.b"));
    assert!(glob_match("!a/b", "a/a"));
    assert!(glob_match("!a/b", "a/c"));
    assert!(glob_match("!a/b", "b/a"));
    assert!(glob_match("!a/b", "b/b"));
    assert!(glob_match("!a/b", "b/c"));

    assert!(!glob_match("!abc", "abc"));
    assert!(glob_match("!!abc", "abc"));
    assert!(!glob_match("!!!abc", "abc"));
    assert!(glob_match("!!!!abc", "abc"));
    assert!(!glob_match("!!!!!abc", "abc"));
    assert!(glob_match("!!!!!!abc", "abc"));
    assert!(!glob_match("!!!!!!!abc", "abc"));
    assert!(glob_match("!!!!!!!!abc", "abc"));

    assert!(!glob_match("!*", "a"));
    assert!(!glob_match("!*", "a.b"));
    assert!(!glob_match("!*/*", "a/a"));
    assert!(!glob_match("!*/*", "a/b"));
    assert!(!glob_match("!*/*", "a/c"));
    assert!(!glob_match("!*/*", "b/a"));
    assert!(!glob_match("!*/*", "b/b"));
    assert!(!glob_match("!*/*", "b/c"));
    assert!(!glob_match("!*/b", "a/b"));
    assert!(!glob_match("!*/b", "b/b"));
    assert!(!glob_match("!*/c", "a/c"));
    assert!(!glob_match("!*/c", "a/c"));
    assert!(!glob_match("!*/c", "b/c"));
    assert!(!glob_match("!*/c", "b/c"));
    assert!(!glob_match("!*a*", "bar"));
    assert!(!glob_match("!*a*", "fab"));
    assert!(!glob_match("!a/*", "a/a"));
    assert!(!glob_match("!a/*", "a/b"));
    assert!(!glob_match("!a/*", "a/c"));
    assert!(!glob_match("!f*b", "fab"));
    assert!(glob_match("!*", "a/a"));
    assert!(glob_match("!*", "a/b"));
    assert!(glob_match("!*", "a/c"));
    assert!(glob_match("!*", "b/a"));
    assert!(glob_match("!*", "b/b"));
    assert!(glob_match("!*", "b/c"));
    assert!(glob_match("!*/*", "a"));
    assert!(glob_match("!*/*", "a.b"));
    assert!(glob_match("!*/b", "a"));
    assert!(glob_match("!*/b", "a.b"));
    assert!(glob_match("!*/b", "a/a"));
    assert!(glob_match("!*/b", "a/c"));
    assert!(glob_match("!*/b", "b/a"));
    assert!(glob_match("!*/b", "b/c"));
    assert!(glob_match("!*/c", "a"));
    assert!(glob_match("!*/c", "a.b"));
    assert!(glob_match("!*/c", "a/a"));
    assert!(glob_match("!*/c", "a/b"));
    assert!(glob_match("!*/c", "b/a"));
    assert!(glob_match("!*/c", "b/b"));
    assert!(glob_match("!*a*", "foo"));
    assert!(glob_match("!a/*", "a"));
    assert!(glob_match("!a/*", "a.b"));
    assert!(glob_match("!a/*", "b/a"));
    assert!(glob_match("!a/*", "b/b"));
    assert!(glob_match("!a/*", "b/c"));
    assert!(glob_match("!f*b", "bar"));
    assert!(glob_match("!f*b", "foo"));

    assert!(!glob_match("!.md", ".md"));
    assert!(glob_match("!**/*.md", "a.js"));
    assert!(glob_match("!**/*.md", "c.txt"));
    assert!(glob_match("!*.md", "a.js"));
    assert!(!glob_match("!*.md", "b.md"));
    assert!(glob_match("!*.md", "c.txt"));
    assert!(!glob_match("!*.md", "abc.md"));
    assert!(glob_match("!*.md", "abc.txt"));
    assert!(!glob_match("!*.md", "foo.md"));
    assert!(glob_match("!.md", "foo.md"));

    assert!(glob_match("!*.md", "a.js"));
    assert!(glob_match("!*.md", "b.txt"));
    assert!(!glob_match("!*.md", "c.md"));
    assert!(!glob_match("!a/*/a.js", "a/a/a.js"));
    assert!(!glob_match("!a/*/a.js", "a/b/a.js"));
    assert!(!glob_match("!a/*/a.js", "a/c/a.js"));
    assert!(!glob_match("!a/*/*/a.js", "a/a/a/a.js"));
    assert!(glob_match("!a/*/*/a.js", "b/a/b/a.js"));
    assert!(glob_match("!a/*/*/a.js", "c/a/c/a.js"));
    assert!(!glob_match("!a/a*.txt", "a/a.txt"));
    assert!(glob_match("!a/a*.txt", "a/b.txt"));
    assert!(glob_match("!a/a*.txt", "a/c.txt"));
    assert!(!glob_match("!a.a*.txt", "a.a.txt"));
    assert!(glob_match("!a.a*.txt", "a.b.txt"));
    assert!(glob_match("!a.a*.txt", "a.c.txt"));
    assert!(!glob_match("!a/*.txt", "a/a.txt"));
    assert!(!glob_match("!a/*.txt", "a/b.txt"));
    assert!(!glob_match("!a/*.txt", "a/c.txt"));

    assert!(glob_match("!*.md", "a.js"));
    assert!(glob_match("!*.md", "b.txt"));
    assert!(!glob_match("!*.md", "c.md"));
    assert!(glob_match("!**/a.js", "a/a/b.js"));
    assert!(!glob_match("!a/**/a.js", "a/a/a/a.js"));
    assert!(glob_match("!a/**/a.js", "b/a/b/a.js"));
    assert!(glob_match("!a/**/a.js", "c/a/c/a.js"));
    assert!(glob_match("!**/*.md", "a/b.js"));
    assert!(glob_match("!**/*.md", "a.js"));
    assert!(!glob_match("!**/*.md", "a/b.md"));
    assert!(!glob_match("**/*.md", "a/b.js"));
    assert!(!glob_match("**/*.md", "a.js"));
    assert!(glob_match("**/*.md", "a/b.md"));
    assert!(glob_match("**/*.md", "a.md"));
    assert!(glob_match("!**/*.md", "a/b.js"));
    assert!(glob_match("!**/*.md", "a.js"));
    assert!(!glob_match("!**/*.md", "a/b.md"));
    assert!(glob_match("!*.md", "a/b.js"));
    assert!(glob_match("!*.md", "a.js"));
    assert!(glob_match("!*.md", "a/b.md"));
    assert!(!glob_match("!*.md", "a.md"));
    assert!(glob_match("!**/*.md", "a.js"));
    assert!(glob_match("!**/*.md", "c.txt"));
  }

  #[test]
  fn question_mark() {
    assert!(glob_match("?", "a"));
    assert!(!glob_match("?", "aa"));
    assert!(!glob_match("?", "ab"));
    assert!(!glob_match("?", "aaa"));
    assert!(!glob_match("?", "abcdefg"));

    assert!(!glob_match("??", "a"));
    assert!(glob_match("??", "aa"));
    assert!(glob_match("??", "ab"));
    assert!(!glob_match("??", "aaa"));
    assert!(!glob_match("??", "abcdefg"));

    assert!(!glob_match("???", "a"));
    assert!(!glob_match("???", "aa"));
    assert!(!glob_match("???", "ab"));
    assert!(glob_match("???", "aaa"));
    assert!(!glob_match("???", "abcdefg"));

    assert!(!glob_match("a?c", "aaa"));
    assert!(glob_match("a?c", "aac"));
    assert!(glob_match("a?c", "abc"));
    assert!(!glob_match("ab?", "a"));
    assert!(!glob_match("ab?", "aa"));
    assert!(!glob_match("ab?", "ab"));
    assert!(!glob_match("ab?", "ac"));
    assert!(!glob_match("ab?", "abcd"));
    assert!(!glob_match("ab?", "abbb"));
    assert!(glob_match("a?b", "acb"));

    assert!(!glob_match("a/?/c/?/e.md", "a/bb/c/dd/e.md"));
    assert!(glob_match("a/??/c/??/e.md", "a/bb/c/dd/e.md"));
    assert!(!glob_match("a/??/c.md", "a/bbb/c.md"));
    assert!(glob_match("a/?/c.md", "a/b/c.md"));
    assert!(glob_match("a/?/c/?/e.md", "a/b/c/d/e.md"));
    assert!(!glob_match("a/?/c/???/e.md", "a/b/c/d/e.md"));
    assert!(glob_match("a/?/c/???/e.md", "a/b/c/zzz/e.md"));
    assert!(!glob_match("a/?/c.md", "a/bb/c.md"));
    assert!(glob_match("a/??/c.md", "a/bb/c.md"));
    assert!(glob_match("a/???/c.md", "a/bbb/c.md"));
    assert!(glob_match("a/????/c.md", "a/bbbb/c.md"));
  }
}
