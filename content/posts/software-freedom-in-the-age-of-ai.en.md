---
title: What Software Freedom Means in the Age of AI
description: As models, data, and automated workflows become new dependencies, freedom means more than source access. It also requires understanding, portability, reversibility, and an exit.
date: 2026-08-28
taxonomies:
  tags: [free-software, open-source, llm, digital-autonomy]
---

My articles live in local Markdown files. Git records their history, and the site can be built as static files. From the source format to the published result, the work appears to be under my control.

The writing process, however, no longer happens entirely on my machine. I use LLMs to analyze old drafts, organize structures, inspect code, and translate articles. Hosted services build and deploy the site. Link checks reach websites I do not control. If a model API disappeared tomorrow, raised its prices, disabled my account, or adopted a policy I could not accept, how much of this workflow would remain?

That question has made me reconsider what software freedom means.

The traditional questions remain essential: Can I run the program? Can I study and modify its source? Can I share my changes? In an AI system, however, source code is only one layer. An application can publish all of its client code while locking its models, data, memory, permissions, and identity inside a platform that cannot be migrated.

If I can inspect the source but cannot take my content with me, modify the client but cannot replace the underlying model, or let an agent act without knowing what it read and changed, then “open source” has not answered the entire question of freedom.

## How I first understood freedom

I became involved with open source in 2019. What attracted me was not simply the ability to use software without paying for it. It was a different relationship to participation.

In an open-source community, someone maintains a terminal tool, polishes a configuration, fixes a bug that is not urgent, or writes the missing paragraph in the documentation. The work does not always come from a KPI, a course requirement, or a commercial assignment. People contribute because they care about the project or want the next person to have a slightly easier path.

That was my first concrete experience of voluntary participation. Freedom did not mean an absence of responsibility. Because nobody had compelled the contribution, the decision to submit code, answer a question, and maintain the result carried a different kind of responsibility.

I later paused my site and removed its public material. When I eventually built a personal site again, one idea had become clear: my content should not be permanently bound to a platform. Domains, frameworks, and deployment methods could change. The original writing should remain intact, portable, and publishable elsewhere.

At that point, I understood freedom mainly as ownership of code and content. AI added another question: when software begins to read, judge, and act on my behalf, what exactly have I handed over?

## Control has moved to new places

The dependencies of traditional software are usually visible: source code, compiler, runtime libraries, operating system, and hardware. AI workflows add several new points of control.

The first is data. My articles may remain local while conversation histories, vector indexes, model memory, and feedback records exist only inside a provider account. An Export button is not automatically portability. Whether the export is complete, whether its format is open, and whether another tool can continue from it are separate questions.

The second is the model. Two APIs may accept similar message formats while behaving very differently around tool calls, context windows, structured output, and safety policies. If an application scatters those assumptions throughout its business logic, switching models is no longer a configuration change. It becomes a rewrite.

The third is permission. An ordinary chat produces text. An agent may read files, execute commands, modify a repository, control a browser, or send information to an external system. As capability grows, the system must become clearer about what it is allowed to do and what it has just done. Otherwise, automation does not increase my control. It merely moves control from one opaque interface to another.

The final point is identity. Many services attach models, data, billing, and permissions to one account. When that account becomes unavailable, the loss may include not only a tool but years of accumulated context and the entrance to an entire workflow.

These dependencies show why an open-source client does not necessarily create a free system. The reverse is also true: using a proprietary service does not automatically eliminate every form of freedom. The practical questions are whether dependencies are visible, data is portable, critical capabilities are replaceable, and work can continue after leaving.

## Freedom does not require rebuilding everything

I used to find it easy to equate autonomy with implementing things myself. If I disliked an existing tool, I could make another one. If a dependency felt too complicated, I could replace it with code under my control.

mdout did not follow that instinct to its conclusion.

It uses Rust for content validation, version diagnostics, external-link checks, and build orchestration. Markdown parsing, templates, syntax highlighting, RSS, search indexes, and the development server remain the responsibility of Zola. Reimplementing all of that would give me more code of my own and more problems that only I could maintain.

Depending on Zola did not remove my freedom because the relationship is legible. The content remains Markdown. Configuration and templates remain in the repository. The build can run locally, and its output consists of ordinary static files. If I replace Zola one day, I will not first have to rescue the articles from a closed database.

That experience changed my definition:

> Freedom does not mean having zero dependencies. It means that dependencies are visible, their boundaries are stable, and the cost of replacement can be understood and borne.

Hosted services can also be reasonable choices. They save hardware, operations, and time, allowing an individual to use capabilities that would otherwise be unaffordable. The problem is not that a system belongs to someone else. The problem begins when convenience quietly removes every alternative.

## Five freedoms for an AI workflow

When working with models and agents, I now look for five concrete capabilities.

### Understanding

I should know what context the system sends to a model, which tools it exposes, and which directories and external services it permits the model to access. A model's internals may never be fully transparent, but its inputs, permissions, and operational boundaries should not be secret.

### Taking my work with me

Articles, conversations, configuration, prompts, and necessary history should be exportable in formats that people and ordinary tools can read. Screenshots and platform-specific backups are not enough to establish meaningful ownership of data.

### Replacing a component

A workflow should not treat the special behavior of one model as permanent infrastructure. Interfaces can be adapted, models can be changed, and core data structures should remain as independent from a provider as practical. Replacement does not have to be free, but it should not cost as much as rebuilding the whole system.

### Reversing an action

Generating a paragraph and deleting a directory are not equivalent operations. Automated changes need visible diffs. Important actions need confirmation. Failures need a path back. The more capable a system becomes at acting, the more important reversibility becomes.

### Leaving

Exit is the final test of the other four capabilities. If I reject a service's pricing, policies, or direction, can I still write, build, search, and publish? If the answer is no, what I had was temporary permission to use a system, not a durable choice.

A choice that cannot be left is difficult to call a choice.

## Open models are not an automatic answer

Open weights and open code expand the available choices, and that matters. The label “open” alone still cannot establish whether an AI system is free.

A downloadable model may require hardware an individual cannot afford. Its weights may be available while the training data and process remain impossible to examine. The model may run locally while its memory, tool platform, and identity system remain closed. An application may support another model yet provide no way to export years of user data.

The problem cannot be reduced to a simple opposition:

```text
open model = freedom
proprietary model = no freedom
```

Licenses determine an important set of rights, but system freedom also depends on portability, observability, permission design, and reversibility. An open agent that runs locally while silently modifying files is not more trustworthy than a hosted tool with explicit permissions and an auditable record of every action.

Freedom is not a product label. It is a collection of system properties that can be tested in practice.

## Preserving the ability to say no and continue

I will continue to use model APIs, GitHub, Zola, and other external services. Refusing every dependency would not make me freer. It would spend my time rebuilding work that others already do well.

But whenever I accept convenience, I want to understand the exchange. Where does the data go? Can I inspect the actions? Can I reverse the result? Does an alternative exist? If a service disappears, I may temporarily lose efficiency, but I should not lose my content and my ability to continue at the same time.

That is what software freedom means to me now.

It does not require one person to do everything, or every model to run on a machine they own. It requires that people facing increasingly powerful automation can still see their dependencies, define boundaries, take their work with them, and remain responsible for the final decision.

Freedom is not the absence of dependence. It is refusing to let any one dependency decide whether the work may continue.

AI can generate my next paragraph, but accepting it, deciding where it belongs, and choosing when to stop using the system should remain choices I can make.
