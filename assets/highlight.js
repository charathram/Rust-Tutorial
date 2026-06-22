/* ============================================================
   Lightweight Rust syntax highlighter.
   Link AFTER linenumbers.js: <script src="../assets/highlight.js" defer></script>

   Runs on every <pre:not(.shell)> > code (Rust only — shell blocks keep
   their plain styling). Walks TEXT NODES, so existing markup is preserved:
   line wrappers (.code-line), added-line highlights (.add), and the manual
   comment (.c) / error (.err) spans are left untouched — text inside .c/.err
   is skipped so comments aren't re-tokenized.

   Tokens get .tok-* classes (styled in style.css). Generated spans are not
   selectable as separate content, and the original code text is preserved
   verbatim, so copying still yields clean code.
   ============================================================ */
(function () {
  var KEYWORDS = {};
  ('as async await break const continue crate dyn else enum extern false fn for ' +
   'if impl in let loop match mod move mut pub ref return self Self static struct ' +
   'super trait true type unsafe use where while box').split(' ')
    .forEach(function (k) { KEYWORDS[k] = 1; });

  var PRIMS = {};
  ('i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 bool char str ' +
   'String Vec Option Result Box String').split(' ')
    .forEach(function (t) { PRIMS[t] = 1; });

  var TOKEN = new RegExp(
    '(\\/\\/[^\\n]*)' +                                    // 1 line comment
    '|("(?:\\\\.|[^"\\\\])*")' +                           // 2 string
    "|('(?:\\\\.|[^'\\\\])')" +                            // 3 char literal
    "|('[A-Za-z_]\\w*)" +                                  // 4 lifetime
    '|(\\b\\d[\\d_]*(?:\\.\\d[\\d_]*)?(?:[iu](?:8|16|32|64|128|size)|f(?:32|64))?\\b)' + // 5 number
    '|([A-Za-z_]\\w*!)' +                                  // 6 macro
    '|([A-Za-z_]\\w*)' +                                   // 7 word
    '|([\\s\\S])',                                         // 8 anything else
    'g'
  );

  function esc(s) {
    return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }
  function span(cls, text) { return '<span class="' + cls + '">' + esc(text) + '</span>'; }

  function highlight(code) {
    var out = '', m;
    TOKEN.lastIndex = 0;
    while ((m = TOKEN.exec(code)) !== null) {
      if (m[1] !== undefined)      out += span('tok-com', m[1]);
      else if (m[2] !== undefined) out += span('tok-str', m[2]);
      else if (m[3] !== undefined) out += span('tok-str', m[3]);
      else if (m[4] !== undefined) out += span('tok-life', m[4]);
      else if (m[5] !== undefined) out += span('tok-num', m[5]);
      else if (m[6] !== undefined) out += span('tok-mac', m[6]);
      else if (m[7] !== undefined) {
        var w = m[7];
        if (KEYWORDS[w]) out += span('tok-kw', w);
        else if (PRIMS[w] || /^[A-Z]/.test(w)) out += span('tok-ty', w);
        else out += esc(w);
      } else {
        out += esc(m[8]);
      }
    }
    return out;
  }

  function skip(node, root) {
    var p = node.parentNode;
    while (p && p !== root) {
      if (p.classList && (p.classList.contains('c') || p.classList.contains('err'))) return true;
      p = p.parentNode;
    }
    return false;
  }

  function process(code) {
    if (code.dataset.hl) return;     // idempotent
    code.dataset.hl = '1';
    var walker = document.createTreeWalker(code, NodeFilter.SHOW_TEXT, null);
    var texts = [], n;
    while ((n = walker.nextNode())) texts.push(n);
    texts.forEach(function (node) {
      if (skip(node, code) || !node.nodeValue) return;
      var tmp = document.createElement('span');
      tmp.innerHTML = highlight(node.nodeValue);
      var frag = document.createDocumentFragment();
      while (tmp.firstChild) frag.appendChild(tmp.firstChild);
      node.parentNode.replaceChild(frag, node);
    });
  }

  function init() {
    document.querySelectorAll('pre:not(.shell) > code').forEach(process);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
