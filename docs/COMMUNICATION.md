# Communication

This document is how to write for this project: source comments, commit messages, reviews, documentation, design records, discussions, plans, tickets, reports, and chat replies. Each entry states the rule, the reasoning where it decides something, and the anti-pattern a reviewer flags on.

What the text is decides which rules apply. A **description of the system** (a comment, a doc, a guide, a skill, an instruction, a packet) is read by someone who needs to understand the system as it stands, so it describes the present state, and carries temporal framing only where that makes the current behavior clearer. A **working record** (a discussion, a plan, a ticket, a review, a handoff, a ledger, a report, a commit message) is dated, and its content is what was decided, when, and how that changed; a commit message describes the change and the reason that matters for it. A **conversation** (chat, a discussion turn) has a reader in front of it and moves in time like any conversation does. The rules under *Everywhere* hold for all three; the rest of the document says what changes between them.

## Everywhere

### Say it plainly

Plain language in complete sentences, in terms any developer follows. State the claim up front and stop: no alternative invented to knock down ("X, not Y", "it's not just X, it's Y", "rather X than Y") unless it corrects an ambiguity the reader actually holds; no fact withheld for a closing reveal; no hedging on something known; no performative praise or enthusiasm aimed at the reader or the work. Plain acknowledgment such as "good job", "well done" or "thank you" is ordinary. A real contrast, the thing people actually get wrong, is welcome; the manufactured one goes. A thing is described by what it does. A sentence whose content is what something lacks is replaced by the statement of what does it, and words that carry emphasis rather than meaning — "just", a "never" that stresses rather than states a rule — are cut.

**Why**: a plain statement is shorter and checks directly against the code or system it describes. A statement of what a thing does is checkable; a statement of what it lacks leaves the reader to work out what does it.

**Anti-pattern**: "it's not X, it's Y" or "rather X than Y" against a Y nobody proposed; "perhaps", "arguably", "it seems" on a fact the writer verified; "seamless", "robust", "comprehensive"; a sentence whose second half exists only to negate its first; "what X can't do is", "it just doesn't".

### No justification tails, no self-reproach

A request states the ask and stops; a "would rather X than Y" tail is cut. A decision already made is stated, and the case for it is made where it was made. A correction states the corrected claim, the evidence that decides it, and what downstream depends on it, then the work continues. A short acknowledgment between collaborators, such as "my bad" or "correction accepted", is ordinary. A correction carries no account of how the error came about, no assessment of the writer, and no finding of fault placed ahead of the corrected claim.

**Why**: the reader wants the ask or the outcome; a defense of it, or an account of the writer's fault, is a second message they did not send for.

**Anti-pattern**: "so that we do not build to a guess", "I apologize for the confusion", "to be transparent about my reasoning", "I should have caught this", a correction that opens by establishing the writer's fault before stating what is now true, a paragraph re-arguing a settled call.

### Counts are computed

Any count, enumeration, sweep, comparison, or all/every/none/only claim is computed with a scratch script (bash or python) written on the side for that claim, never asserted from reading, memory, or a prior record. The command and its output sit beside the claim so a reviewer re-runs the receipt instead of re-deriving the fact. A receipt whose result can be empty prints the size of the set it searched in the same block ("N examined, M matched") so a false clean is distinguishable from a true one, and that set comes from a tool (a diff, a file list, a query) rather than from paths or symbol names typed into the command, so a typo fails loudly instead of returning zero. When the claim quantifies over a set, the denominator is that set, not the number of matches. Dated text (chat, a review, a discussion turn, a handoff) carries the number and its receipt. A document meant to stay current keeps the command and drops the number, because a copied count stops being true on the next change; it names where state is maintained instead of quoting its current value.

**Why**: a stale number inside a maintained-looking document is worse than no number.

**Anti-pattern**: "26 items", "the active arc is X", a progress summary or inventory count in an orientation record.

### Form

Explanation is prose in complete sentences. Bullets and tables are welcome where the text introduces a thought and then enumerates its aspects, or lays out anything a reader will scan or filter; a checklist may be terse. What goes is the bullet that says nothing: a two-word fragment standing in for a point the writer did not have, there so the section looks like it has content. A markdown paragraph is one line, because wrapping belongs to the renderer. Length follows content.

**Anti-pattern**: a list of bare headings ("Performance", "Security", "Scalability") with nothing under them; a paragraph chopped into bullets to look organized; a sentence run together with commas where it has three or four separate claims.

## Descriptions of the system: comments, docs, guides, skills, instructions, packets

### Describe the current system, not its history

Text states what the system is. Prior implementations, renamed concepts, migrations, and transitional states belong to the diff, the commit log, and the working records. A timeless negative that describes a standing property is fine ("the handler rejects an empty path"); a history-reactive one ("no longer emits", "the legacy X", "do not use the old pattern") is the failure. In an instruction the cost is sharper: warning an agent off a discarded pattern plants the pattern it never knew. Several concurrently supported schema or protocol versions are documented as concurrent behavior, each on its own terms.

Cut the history and keep the texture. A troubleshooting guide or a skill still needs the concrete detail that lets a reader recognize their situation ("the commit is in branch history, even buried under later task commits"). Keep the recognizer; drop the incident narrative it came from.

**Why**: readers understand the system from the current repository. History does not define the contract, and in agent-facing text it introduces the option it warns against.

**Anti-pattern**: "formerly", "previously", "no longer", "legacy surface", "after the migration", "still consumed by", "preserves the old behavior", or an instruction phrased as "do not use the old / previous X". Also the over-correction: a rewrite that replaces lived, recognizable detail with abstract state language and calls the result cleaner.

### Roles, in the third person

A description names roles, in the third person: the reviewer records the verdict, the operator runs the script, the developer accepts the item. It carries no "you", no "I", and no pronoun standing in for a person; a pronoun for a thing is ordinary grammar. Register survives the third person; the person is what goes.

**Why**: a description is read by whoever holds the role next. "You" assumes a particular reader, and first person assumes the author is in the room; a role holds whoever fills it.

**Anti-pattern**: "you should", "when you run it", "I handle the reviews", "she decides", "we are the first to".

### The text does not narrate itself

A document may state its scope, its authority, and where related material lives, in a sentence or two at the top. Beyond that, no sentence takes the document as its subject: nothing announces what a section is for, summarizes what follows, or reports the document's own edits. Content that was removed leaves no trace; "this section no longer covers X" is the same defect as leaving X in.

**Why**: words about the text instead of its subject are noise to every reader after the first.

**Anti-pattern**: "this section explains", "as noted above", "for clarity", "removed the stale paragraph", a scope statement that runs to a paragraph.

### Comments are minimal and self-contained

A comment states a non-obvious fact about the code as it stands: the invariant it maintains, the contract its callers rely on, the reason a construct is shaped the way it is. An item takes a `///` doc comment where its contract is not evident from the signature: what it guarantees, what callers must hold before calling, which errors it returns, and when it panics. A `//` comment carries an implementation note beside the code it explains, including `// reason:` on a lint suppression and `// SAFETY:` on `unsafe` code. It points at nothing outside the source tree: no ticket id, roadmap tag, commit hash, dependency version or date, `file:line` pointer into another repository or document, or path to a design document; when it explains why a rule exists it states the rule. A comment states its rule in the code's own terms, not through a sample case; a placeholder or fixture name in a comment ties it to one instance and to the report that produced it. A schema or format identifier the code operates on (`<format>/v<n>`, a wire tag, an example value the code itself parses or emits) is data the code uses and is fine. If deleting the comment loses nothing the code already says, delete it.

**Why**: the source stays authoritative on its own; a reader verifies the comment against the code beside it. A `///` doc comment binds to the item it documents and renders in `cargo doc`.

**Anti-pattern**: `// per <ticket>`, `// see <design-doc>`, `// now uses X`, a `///` doc comment that restates the signature, a `//` comment standing above an item in place of a `///` doc comment or an implementation note beside the code, a comment that lectures, warns about the future, or explains how the code got here.

## Working records: discussions, plans, tickets, reviews, handoffs, ledgers, reports

### History is the content, ceremony is not

A working record is dated and says what was decided, what changed since, and what is still open; before-and-after, comparison, and prior positions are its ordinary material. It carries the metadata a reader needs to act on it: a status (proposed, ready for planning, in delivery), the baseline it was verified against, and who holds the decision. What it does not carry is process about itself: no approver columns, sign-off rows, authorization ceremony, or procedure for its own upkeep. Plans, tickets, backlogs, and ledgers exist so the person doing the work can see what is on their plate. Structure earns its place by making a record easier to read or filter: a numbered item list and a controlled vocabulary for a column are help; a justification block and a rule for adding a row are ceremony.

**Why**: a record that enforces process on its owner stops being read.

**Anti-pattern**: "who authorized this change", an "approved by" column, a procedure section in a ledger.

### A review ends in a verdict

A review states each finding in directive language with the file, the line, the reason, and the change, separates required changes (defects already visible, closed before shipping) from open risks (concerns that only surface at runtime), and ends in a decisive verdict; a concern already visible is a required change. It is revised while the reviewer and the author work the findings, and frozen once delivered to whoever must respond.

**Why**: the author acts on what the review decides, and a finding without a change, or a review without a verdict, costs another round.

**Anti-pattern**: a finding hedged into an open risk when the defect is already visible; a review that closes without a verdict.

## Conversation: chat, discussion turns

### Talk like a colleague

Chat and discussion turns are direct and first person. Questions are questions; disagreement is stated plainly and is welcome; an opinion is given as an opinion. History, comparison, what was tried, what will change next, and why a prior call is being revisited are all ordinary things to say here; only the dramatic framing under *Say it plainly* goes. Plain does not mean clipped. A reader who asks for an example, a sketch, or another anchor for an idea is owed one, and useful context is never cut in the name of brevity.

**Why**: the reader is in the room, and a turn that addresses nobody wastes the exchange.

**Anti-pattern**: a reply so terse it reads as a label; a turn written in document register that addresses nobody and answers nothing; agreement with the reader in place of a reasoned position.

### Report what you did, not the request

A reply carries the outcome, what changed, and what was found. It does not restate the request, walk through every step taken, or close with a recap of what precedes it. When a turn is asked to describe changes made, that description is the content, and it is given plainly.

**Anti-pattern**: "you asked me to X, so I did X"; a numbered log of tool calls; a closing summary paragraph.

### Answer the question, and propose rather than punt

A direct question gets a direct answer in its own terms ("yes, one migration: add column X to table Y"). A request for next steps gets a short, tight list of specific things to do next. An open decision gets a call and an internally consistent proposal. "This needs a decision" without one, "defer until we know more" when the answer is already knowable, and "good enough for now" on a system going to production are all punts.

**Why**: the work moves on what is decided, and a non-answer costs another round.

**Anti-pattern**: "X itself does not require a schema change, but Y would require extending the stored configuration" in reply to "do I need a migration or not"; "that last sketch needs one code check before implementation" as a next step.

## Outbound to the customer: question sheets, status notes

### State the ask, then stop

A question to the customer carries the question, the answer options, and nothing else. A status note carries the current outcome, what the customer sees changed, and any action that is theirs to take, with no account of the development behind it. History and the recap of who said what are cut, except for a genuine contradiction the customer has to settle, which is stated as one sentence per position. "We" is cut with it: ask about their practice instead of putting our problem in front of their answer. A recommendation is one sentence and follows the question. Two questions with one answer are one question.

**Why**: the customer wants the system built and has no stake in development details or process. A question framed around their practice is one they can answer; a question framed around our problem asks them to care about something that is not theirs.

**Anti-pattern**: a paragraph explaining why our preferred answer is the better one; "we cannot build both until you tell us"; twenty questions when eight would do.
