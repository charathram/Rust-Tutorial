/* ============================================================
   Left-margin lesson index for lesson pages.
   Link with: <script src="../assets/nav.js" defer></script>

   Single source of truth for the course outline: edit LESSONS / REFS
   below when you add a lesson, and every page's sidebar updates.
   The script injects <nav class="lesson-nav"> as the first body child
   and marks the current page active. Styling lives in style.css.

   Paths are relative to a page inside lessons/ — this component is
   intended for lesson pages only.
   ============================================================ */
(function () {
  var LESSONS = [
    { n: '1', title: 'Build a CLI: rwc', file: '0001-build-rwc-cli.html' },
    { n: '2', title: 'Error Handling',   file: '0002-error-handling.html' },
    { n: '3', title: 'Tests',            file: '0003-tests.html' },
    { n: '4', title: 'Borrowing &amp; references', file: '0004-borrowing.html' },
    { n: '5', title: 'Reading borrow-checker errors', file: '0005-borrow-checker-errors.html' },
    { n: '6', title: 'Structs &amp; enums', file: '0006-structs-enums.html' },
    { n: '7', title: 'Pattern matching', file: '0007-pattern-matching.html' },
    { n: '8', title: 'Strings &amp; slices', file: '0008-strings-and-slices.html' },
    { n: '9', title: 'Lifetimes',        file: '0009-lifetimes.html' },
    { n: '10', title: 'Cargo &amp; dependencies', file: '0010-cargo-dependencies.html' },
    { n: '11', title: 'Flags with clap', file: '0011-clap-flags.html' }
  ];
  var REFS = [
    { title: 'Glossary', href: '../reference/glossary.html' }
  ];
  var SYLLABUS = { title: 'Syllabus', file: '000-syllabus.html' };
  // Module 0 primer — set apart from the numbered, project-anchored lessons.
  var ORIENTATION = { title: 'Orientation', file: '0000-ownership-and-moves.html', glyph: '○' };

  var here = location.pathname.split('/').pop();

  function lessonItem(l) {
    var active = (l.file === here) ? ' class="active"' : '';
    return '<li><a href="./' + l.file + '"' + active + '>' +
             '<span class="ln-num">' + l.n + '</span>' +
             '<span class="ln-label">' + l.title + '</span>' +
           '</a></li>';
  }
  function refItem(r) {
    return '<li><a href="' + r.href + '"><span class="ln-label">' + r.title + '</span></a></li>';
  }

  function build() {
    var nav = document.createElement('nav');
    nav.className = 'lesson-nav';
    nav.setAttribute('aria-label', 'Course navigation');
    function fixedItem(entry) {
      var active = (entry.file === here) ? ' class="active"' : '';
      return '<li><a href="./' + entry.file + '"' + active + '>' +
               '<span class="ln-num">' + entry.glyph + '</span>' +
               '<span class="ln-label">' + entry.title + '</span>' +
             '</a></li>';
    }
    var syllabusItem = fixedItem({ file: SYLLABUS.file, glyph: '§', title: SYLLABUS.title });
    var orientationItem = fixedItem(ORIENTATION);
    nav.innerHTML =
      '<p class="ln-head">Rust course</p>' +
      '<ul class="ln-list">' + syllabusItem + orientationItem + '</ul>' +
      '<p class="ln-sub">Lessons</p>' +
      '<ul class="ln-list">' + LESSONS.map(lessonItem).join('') + '</ul>' +
      '<p class="ln-sub">Reference</p>' +
      '<ul class="ln-list">' + REFS.map(refItem).join('') + '</ul>';
    return nav;
  }

  function init() {
    if (document.querySelector('.lesson-nav')) return; // idempotent
    document.body.insertBefore(build(), document.body.firstChild);
    document.body.classList.add('has-nav');
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
