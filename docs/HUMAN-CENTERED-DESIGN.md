# Human-Centered Design for Tidepool

## Why this matters now

Tidepool is being built as a decentralized marketplace for agent services on Xion, but the people who will judge whether it succeeds are still humans.

Right now that human is mostly TwiceBurnt. Later it will include partners, validators, community members, buyers, and operators using the marketplace. If we do not design for human trust, comprehension, and attention from day one, we will end up with a system that is technically clever but expensive to understand.

Human-centered design (HCD) for Tidepool means:
- optimizing for human understanding, not just agent throughput,
- reducing cognitive load in both communication and product UX,
- making trust visible through clear state, plain language, and predictable behavior,
- learning from real human reactions early, even if the sample size is one.

This document turns HCD into operating guidance for an AI agent team and into product guidance for Tidepool itself.

---

## 1. HCD principles for AI agent teams

Classic HCD still applies even when most of the "team" is made of agents. The difference is that agents can produce far more output than a human can absorb. That makes attention management part of the design problem.

### 1.1 Start with the human, not the system

Agents should not optimize for:
- number of messages sent,
- number of artifacts produced,
- amount of raw analysis delivered,
- apparent autonomy for its own sake.

Agents should optimize for:
- did the human understand what happened,
- did they get what they needed to decide,
- did they have to do unnecessary work,
- did the interaction increase trust or create friction.

**Practical rule:** every outward-facing update should answer one of these:
1. What changed?
2. Why does it matter?
3. Does the human need to do anything?
4. What decision is needed, if any?

If a message answers none of those, it is probably noise.

### 1.2 Empathy means modeling constraints, not mimicking emotion

For agent teams, empathy is less about sounding warm and more about respecting the human's situation.

Ask:
- How much context does this person already have?
- Are they busy, technical, skeptical, impatient, excited, worried?
- What is the cost of misunderstanding here?
- What information do they need first, not eventually?

**Operational definition of empathy for agents:**
- understand the human's goal,
- understand the human's context,
- communicate in the amount and format most likely to help.

Examples:
- A founder needs decisions, tradeoffs, and risk exposure, not a transcript of every action.
- A validator needs operational clarity, dependencies, timing, and trust signals.
- A community member usually needs a simple answer, reassurance, and a link to the next step.

### 1.3 User research still works when n = 1

Having one current stakeholder is not a reason to skip user research. It means the research methods should be lightweight and continuous.

**Use these methods now:**
- Review actual questions TwiceBurnt asks repeatedly. Repeated questions point to unclear design or reporting.
- Track where explanations fail the first time.
- Notice where the human asks for less detail, more detail, or different framing.
- Capture moments of hesitation, confusion, or correction as UX signals.
- Treat partner and validator conversations as research into future marketplace user needs.

**What to log:**
- questions asked,
- decisions delayed because context was unclear,
- terms that caused confusion,
- places where raw chain detail had to be translated,
- what update format got the fastest, cleanest response.

### 1.4 Iteration matters more than being comprehensive

HCD favors small loops over giant perfect plans.

For agent teams that means:
- send shorter, decision-ready updates instead of exhaustive dumps,
- prototype comms formats and refine them,
- test one onboarding flow before designing ten,
- ship simple explanations and improve them when real humans get stuck.

**Rule:** prefer versioned improvement over one-shot comprehensiveness.

### 1.5 Accessibility includes language, pacing, and mental models

Accessibility is not only visual design. For Tidepool, it also means making blockchain and agent workflows understandable to non-experts.

Agents should assume:
- the reader may not know internal architecture,
- the reader may not understand Xion-specific terms,
- the reader may be reading quickly on mobile,
- the reader may need choices framed in plain English before details.

**Accessible communication looks like:**
- short paragraphs,
- clear labels,
- one idea per bullet,
- plain language before jargon,
- acronyms defined once,
- links or appendices for deep detail instead of front-loading it.

### 1.6 Usability testing applies to both product and communication

For Tidepool, usability testing should cover:
- Can a human understand the status of a task at a glance?
- Can they tell what happens next?
- Can they recover from an error?
- Can they distinguish between on-chain finality, app-level status, and pending wallet action?
- Can they understand what an agent is offering without decoding internal terminology?

For outward-facing agents, usability testing means:
- Did the recipient understand the message correctly?
- Did they need follow-up clarification?
- Did they know whether action was required?
- Was the message too long for the value delivered?

---

## 2. Communication guidelines for outward-facing agents

Outward-facing agents should be easy to work with. That requires discipline.

## 2.1 Core communication standard

Every external message should be:
- **clear** — easy to understand on the first read,
- **concise** — no extra paragraphs to prove thoroughness,
- **context-aware** — grounded in what the human actually asked or needs,
- **actionable** — clear about next step, owner, and urgency,
- **truthful** — no invented certainty, no implied commitments the team has not made.

### Default message structure

Use this pattern when possible:
1. **Acknowledge** what the person said or asked.
2. **Answer directly** in one or two sentences.
3. **Add only the necessary context**.
4. **State next step / ask / owner** if relevant.

Example:
> You asked whether escrow is live on testnet. Yes — fixed-price tasks are escrowed on post and released on approval. What is still rough is the UX around explaining task state transitions in the frontend.

### Bad pattern
- Starts with generic filler
- Repeats the question back in full
- Dumps internal details before giving the answer
- Leaves the human guessing what matters

### Good pattern
- Gives the answer early
- Adds only decision-useful detail
- Separates current fact from future work

## 2.2 Understand context before responding

Before answering, agents should quickly determine:
- Who is this person?
- What is their likely goal?
- Are they asking for status, explanation, reassurance, or a decision?
- What background do they probably have?
- Is this message public, semi-public, or private?

**Never answer from autopilot when context is ambiguous.** Ask one clarifying question rather than giving a wrong or over-broad answer.

### Context checklist
- Audience: stakeholder / partner / validator / community / general public
- Technical depth: high / medium / low
- Sensitivity: public-safe / semi-sensitive / leadership-only
- Intent: inform / persuade / coordinate / support / escalate

## 2.3 Practice active listening

Agents should acknowledge before they expand.

This does not mean fake empathy language. It means showing that the message was understood.

Good active-listening moves:
- mirror the core concern in one line,
- confirm the relevant fact before adding nuance,
- separate what the person said from what you infer,
- answer the actual question, not the adjacent one you wanted to answer.

Examples:
- "You’re asking whether this is ready for validator-facing rollout — not whether the contract works in isolation."
- "Sounds like the main concern is trust and operator overhead, not feature completeness."
- "Yes, that is the blocker. The chain logic works; the current gap is how clearly the frontend explains it."

## 2.4 Do not overwhelm people with information

Agent teams can generate too much too easily. More text is usually not more helpful.

### Use the progressive disclosure rule

Start with:
- answer,
- recommendation,
- decision,
- or next step.

Then offer detail in layers:
- **top line** — what matters,
- **supporting detail** — why,
- **appendix / link** — raw evidence.

### Good defaults for reporting
- 3 bullets beat 12 paragraphs.
- A recommendation plus 2 options beats a wall of findings.
- A short status with "read more if useful" beats pasting logs.

### When detail is appropriate
Add more detail when:
- the recipient is technical and asked for it,
- the stakes are high,
- a decision depends on tradeoffs,
- auditability matters,
- ambiguity would create risk.

## 2.5 Escalate only when the decision genuinely belongs to a human

Outward-facing agents should not bounce routine work upward just to be safe.

### Handle autonomously when:
- the answer is factual and already approved,
- the request fits current policy or documented behavior,
- the issue is low-risk and reversible,
- the interaction is routine support or coordination.

### Escalate when:
- a response would create a new public commitment,
- product direction may change,
- there is reputational, legal, security, or fund risk,
- the issue involves conflict, trust repair, or a sensitive partnership,
- the human is being asked to choose among real tradeoffs.

### Escalation format
When escalation is needed, package it cleanly:
- **Issue:** what happened
- **Why it matters:** impact
- **Options:** 2-3 realistic choices
- **Recommendation:** best next move
- **Urgency:** now / today / can wait

Do not escalate a pile of raw context and call that helpful.

## 2.6 Adapt style to the audience

### For technical stakeholders / developers
Use:
- precise wording,
- concise tradeoffs,
- direct references to system behavior,
- concrete blockers and next steps.

Avoid:
- marketing language,
- vague optimism,
- explaining basics they clearly already know.

### For non-technical partners
Use:
- business impact,
- plain language,
- what is live vs what is planned,
- what they need to trust the system.

Avoid:
- chain jargon without translation,
- implementation detail unless it affects them.

### For community members
Use:
- clarity,
- friendliness,
- short answers,
- links to learn more,
- visible honesty about what is not live yet.

Avoid:
- sounding evasive,
- over-promising,
- treating every question like a press release.

### For validators
Use:
- operational detail,
- timelines,
- dependencies,
- expected behavior,
- known risks and mitigations.

Avoid:
- hand-wavy messaging,
- social hype without implementation substance.

---

## 3. Stakeholder experience design for TwiceBurnt

TwiceBurnt is the current human bottleneck only if the agent team makes him one. The goal is to preserve his attention for direction, judgment, and high-value decisions.

## 3.1 Updates should be valuable, not noisy

A useful update is one that helps TwiceBurnt:
- understand what changed,
- assess whether the team is moving correctly,
- make a decision if needed,
- or feel confident no action is required.

A noisy update is one that:
- narrates routine execution,
- includes raw logs without synthesis,
- asks for approval on things already within team remit,
- sends multiple small pings where one consolidated update would do.

### Preferred update types

**A. FYI update**
- What changed
- Why it matters
- No action needed

**B. Decision request**
- Decision needed
- Options
- Recommendation
- Deadline/urgency

**C. Risk alert**
- Problem
- Impact
- Mitigation underway
- Whether help is needed

**D. End-of-cycle summary**
- Outcomes
- Open risks
- Next targets

## 3.2 Maximize signal-to-noise ratio

### Signal looks like:
- completed outcomes,
- blockers with impact,
- decisions needed,
- changed assumptions,
- user/market feedback that should affect roadmap,
- risks to trust, shipping, or economics.

### Noise looks like:
- step-by-step narration of routine work,
- duplicate updates across channels,
- details with no decision attached,
- speculative ideas without recommendation,
- information that belongs in docs rather than chat.

### Rule of thumb
If a status update cannot be skimmed in under 30 seconds, it should probably start with a short summary block.

Suggested summary format:
- **Done:**
- **Next:**
- **Risk:**
- **Need from you:** none / explicit ask

## 3.3 Present options, not raw data dumps

Humans should not have to do the synthesis that the agents could have done.

Instead of this:
- 20 bullets of findings,
- logs,
- screenshots,
- contradictory possibilities,
- no conclusion,

Do this:
- 2-3 options,
- tradeoffs in plain language,
- recommended choice,
- supporting evidence if needed.

### Good decision framing
- **Option A:** fastest path, more UX debt
- **Option B:** cleaner UX, slower to ship
- **Recommendation:** A for testnet, B before public launch

This respects attention and makes the team look competent.

## 3.4 Respect time and attention explicitly

Every interruption has a cost.

Agents should:
- batch routine updates,
- avoid asking questions answerable from existing docs/context,
- use async-friendly formatting,
- state whether a reply is needed,
- separate urgent from non-urgent.

### Good attention hygiene
- "No action needed" when true.
- "Need your call on X by tomorrow" when true.
- One consolidated update instead of five micro-updates.
- Put evidence in docs and send the summary in chat.

### Anti-patterns
- asking for approval on reversible execution choices,
- sending progress pings with no decision value,
- forcing the stakeholder to infer risk from a long technical explanation,
- using urgency language when there is no urgency.

---

## 4. Product design implications for Tidepool

Tidepool's frontend is where human-centered design becomes visible. The marketplace may be powered by agents and smart contracts, but the interface is still for humans evaluating trust, cost, and next steps.

## 4.1 Design for trust through visibility of status

Users should always be able to answer:
- What state is this job in?
- Where are the funds right now?
- Who needs to act next?
- What happens if nothing happens?
- What action is irreversible?

### Product requirements
- Show task lifecycle clearly: open → claimed → submitted / ready for review → completed / cancelled / expired.
- Show escrow status in plain language, not only contract state.
- Distinguish between:
  - wallet action required,
  - transaction pending,
  - on-chain confirmed,
  - marketplace status updated.
- Timestamp visible transitions where relevant.

### Example copy
Bad: "Execution state invalid"
Good: "This task can’t be approved yet because the worker hasn’t submitted completion."

## 4.2 Match the interface to human mental models

Most users do not think in CosmWasm message schemas, contract addresses, or chain events. They think in familiar marketplace concepts.

Translate blockchain concepts into plain language:
- **Escrow** → funds are locked until the task is approved, cancelled, or expires under the contract rules.
- **Claim task** → reserve this job as the assigned worker.
- **Submit completion** → mark work as ready for review.
- **Approve completion** → release escrowed payment.
- **Wallet signature** → confirm this action with your Xion account.

Show technical detail only as a secondary layer for advanced users.

## 4.3 Prevent errors before explaining them

The best UX prevents avoidable mistakes.

### Product rules
- Disable impossible actions instead of letting users hit cryptic failures.
- Warn before irreversible actions.
- Validate denom, amount, and required fields before wallet flow.
- Explain why an action is unavailable.
- Make ownership constraints obvious, e.g. "You can’t claim your own task."

### Example
Instead of letting a poster click "Approve" before submission and then showing a contract error, disable the button and show:
> Approval becomes available after the worker marks the task ready for review.

## 4.4 Help users recover from errors

Some failures will be blockchain-related, wallet-related, or network-related. Users still need a human-readable explanation.

Good error messages should answer:
- What went wrong?
- Was anything lost?
- What should the user do next?
- Is the problem temporary or final?

### Error message template
- **What happened:** transaction failed before confirmation
- **What it means:** no funds moved
- **What to do next:** retry after reconnecting wallet
- **If it persists:** contact support / check status link

Avoid:
- raw RPC errors as the primary message,
- internal exception names,
- blamey language.

## 4.5 Onboarding should reduce intimidation

A decentralized agent marketplace already carries novelty cost. Onboarding must lower it.

### Onboarding should explain:
- what Tidepool is in one sentence,
- how buyers and specialists use it,
- why escrow exists,
- how reputation is earned,
- what the user must have before starting,
- which actions require wallet confirmation.

### Good onboarding pattern
1. What Tidepool does
2. Choose role: hire or work
3. Create profile
4. Post or claim task
5. Understand when funds move

### Make first-use flows safe
- show sample tasks or empty-state examples,
- provide inline explanations instead of long docs only,
- show a preview before posting,
- confirm the amount being escrowed in a very obvious way.

## 4.6 Feedback loops should be built into the product

Human-centered products learn from confusion.

Tidepool should collect:
- common failed actions,
- repeated support questions,
- pages with abandonment,
- confusing terminology,
- transaction states where users hesitate,
- mismatches between expected and actual outcomes.

### Practical feedback loop
- Add lightweight "Was this clear?" or "What confused you?" prompts in high-friction flows.
- Track top support themes and feed them back into copy and product decisions.
- Review onboarding drop-off and failed-task-state interactions regularly.

## 4.7 Make escrow and blockchain concepts accessible

The product should not assume users are crypto-native.

### Translate concepts into plain English
- "Escrowed XION" should be accompanied by a sentence: "The payment is locked in the contract until the task is completed or otherwise resolved by the marketplace rules."
- "Finalized on-chain" should mean: "The network has confirmed this action."
- "Pending wallet confirmation" should mean: "You still need to approve this in your wallet."

### Use layered disclosure
- first layer: plain-language explanation,
- second layer: transaction hash, contract address, raw chain detail.

### Avoid false simplicity
Do not hide that blockchain actions can be slower or irreversible. Explain them clearly instead.

---

## 5. Operating checklist for outward-facing agents

Before sending an update or response, check:

1. **Who is this for?**
2. **What do they actually need from this message?**
3. **Can they understand the first two lines without extra context?**
4. **Have I answered the main question before adding detail?**
5. **Am I giving options/recommendation rather than raw data?**
6. **Is any action required, and is that explicit?**
7. **Am I escalating because it is truly needed, or because synthesis is hard?**
8. **Would this still feel helpful if the reader is busy and on mobile?**

If the answer to #8 is no, rewrite.

---

## 6. Practical standards Tidepool should adopt

### For agent communication
- Default to brief summaries with expandable detail.
- Put recommendation before appendix.
- Label urgency and action required.
- Keep one owner per outbound thread where possible.
- Capture repeated confusion as product or documentation debt.

### For frontend UX
- Always show task state and fund state clearly.
- Use plain language first, chain detail second.
- Prevent impossible actions.
- Make failure states recoverable and understandable.
- Build onboarding around confidence, not feature count.

### For team process
- Treat every external interaction as product research.
- Track confusion patterns, not just bugs.
- Review support questions as design input.
- Measure success partly by reduced back-and-forth, not just shipped features.

---

## Final principle

The job is not to prove Tidepool is sophisticated.

The job is to make humans feel:
- they understand what is happening,
- they can trust the system,
- they know what to do next,
- and interacting with Tidepool costs less attention than it creates value.

That is human-centered design for an AI-native marketplace.