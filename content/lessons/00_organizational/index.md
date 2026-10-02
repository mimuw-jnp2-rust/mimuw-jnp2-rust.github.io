+++
title = "Organizational lesson"
date = 2026-10-02
weight = 0
[extra]
lesson_date = 2026-10-08
+++

# Rust course

We will be using [Github Classroom](https://classroom.github.com) for task submission and [Discord (link TODO)](TODO) for discussions.

Our learning/teaching materials are going to be:

- the content of this site - it contains explanations and questions tailored towards MIMUW students,
- well-known Rust teaching materials (["The Book"](https://doc.rust-lang.org/stable/book/), ["Rust by Example"](https://doc.rust-lang.org/rust-by-example/index.html)) that complement this site,
- explanations/deeper insight during class and many examples of Rust code.

## Final grade

- 1/3 of the grade is based on small tasks.
  - There will be 9 tasks.
  - Each task will be graded on a scale of 0 to 3.
  - You can get up to 24 points from the small tasks. It means that it is enough to do 8 tasks with a score of 3 points.
  - You can solve the task between the end of the lesson and strictly before the start of the next lesson. The deadline is different for different lab groups.
  - Note: we will have 14 (_TODO: verify this_) classes in total, so you can expect a task every week or two.
- 1/3 of the grade is based on a big project. You can choose a topic yourself, but it must be accepted by us. The project can be done in groups of two (or bigger, if ambitious enough). The grading is as follows:
  1. Usability - 2 points.
  2. Usage of Rust functionalities - 3 points.
  3. Programming challenges - 3 points.
  4. Size of project - 1 point.
  5. Quality of code - 1 point.

  To score high on `Usability`, there shouldn't be any issues with running the project (be sure that all commands are running and that we will precisely know when and what to execute to test the project, if needed also try to give some reproduceable environment, e.g. docker). The `Usage of Rust functionalities` forces you to use harder features/bigger Rust libraries (async, wasm, gui, testing, macros, etc). The `Programming challenges` just means whether it was straightforward code that is simple to write, or something that required effort/thought/debugging. The `Size of project` just means how much features there are in the project, and how much meaningful code there is. If we don't have much comments about design of the code and there isn't any issues with readability, then `Quality of code` will be high.

- 1/3 of the grade is based on the final exam. The exam will be done in the lab, with no access to any notes or the Internet. The scope of the exam will be covered by the lessons, including obligatory reading. The difficulty of exam questions is going to vary, with easier questions to verify basic knowledge and harder questions to verify deeper insight.

### Additional requirements to pass

- At least half (12) of the points gained from the exam.
- At least half (12) of the points gained from the small tasks.

### Passing threshold

- We guarantee that gaining >=60% of all points + satisfying the above requirements will be enough to pass.
- We might lower the passing threshold if we believe it's necessary.

In specific, individual cases, it is possible to raise the grade by doing additional work, but it has to be agreed by us beforehand and it should be not easier than going through the standard path.
The additional work might be:

- Making a presentation about some advanced topic (const generics, futures, macros, etc.) or about architecture of a selected Rust open-source library.
- Contributing to a selected Rust open-source library.
- Contributing to this course's materials.

## Big project deadlines

The deadlines for the big project (not to be confused with separate deadlines for the small projects) are set to the start of your class.
If you have a class on Wednesday at 12:15 and the project deadline is 2026-11-04 - 2026-11-05, then your deadline is 2026-11-05, 12:15.

1. 2026-11-04 - 2026-11-05: Project ideas should be presented to us for further refining. If you wish to pair up with someone, tell us before the deadline.
2. 2026-11-12 - 2026-11-13: Final project ideas should be accepted by now.
3. 2026-12-16 - 2026-12-17: Deadline for submitting the project.
4. 2027-01-13 - 2027-01-14: Deadline for **optional** submission of the corrected version of the project, if settled so with the lab teacher after the first submission had significant flaws.

## Small tasks grading

### Deadlines

The deadlines are strict. After a deadline passes, we will publish our requirements, so it would be unfair for someone to write their solution knowing them.

### No LLMs

- The code must be written on your own, **without any AI/LLM code generation**.
- The task must **not** be fed to LLMs for any help.
- In general, **you must not use LLMs for any part of solving those tasks**.

### Gaining points

- The tests are publicly available.
- If your code passes the tests AND the linter (`cargo fmt` + `cargo clippy`), you get 1 point.
- Another 1 point you can get based on the manual peer review of your code. For every flaw, some part of a point will be subtracted, up to losing the whole point.
- The last third point you can get based on having done a peer review of someone else's code.
- **If you won't post a review for your reviewee, you will get 0 points for your solution.**

### The review process

- Once the deadline for submitting a task has passed, we will randomly assign reviewers to reviewees from the set of people who submitted the task in your group.
- We will publish our requirements, frequent mistakes and suggested scoring for each flaw.
- Within the next one week, it is your responsibility to post a review on GitHub on your reviewee's code, subtracting points for each flaw. You're encouraged to write inline comments, selecting lines of code that contain given flaw. You're welcome to spot additional flaws, also those that are not listed in our requirements.
- In case of a conflict between reviewer and reviewee, we will perform the review ourselves and decide.
- We will choose a subset of solutions randomly and verify the review correctness ourselves. For each missed or excessively spotted flaw we will subtract the equivalent from the reviewer's review point.

### Guidelines

- By default, as long as the solution makes sense and passes all the tests, you get the maximum number of points, which then gets reduced based on the number of review comments we gave. There is no fixed convention, but usually tiny review comments give just a tiny penalty (multiples of 0.1 point).
- The solutions shouldn't have any unnecessary files or code. As long as they satisfy the task requirements, they are fine.
- As memory management is a big part of the Rust language, to learn how to do it optimally, the solutions should minimize the used memory (while still doing it safely). When possible, prefer to avoid allocations.
- Algorithmic complexity and the constant behind the solutions matters too. For example, prefer using single operations on `HashMap` instead of multiple operations, and prefer to use data structures that minimize lookup time.
- As the functional programming approach is also a big part of the Rust language, prefer to use it over imperative programming. Especially whenever you do operations on containers. Finding the cleanest (and usually safest) approach is left to you.
- Error handling should be done consistently, as per lessons' guidelines. Be sure that you understand when to use `unwrap`, `expect`, `?`, `Result`. Design the code to minimize the number of assumptions and `unwrap`s.
- During lessons we do not dive into the documentation to learn all available functions from the standard library. It is your job to explore the documentation and find functions that solve whatever specific issue you have. We might give review comments whenever there's some cleaner approach using the standard library.
- We can also give review comments related to general code quality, not strictly related to the Rust language. In particular, please avoid copy-pasting, writing non-meaningful comments, overcomplicating the approach, using unoptimal data types. The approach and readability of the code also matters.
- Lastly, of course the solutions should adhere to guidelines that are in the lessons (both in the provided text and spoken during classes).
