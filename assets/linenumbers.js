/* ============================================================
   Line numbers for code samples.
   Link with: <script src="../assets/linenumbers.js" defer></script>

   Wraps each line of every <pre><code> block in a <span class="code-line">.
   The numbers themselves are drawn by CSS (::before counter in style.css),
   so they are NOT part of the selectable text — copying code copies code only.
   ============================================================ */
(function () {
  function number(code) {
    if (code.querySelector('.code-line')) return; // idempotent: don't double-wrap

    var lines = code.innerHTML.split('\n');
    // A trailing newline in the source yields a stray empty final line — drop it.
    if (lines.length > 1 && lines[lines.length - 1].trim() === '') lines.pop();

    // Join with '' (not '\n'): each .code-line is display:block and supplies its
    // own line break, so an extra newline inside <pre> would double the spacing.
    // A line authored as <span class="add">…</span> (whole line) is promoted to a
    // highlighted .code-line.add — used to flag lines added/changed in a step.
    code.innerHTML = lines.map(function (line) {
      var cls = 'code-line';
      // Indentation may sit inside OR outside the span: leading/trailing
      // whitespace around the wrapper is tolerated and moved back inside, so an
      // author who writes "    <span class="add">x</span>" (indent outside the
      // span) still gets the highlight instead of silently losing it.
      var added = line.match(/^(\s*)<span class="add">([\s\S]*)<\/span>(\s*)$/);
      if (added) { cls += ' add'; line = added[1] + added[2] + added[3]; }
      return '<span class="' + cls + '">' + (line.length ? line : ' ') + '</span>';
    }).join('');
  }

  function init() {
    // Skip shell/terminal blocks (<pre class="shell">) — only source gets numbered.
    document.querySelectorAll('pre:not(.shell) > code').forEach(number);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
