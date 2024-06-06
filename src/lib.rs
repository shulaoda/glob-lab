use std::path::is_separator;

#[derive(Clone, Copy, Debug, Default)]
struct State {
  path_index: usize,
  glob_index: usize,

  wildcard: Wildcard,
  globstar: Wildcard,
}

#[derive(Clone, Copy, Debug, Default)]
struct Wildcard {
  glob_index: usize,
  path_index: usize,
}

#[derive(Debug, Default)]
struct BraceState {
  length: u8,

  stack: [Option<State>; 10],

  wildcard: Option<BraceWildcard>,
  globstar: Option<BraceWildcard>,
}

#[derive(Clone, Copy, Debug, Default)]
struct BraceWildcard {
  stack: [Option<State>; 10],
  length: u8,
}

pub fn glob_match(glob: &str, path: &str) -> bool {
  glob_match_internal(glob.as_bytes(), path.as_bytes())
}

fn glob_match_internal(glob: &[u8], path: &[u8]) -> bool {
  let mut state = State::default();
  let mut brace_state = BraceState::default();

  let mut negated = false;
  while state.glob_index < glob.len() && glob[state.glob_index] == b'!' {
    negated = !negated;
    state.glob_index += 1;
  }

  while state.glob_index < glob.len() || state.path_index < path.len() {
    if state.glob_index < glob.len() {
      match glob[state.glob_index] {
        b'*' => {
          let is_globstar = state.glob_index + 1 < glob.len() && glob[state.glob_index + 1] == b'*';
          if is_globstar {
            state.skip_globstars(glob);
          }

          state.wildcard.glob_index = state.glob_index;
          state.wildcard.path_index = state.path_index + 1;

          let mut in_globstar = false;
          if is_globstar {
            state.glob_index += 2;

            let is_end_invalid = state.glob_index != glob.len();

            if !is_end_invalid
              || ((state.glob_index < 3 || glob[state.glob_index - 3] == b'/')
                && glob[state.glob_index] == b'/')
            {
              if is_end_invalid {
                state.glob_index += 1;
              }

              state.skip_to_separator(path, is_end_invalid);
              in_globstar = true;
            }
          } else {
            state.glob_index += 1;
          }

          if !in_globstar
            && state.path_index < path.len()
            && is_separator(path[state.path_index] as char)
          {
            state.wildcard = state.globstar;
          }

          continue;
        }
        b'?' if state.path_index < path.len() => {
          if !is_separator(path[state.path_index] as char) {
            state.glob_index += 1;
            state.path_index += 1;
            continue;
          }
        }
        b'[' if state.path_index < path.len() => {
          state.glob_index += 1;

          let mut negated = false;
          if state.glob_index < glob.len() && matches!(glob[state.glob_index], b'^' | b'!') {
            negated = true;
            state.glob_index += 1;
          }

          let mut first = true;
          let mut is_match = false;
          let c = path[state.path_index];
          while state.glob_index < glob.len() && (first || glob[state.glob_index] != b']') {
            let mut low = glob[state.glob_index];
            if !unescape(&mut low, glob, &mut state.glob_index) {
              return false;
            }
            state.glob_index += 1;

            let high = if state.glob_index + 1 < glob.len()
              && glob[state.glob_index] == b'-'
              && glob[state.glob_index + 1] != b']'
            {
              state.glob_index += 1;

              let mut high = glob[state.glob_index];
              if !unescape(&mut high, glob, &mut state.glob_index) {
                return false;
              }

              state.glob_index += 1;
              high
            } else {
              low
            };

            if low <= c && c <= high {
              is_match = true;
            }

            first = false;
          }

          if state.glob_index >= glob.len() {
            return false;
          }

          state.glob_index += 1;
          if is_match != negated {
            state.path_index += 1;
            continue;
          }
        }
        mut c if state.path_index < path.len() => {
          if !unescape(&mut c, glob, &mut state.glob_index) {
            return false;
          }

          let is_match = if c == b'/' {
            is_separator(path[state.path_index] as char)
          } else {
            path[state.path_index] == c
          };

          if is_match {
            state.glob_index += 1;
            state.path_index += 1;

            if c == b'/' {
              state.wildcard = state.globstar;
            }

            continue;
          }
        }
        _ => {}
      }
    }

    if state.wildcard.path_index > 0 && state.wildcard.path_index <= path.len() {
      state.backtrack();
      continue;
    }

    return negated;
  }

  !negated
}

#[inline(always)]
fn unescape(c: &mut u8, glob: &[u8], glob_index: &mut usize) -> bool {
  if *c == b'\\' {
    *glob_index += 1;
    if *glob_index >= glob.len() {
      return false;
    }
    *c = match glob[*glob_index] {
      b'a' => b'\x61',
      b'b' => b'\x08',
      b'n' => b'\n',
      b'r' => b'\r',
      b't' => b'\t',
      c => c,
    }
  }
  true
}

impl State {
  #[inline(always)]
  fn backtrack(&mut self) {
    self.glob_index = self.wildcard.glob_index;
    self.path_index = self.wildcard.path_index;
  }

  #[inline(always)]
  fn skip_globstars(&mut self, glob: &[u8]) {
    let mut glob_index = self.glob_index + 2;

    while glob_index + 4 <= glob.len()
      && unsafe { glob.get_unchecked(glob_index..glob_index + 4) } == b"/**/"
    {
      glob_index += 3;
    }

    if glob_index + 3 == glob.len()
      && unsafe { glob.get_unchecked(glob_index..glob_index + 3) } == b"/**"
    {
      glob_index += 3;
    }

    self.glob_index = glob_index - 2;
  }

  #[inline(always)]
  fn skip_to_separator(&mut self, path: &[u8], is_end_invalid: bool) {
    if self.path_index == path.len() {
      self.wildcard.path_index += 1;
      return;
    }

    let mut path_index = self.path_index;
    while path_index < path.len() {
      if is_separator(path[path_index] as char) {
        break;
      }
      path_index += 1;
    }

    if is_end_invalid || path_index != path.len() {
      path_index += 1;
    }

    self.wildcard.path_index = path_index;
    self.globstar = self.wildcard;
  }
}
