/* ============================================================
   Reusable quiz widget for lessons (retrieval practice).
   Link with: <script src="../assets/quiz.js" defer></script>

   Markup contract:
   <div class="quiz">
     <p class="quiz-q">Question text?</p>
     <button class="quiz-opt" data-correct>Right answer</button>
     <button class="quiz-opt">Wrong answer</button>
     <p class="quiz-fb ok"  hidden>Why the right answer is right.</p>
     <p class="quiz-fb no"  hidden>What the common mistake reveals.</p>
   </div>

   Behaviour: first click locks the question, marks the chosen option
   correct/wrong, reveals the correct option, and shows feedback —
   an immediate, automatic feedback loop. One attempt, by design.
   ============================================================ */
(function () {
  // Shuffle the option buttons in the DOM so the correct answer (identified by
  // the data-correct attribute, not by position) doesn't always sit first.
  // Fisher–Yates; reinserts the buttons in random order before the first
  // feedback paragraph, leaving the .quiz-fb elements untouched.
  function shuffleOptions(quiz, opts) {
    var anchor = quiz.querySelector('.quiz-fb');
    for (var i = opts.length - 1; i > 0; i--) {
      var j = Math.floor(Math.random() * (i + 1));
      var tmp = opts[i]; opts[i] = opts[j]; opts[j] = tmp;
    }
    opts.forEach(function (o) { quiz.insertBefore(o, anchor); });
  }

  function wire(quiz) {
    var opts = Array.prototype.slice.call(quiz.querySelectorAll('.quiz-opt'));
    var fbOk = quiz.querySelector('.quiz-fb.ok');
    var fbNo = quiz.querySelector('.quiz-fb.no');

    shuffleOptions(quiz, opts);

    opts.forEach(function (opt) {
      opt.addEventListener('click', function () {
        if (quiz.dataset.answered) return;
        quiz.dataset.answered = '1';

        var choseCorrect = opt.hasAttribute('data-correct');

        opts.forEach(function (o) {
          o.disabled = true;
          if (o.hasAttribute('data-correct')) o.classList.add('correct');
        });
        if (!choseCorrect) opt.classList.add('wrong');

        var fb = choseCorrect ? fbOk : fbNo;
        // if no specific "wrong" feedback exists, fall back to the ok one
        if (!fb) fb = fbOk;
        if (fb) {
          fb.hidden = false;
          fb.classList.add('show');
          fb.textContent = (choseCorrect ? '✓  ' : '✗  ') + fb.textContent;
        }
      });
    });
  }

  function init() {
    document.querySelectorAll('.quiz').forEach(wire);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
