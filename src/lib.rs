use std::path::is_separator;

/// The current matching state.
#[derive(Clone, Copy, Debug, Default)]
struct State {
  // These store character indices into the glob and path strings.
  path_index: usize,
  glob_index: usize,

  // When we hit a * or **, we store the state for backtracking.
  wildcard: Wildcard,
  globstar: Wildcard,

  brace_deep: u8,
}

/// Wildcard state such as * or **.
#[derive(Clone, Copy, Debug, Default)]
struct Wildcard {
  glob_index: u32,
  path_index: u32,

  brace_deep: u8,
}

/// Result type of matching braces.
#[derive(PartialEq)]
enum BraceState {
  Comma,
  Invalid,
  EndBrace,
}

/// Matching state stack of braces.
struct BraceStack {
  stack: [State; 10],
  length: u8,
  longest_brace_match: u32,
}

pub fn glob_match(glob: &str, path: &str) -> bool {
  glob_match_internal(glob.as_bytes(), path.as_bytes())
}

fn glob_match_internal(glob: &[u8], path: &[u8]) -> bool {
  // This algorithm is based on https://research.swtch.com/glob
  let mut state = State::default();

  // Store the state when we see an opening '{' brace in a stack.
  // Up to 10 nested braces are supported.
  let mut brace_stack = BraceStack::default();

  // First, check if the pattern is negated with a leading '!' character.
  // Multiple negations can occur.
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
            // Coalesce multiple ** segments into one.
            state.skip_globstars(&glob);
          }

          state.wildcard.glob_index = state.glob_index as u32;
          state.wildcard.path_index = state.path_index as u32 + 1;

          let mut in_globstar = false;

          // ** allows path separators, whereas * does not.
          // However, ** must be a full path component, i.e. a/**/b not a**b.
          if is_globstar {
            state.glob_index += 2;

            // /**/ or /**
            let is_end_invalid = state.glob_index != glob.len();

            if !is_end_invalid
              || ((state.glob_index < 3 || glob[state.glob_index - 3] == b'/')
                && glob[state.glob_index] == b'/')
            {
              if is_end_invalid {
                // Matched a full /**/ segment. If the last character in the path was a separator,
                // skip the separator in the glob so we search for the next character.
                // In effect, this makes the whole segment optional so that a/**/b matches a/b.
                state.glob_index += 1;
              }

              state.skip_to_separator(&path, is_end_invalid);
              in_globstar = true;
            }
          } else {
            state.glob_index += 1;
          }

          // If we are in a * segment and hit a separator,
          // either jump back to a previous ** or end the wildcard.
          if !in_globstar
            && state.path_index < path.len()
            && is_separator(path[state.path_index] as char)
          {
            state.wildcard = state.globstar;
          }

          // If the next char is a special brace separator,
          // skip to the end of the braces so we don't try to match it.
          if brace_stack.length > 0
            && state.glob_index < glob.len()
            && matches!(glob[state.glob_index], b',' | b'}')
          {
            if state.skip_braces(glob, false) == BraceState::Invalid {
              // invalid pattern!
              return false;
            }
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
          let c = path[state.path_index];

          // Check if the character class is negated.
          let mut negated = false;
          if state.glob_index < glob.len() && matches!(glob[state.glob_index], b'^' | b'!') {
            negated = true;
            state.glob_index += 1;
          }

          // Try each range.
          let mut first = true;
          let mut is_match = false;
          while state.glob_index < glob.len() && (first || glob[state.glob_index] != b']') {
            let mut low = glob[state.glob_index];
            if !unescape(&mut low, &glob, &mut state.glob_index) {
              // Invalid pattern!
              return false;
            }
            state.glob_index += 1;

            // If there is a - and the following character is not ], read the range end character.
            let high = if state.glob_index + 1 < glob.len()
              && glob[state.glob_index] == b'-'
              && glob[state.glob_index + 1] != b']'
            {
              state.glob_index += 1;
              let mut high = glob[state.glob_index];
              if !unescape(&mut high, &glob, &mut state.glob_index) {
                // Invalid pattern!
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
            // invalid pattern!
            return false;
          }

          state.glob_index += 1;
          if is_match != negated {
            state.path_index += 1;
            continue;
          }
        }
        b'{' if state.path_index < path.len() => {
          if brace_stack.length as usize >= brace_stack.stack.len() {
            // Invalid pattern! Too many nested braces.
            return false;
          }

          // Push old state to the stack, and reset current state.
          state = brace_stack.push(&state);
          continue;
        }
        b'}' if brace_stack.length > 0 => {
          // If we hit the end of the braces, we matched the last option.
          brace_stack.longest_brace_match =
            brace_stack.longest_brace_match.max(state.path_index as u32);

          state.glob_index += 1;
          state = brace_stack.pop(&state);
          continue;
        }
        b',' if brace_stack.length > 0 => {
          // If we hit a comma, we matched one of the options!
          // But we still need to check the others in case there is a longer match.
          brace_stack.longest_brace_match =
            brace_stack.longest_brace_match.max(state.path_index as u32);

          state.path_index = brace_stack.last().path_index;
          state.glob_index += 1;
          state.wildcard = Wildcard::default();
          state.globstar = Wildcard::default();
          continue;
        }
        mut c if state.path_index < path.len() => {
          // Match escaped characters as literals.
          if !unescape(&mut c, &glob, &mut state.glob_index) {
            // Invalid pattern!
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

    // If we didn't match, restore state to the previous star pattern.
    // **/*/abc  /abc
    if state.wildcard.path_index > 0 && state.wildcard.path_index as usize <= path.len() {
      state.backtrack();
      continue;
    }

    if brace_stack.length > 0 {
      // If in braces, find next option and reset path to index where we saw the '{'
      match state.skip_braces(&glob, true) {
        BraceState::Comma => {
          state.path_index = brace_stack.last().path_index;
          continue;
        }
        BraceState::Invalid => {
          return false;
        }
        BraceState::EndBrace => {
          // Hit the end. Pop the stack.
          // If we matched a previous option, use that.
          if brace_stack.longest_brace_match > 0 {
            state = brace_stack.pop(&state);
            continue;
          } else {
            // Didn't match. Restore state, and check if we need to jump back to a star pattern.
            state = *brace_stack.last();
            brace_stack.length -= 1;

            if state.wildcard.path_index > 0 && state.wildcard.path_index as usize <= path.len() {
              state.backtrack();
              continue;
            }
          }
        }
      }
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
      // Invalid pattern!
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
    self.glob_index = self.wildcard.glob_index as usize;
    self.path_index = self.wildcard.path_index as usize;
  }

  #[inline(always)]
  fn skip_globstars(&mut self, glob: &[u8]) {
    // Coalesce multiple ** segments into one.
    let mut glob_index = self.glob_index + 2;

    // Only entire path components can be skipped.
    // i.e. '**' in the pattern 'a/**/**/b' can match multiple path components, but in 'a/**/**b' it cannot.
    while glob_index + 4 <= glob.len()
      && unsafe { glob.get_unchecked(glob_index..glob_index + 4) } == b"/**/"
    {
      glob_index += 3;
    }

    // A trailing '**' can also match multiple trailing path components.
    // i.e. the pattern '**/**/**' is valid and can match multiple components.
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

    self.wildcard.path_index = path_index as u32;
    self.globstar = self.wildcard;
  }

  fn skip_braces(&mut self, glob: &[u8], stop_on_comma: bool) -> BraceState {
    let mut braces = 1;
    let mut in_brackets = false;

    while self.glob_index < glob.len() && braces > 0 {
      match glob[self.glob_index] {
        // Skip nested braces.
        b'{' if !in_brackets => braces += 1,
        b'}' if !in_brackets => braces -= 1,
        b',' if stop_on_comma && braces == 1 && !in_brackets => {
          self.glob_index += 1;
          return BraceState::Comma;
        }
        c @ (b'*' | b'?' | b'[') if !in_brackets => {
          if c == b'[' {
            in_brackets = true;
          }

          if c == b'*' {
            if self.glob_index + 1 < glob.len() && glob[self.glob_index + 1] == b'*' {
              self.skip_globstars(glob);
            }
          }
        }
        b']' => in_brackets = false,
        b'\\' => self.glob_index += 1,
        _ => {}
      }
      self.glob_index += 1;
    }

    if braces != 0 {
      return BraceState::Invalid;
    }

    BraceState::EndBrace
  }
}

impl Default for BraceStack {
  #[inline]
  fn default() -> Self {
    // Manual implementation is faster than the automatically derived one.
    BraceStack {
      stack: [State::default(); 10],
      length: 0,
      longest_brace_match: 0,
    }
  }
}

impl BraceStack {
  #[inline(always)]
  fn push(&mut self, state: &State) -> State {
    // Push old state to the stack, and reset current state.
    self.stack[self.length as usize] = *state;
    self.length += 1;

    State {
      path_index: state.path_index,
      glob_index: state.glob_index + 1,
      wildcard: state.wildcard,
      globstar: state.globstar,
      brace_deep: self.length as u8,
    }
  }

  #[inline(always)]
  fn pop(&mut self, state: &State) -> State {
    self.length -= 1;
    let state = State {
      path_index: self.longest_brace_match as usize,
      glob_index: state.glob_index,

      // But restore star state if needed later.
      wildcard: self.stack[self.length as usize].wildcard,
      globstar: self.stack[self.length as usize].globstar,
    };

    if self.length == 0 {
      self.longest_brace_match = 0;
    }

    state
  }

  #[inline(always)]
  fn last(&self) -> &State {
    &self.stack[self.length as usize - 1]
  }
}
