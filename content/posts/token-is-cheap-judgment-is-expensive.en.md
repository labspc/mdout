---
title: "Tokens Are Cheap. Judgment Is Expensive."
description: When an LLM can generate knowledge, code, and prose on demand, technical writing remains valuable when it preserves questions, evidence, tradeoffs, and changed minds.
date: 2026-08-28
taxonomies:
  tags: [writing, llm, engineering, thinking]
---

I recently reread twenty Markdown files from my old blog.

Across more than eight thousand lines, I found C, C++, 8086 assembly, SICP, TAPL, Java frameworks, Windows development, LoongArch, binary translation, and a tool that used a language model to manage tags. They record subjects I took seriously at different stages of my life. Yet rereading them forced me to admit something: with a clear prompt, an LLM could now produce a more complete, more orderly, and sometimes more readable version of a substantial part of that material in seconds.

A register table is no longer scarce. Neither is an explanation of smart pointers or a timeline from J2EE to Spring Cloud. Titles, summaries, code samples, best practices, and conclusions can all be produced on demand, token by token.

Some parts of those files had not lost their value, though.

I still remembered shutting down my site, reducing my contact with the outside world, and writing roughly one hundred thousand Chinese characters during a quiet period with no publication plan. I remembered almost revering SICP on my first encounter, then returning a year later and finally being willing to say that a classic could be powerful and limited at the same time. I also remembered a memory-pool example that ran normally on my machine until an online tool reported `Invalid write of size 8`.

Those moments were less comprehensive than the tutorials around them, but they contained something the tutorials did not: why I arrived at a question, what I believed at the time, and how evidence forced that belief to change.

That distinction made me reconsider what technical writing should preserve in the age of LLMs.

## Answers used to be expensive

Producing a technical article used to be a substantial piece of work in itself.

Explaining a concept meant consulting books, searching documentation, preparing an environment, running examples, and arranging scattered material into an order another person could follow. Even an article that mainly introduced syntax or catalogued tools represented real effort. A complete and lucid answer was often reasonable evidence that its author had done the work.

Many of my old articles were written to that standard. An article about the 8086 moved from registers to segmented memory. One about modern C++ moved from RAII to the three standard smart pointers. A survey of Windows development mapped Win32, WinRT, WinUI, and their toolchains. These articles were not worthless. Writing them helped me establish basic concepts and left a record of how I had approached a subject.

The conditions of knowledge production have since changed.

LLMs have made the marginal cost of obtaining a plausible answer very low. For a common question, a model can usually provide a definition, an example, a comparison, a list of cautions, and suggestions for further reading. What once took an afternoon to organize can now become a first draft in a few exchanges.

This does not make answers worthless, and it certainly does not make generated answers correct by default. The change is narrower and more important: **completeness, fluency, and structure are no longer sufficient evidence of understanding.**

Fluency used to signal ability. It can now be a by-product of compute.

## What remains expensive when generation becomes cheap

Tokens have a price, but generation has become cheap relative to human time. The expensive work has moved to the parts before and after generation.

### Choosing the problem

An LLM can generate many solutions to a problem. It cannot decide for me which problem deserves the next year of my attention.

While maintaining a digital garden, I once treated inconsistent tags as an important problem. I built a controlled vocabulary, alias mapping, AI content analysis, and a mechanism that could extend the tag set. Each component made sense as a piece of system design. I later realized that tag quality was not what prevented me from writing. I was spending too much time managing the system that was supposed to support the work.

"How can I make the tags more intelligent?" is a solvable question. "Is this problem worth solving?" requires a different kind of judgment.

Models are effective at finding paths after an objective has been supplied. They are equally capable of faithfully optimizing an objective that does not matter. Choosing the problem therefore becomes scarcer than generating a solution.

### Verifying the answer

While studying memory pools, I wrote a linked-list implementation. It compiled and ran on my machine, but an online visualization tool reported `Invalid write of size 8` during deallocation.

If "the program runs" had been my acceptance criterion, the implementation was finished. The error forced me to ask harder questions. Did the memory being treated as a list node satisfy the necessary layout and alignment requirements? Did the absence of a crash demonstrate correctness, or had undefined behavior simply remained invisible in one environment?

The lasting lesson was not a particular way to implement a memory pool. It was a principle: **a running result is evidence, but it is not all the evidence.**

An LLM can explain the error immediately and generate an implementation that looks safer. Without a basic model of memory layout, object lifetime, and undefined behavior, however, I cannot tell whether it has fixed the problem or merely buried it more deeply.

Verification is not a check mark placed after an answer. It requires knowing what to observe, what result would disprove the current hypothesis, and which environments or conditions remain uncovered. Those are acts of judgment.

### Owning the result

While working on mdout recently, I found that its external-link page kept saying that no check had been run. At first glance, the problem appeared to be missing data. Following the code revealed that the link command, JSON report, and page template already existed. The missing part was in the deployment workflow: nothing ran the link check before the site was built.

Adding one command to a workflow was easy. The real decisions were where that command belonged, whether a temporary failure on someone else's website should block ordinary code checks, whether a cache helped inside an ephemeral CI runner, and how GitHub's branch rules constrained scheduled workflows.

Generating YAML is cheap. Deciding which risks a publishing system should accept, and taking responsibility for that decision, remains expensive.

Once code enters a real system, it is no longer merely text. It changes files, consumes resources, blocks deployments, and affects other people. A model can offer candidates, but someone still has to define acceptance and own the consequences.

## Mental models have not been devalued

The first time I read SICP, I was overwhelmed by the power of abstraction. I treated it almost as a final answer to programming. When I returned after another year of project work, I was able to revise "abstraction is powerful" into a conditional judgment: a good abstraction controls complexity; a premature or misplaced abstraction creates more of it.

The difference between those readings was not the number of chapters I could recall. I finally had experience against which the concepts could be tested.

That is why C, assembly, type systems, and classic textbooks remain worth studying. The purpose is no longer to compete with a model over who remembers more syntax. It is to build a mental model that can explain results. A model can provide code directly, but when that code fails, performs unexpectedly, crosses a new boundary, or conflicts with another plausible answer, a person still needs to know where doubt should begin.

The value of knowledge is moving from the ability to repeat it toward the ability to judge with it.

## How I want to write now

After rereading the archive, I do not plan to renovate every old article. Much of the encyclopedic material can remain where it is. Expanding it again would only add more prose of a kind that is no longer scarce.

When I write a technical article now, I want it to answer at least a few questions:

1. What real problem did I encounter?
2. Why did my initial judgment seem reasonable?
3. What code, data, or observation supported it?
4. What evidence changed my mind?
5. Within which boundaries does the current conclusion hold?

An LLM can help find omissions, organize a structure, inspect code, and translate prose. It should not invent experience on my behalf, and its fluency should not be allowed to conceal weak evidence. It is useful for extending expression, not for replacing the formation of a position.

An article no longer needs to pretend to be complete. Instead of writing an "ultimate guide" that covers every concept, I would rather make the formation of one concrete judgment legible. The former can be replaced by the next generation. The latter records how a person met uncertainty.

## Writing is not a competition with a model

During that period when I deliberately reduced my contact with the outside world, I wrote roughly one hundred thousand Chinese characters. They had no titles, intended audience, or publication plan. Their purpose was not to prove how much I knew. Writing turned indistinct feelings into sentences I could inspect and gradually gave order to confusion.

LLMs have not removed that need. Precisely because prose can be generated in enormous quantities, I have a stronger need to distinguish sentences that merely sound right from judgments that are mine, and conclusions that have met resistance from reality.

Saying that tokens are cheap does not mean models are free, or that human expression is valuable by nature. It is a reminder that producing more content is no longer the difficult part. The difficult part is knowing what deserves to remain.

So I will continue to write.

Not to produce sentences a model cannot produce, but to preserve how a judgment formed, how evidence revised it, and why I am willing to be responsible for it.
