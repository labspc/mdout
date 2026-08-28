---
title: How Constraints Create Freedom
description: From type systems and RAII to product boundaries and AI agents, good constraints move repeated decisions into explicit, testable rules and free attention for choices that still matter.
date: 2026-08-28
taxonomies:
  tags: [engineering, type-systems, llm, software-design]
---

When I was building Zettelk, I tended to equate freedom with having more options.

The site could use SPA navigation, hover previews, and a content graph. Articles could participate in image systems, tags, and backlinks. MCP and LLM features could be attached to the knowledge base. Each new capability seemed to move the project closer to an ideal digital garden.

The options did not automatically become freedom. They first became a continuous series of decisions. Which controls should remain on mobile? How should the graph be laid out? How should tags be normalized? Who owns image management? When should a component load? Is a feature that may be useful later worth maintaining now?

Every choice was reasonable. Together, they occupied the time and attention that the system was supposed to preserve for writing.

mdout later removed many of those capabilities and reduced the product to a few commands and a plain pipeline: Markdown goes in, content is checked, and static HTML comes out. It could do less, yet I found it easier to begin writing and easier to believe that the site could keep working over time.

That experience forced me to revise an intuition. Freedom does not always come from adding possibilities. Sometimes action is prevented by having to reconsider every possibility at every step.

A good constraint moves a repeated decision out of the moment and into a boundary that is explicit, testable, and revisable. It removes some options while releasing attention for decisions that matter more.

## Options can become debt

Freedom is easy to imagine as “I can choose at any time.” A tool with ten configuration modes appears freer than one with two. A language that permits more implicit behavior appears more flexible than one with strict rules. An agent that can access every file and every tool appears more powerful than one with limited permissions.

Every option left undecided, however, will eventually ask for a decision.

Configurations have to be combined. Combinations have to be tested. Compatibility has to be maintained, and failure paths have to be handled. A possibility preserved today can become decision debt tomorrow. The debt may not appear in a line count, but it appears in the hesitation before every change: will this break a mode that nobody remembers?

The number of available options is therefore not the same as freedom to act.

With a clear boundary, I can move quickly because I know what cannot happen. With an ambiguous boundary, I may have more theoretical choices, but I must first prove that each step will not disturb an unknown dependency.

## Types give decisions a testable shape

When I first studied type systems, I also experienced types as restrictions. They told me which values could not be combined, which functions could not be called, and which programs would never get a chance to run. A dynamic language seemed freer because many decisions could be deferred until runtime.

I gradually came to understand that the deeper value of a type is not prohibition but expression.

Suppose an interface represents a loading result with only `string | null`. The `null` may mean that loading has not started, is in progress, returned no data, or failed. The type gives the implementation considerable freedom while leaving every caller with the cost of interpretation.

Now make the states explicit:

```text
NotLoaded
Loading
Loaded(Data)
Failed(Error)
```

There are more types and fewer arbitrary representations, but the meaning of each state is clear. The interface no longer has to guess what `null` means, and a compiler can identify a branch that has not been handled.

This constraint does not make the product decision for me. It requires the decision to have a shape that can be inspected.

A type system is therefore a discussion held in advance. Which states can exist? Who handles failure? Which combinations have no meaning? Once the answers enter the type, every caller no longer has to invent them again.

## Ownership gives responsibility a location

C gives a programmer substantial control over resources. The program can decide when allocated memory is released, a file is closed, or a lock is returned. That control can be indispensable, but it also means that every exit path must correctly assume responsibility for cleanup.

RAII removes part of that freedom. Resource acquisition is tied to object lifetime, and cleanup follows scope through destruction. Once a resource has been wrapped correctly, a programmer no longer has to repeat the same release logic at every return point.

What disappears is the option to decide at every moment whether cleanup should happen. What appears is the ability to predict resource behavior after exceptions, early returns, and refactoring.

This reveals another function of constraints:

> A constraint does not eliminate responsibility. It gives responsibility an explicit owner.

When ownership is ambiguous, every participant may assume that somebody else will handle the resource. When ownership is clear, local pieces can be composed with confidence. Freedom no longer means that anyone can do anything. It means that I know what belongs to me and what I no longer need to worry about.

## A product boundary is also a type

mdout does not provide a CMS, an online editor, a cover-image system, a complex publication state machine, or automatic translation. It also does not reimplement Markdown parsing, templates, search indexes, and a development server merely to eliminate an external dependency.

Those limits do not imply that the omitted features have no value. They answer a more specific question: which problems belong to this product?

The contract of mdout can be compressed into a small pipeline:

```text
Markdown
    ↓
content checks and a static build
    ↓
HTML
```

Once the boundary is clear, many decisions disappear. An article does not move through database states. A production page does not require an online runtime. Build output can be inspected directly, and daily work can center on a small command set.

A product boundary resembles its type. When the boundary is vague, users must guess which inputs it accepts, which state it maintains, and which responsibilities it assumes. When the boundary is precise, the product is easier to compose with other tools and easier to replace.

Removing features is not the objective. The objective is to avoid making promises the system cannot keep over time.

## An agent boundary cannot live only in a prompt

LLMs make the question of constraints more immediate.

When a model only generates text, an incorrect answer usually remains inside a conversation. An agent with tools can read files, execute commands, modify a repository, control a browser, and access external services. Once capability moves from suggestion to action, a sentence that says “please be careful” is not a sufficient boundary.

Constraints can exist at several levels:

```text
natural-language instructions
    ↓
structured output and schemas
    ↓
tool and directory allowlists
    ↓
confirmation before important actions
    ↓
diffs, tests, and acceptance criteria
```

“Do not modify unrelated files” is useful guidance. Allowing writes only inside a workspace is a stronger system boundary. Asking an agent to check its work expresses an expectation; displaying a diff and running tests provide evidence that can be evaluated. Telling it not to send sensitive information is weaker than denying access to data it does not need.

These mechanisms are not necessary because models are uniquely untrustworthy. Any actor capable of independent action needs a boundary of responsibility. Models simply make it easy to produce a plausible next step quickly enough that permissions and consequences can disappear from view.

The stronger the capability, the less its boundary can exist only in a prompt.

## Not every constraint creates freedom

I once built an AI tagging tool with a rule that every article must have exactly five tags. The rule was clear, consistent, automatically testable, and easy for a model to follow.

It answered “How can tags be made orderly?” It did not establish that five tags improved writing or reading. Meeting the fixed number could require marginal tags. Maintaining the vocabulary introduced aliases, categories, and automatic expansion. A constraint can be executed perfectly while optimizing the wrong objective.

Clarity and strictness are therefore insufficient. A constraint that creates freedom usually has additional properties:

- It serves a real and explicit problem.
- The people subject to it can understand why it exists.
- It is enforced close to the risk it controls.
- Its strength is proportional to the possible consequence.
- New evidence can cause it to be revised.
- Necessary exceptions have explicit, auditable paths.

If a rule cannot be explained, questioned, changed, or left, and asks only for obedience, it is not a foundation for freedom. It is control itself.

A good constraint removes unimportant choices while preserving the ability to change the constraint.

## Freedom is not deciding everything all the time

Type systems reduce illegal states. RAII reduces resources without an owner. Product boundaries reduce an endlessly expanding problem space. Agent permissions reduce the range of irreversible actions. Their shared quality is not an abundance of rules. It is that decisions have moved to a place where they can be understood and checked.

I no longer want a system without constraints. Such a system may look open while distributing its complexity to every use, every caller, and every future maintainer.

I want its boundaries to be visible. I want to know where they are, why they exist, what they protect, and what evidence should cause them to change.

Freedom is not facing every possibility at every moment. It is being able to act within trustworthy boundaries without repeatedly wondering whether the ground will disappear.

The freedom created by a constraint is not freedom from all decisions. It is the freedom to reserve limited judgment for the decisions that still have to be mine.
