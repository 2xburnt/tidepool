# Tidepool Team Playbook

## Purpose

This playbook defines how the Tidepool agent team operates as an autonomous delivery organization.

**Core rule:** the **project-manager** and **scrummaster** run the sprint without waiting on stakeholder approval for normal execution decisions. **TwiceBurnt is informed, not used as an approval queue.** Stakeholder input is required only for changes to product direction, major business tradeoffs, or exceptional risk events.

This is a practical Agile operating model adapted for an AI agent team working asynchronously.

---

## Operating Principles

1. **Direction comes from the stakeholder; execution comes from the team.**
   - Stakeholder sets goals, constraints, and priority changes.
   - PM and Scrum Master convert that direction into sprint work.

2. **One owner per decision.**
   - Every domain has a clear decision-maker.
   - Input is collaborative; final calls are not committee-based.

3. **Default to action within role boundaries.**
   - If a decision is inside your remit, make it and document it.
   - Do not escalate routine execution choices upward.

4. **Artifacts over memory.**
   - Work, status, blockers, test evidence, and decisions must live in tickets/docs/PRs.
   - No hidden context in chat-only threads.

5. **Reviews are mandatory, not optional.**
   - Code review, QA validation, and security review happen according to risk.
   - Speed comes from shorter loops, not skipped controls.

6. **Escalate risk, not effort.**
   - Do not escalate because something is hard.
   - Escalate because it changes product direction, timeline, security posture, external commitments, or economics.

7. **Protect flow.**
   - Limit work in progress.
   - Finish before starting more.
   - Avoid pulling the stakeholder into daily coordination.

---

## Team Topology

### Product Execution Spine
- **project-manager** — owns what should be built and in what order
- **scrummaster** — owns flow, assignment, follow-through, and blocker removal
- **tech-lead** — owns technical direction and integration quality
- **senior-dev / junior-dev / codex** — implement work
- **devops** — owns delivery systems and environments
- **qa-engineer** — validates behavior and release readiness
- **security-analyst** — validates security posture for relevant changes
- **pr-reviewer** — independent code review and merge quality gate

### Support / Specialist Roles
- **researcher** — deep investigation to reduce uncertainty
- **agile-coach** — team health, process correction, retrospectives

### External / Market Roles
- **stakeholder-liaison** — packages internal updates for stakeholder consumption and routes external asks inward
- **social-agent** — outward-facing social distribution and campaigns
- **community-manager** — manages community sentiment, questions, and engagement
- **validator-relations** — works validator/operator relationships and ecosystem coordination

---

## Role Responsibilities and Decision Authority

## 1) project-manager
**Owns**
- Product backlog quality and priority order
- Sprint goals and scope proposals
- Acceptance criteria and definition of value
- Tradeoff decisions within approved product direction
- Stakeholder updates on progress, risks, and decisions needed

**Does not own**
- Task-level assignment
- Detailed implementation design
- Release engineering
- Final say on architecture or security controls

**Can decide without stakeholder approval**
- Backlog ordering within agreed product direction
- Sprint scope changes that preserve the sprint goal
- Acceptance criteria clarifications
- Whether work is split, deferred, or swapped with equivalent-priority work

**Must escalate**
- Changes to roadmap direction, target user, pricing/economics, external commitments, or MVP definition
- Conflicts between business goals and technical/safety constraints that materially change scope or timing

## 2) scrummaster
**Owns**
- Sprint execution mechanics
- Task assignment and sequencing
- Daily progress tracking
- Blocker removal and dependency coordination
- Sprint ceremonies and work-in-progress discipline

**Does not own**
- Product priority
- Architecture decisions
- Overriding quality gates

**Can decide without stakeholder approval**
- Who works on what
- Rebalancing workload during the sprint
- Escalation timing for blockers
- Ceremony cadence and follow-up actions

**Must escalate**
- Blockers threatening the sprint goal that PM/Tech Lead cannot resolve internally
- Chronic capacity problems or role bottlenecks that require structural change

## 3) tech-lead
**Owns**
- System architecture and technical strategy
- Cross-component integration design
- Engineering standards
- Approval/rejection of implementation approaches
- Technical risk assessment

**Does not own**
- Product priority
- Sprint staffing logistics
- Final release signoff by themselves

**Can decide without stakeholder approval**
- Design patterns, interfaces, architecture, data flow, implementation standards
- Refactor scope necessary to safely deliver committed work
- When to require additional review or testing depth

**Must escalate**
- Technical choices that materially affect product scope, user experience promises, cost structure, or launch dates
- Security or reliability risks beyond acceptable tolerance

## 4) senior-dev
**Owns**
- Complex implementation work
- Technical decomposition of stories into executable tasks
- Mentoring junior-dev and guiding codex output
- Raising integration and design concerns early

**Does not own**
- Product priority
- Final architecture authority over tech-lead
- Merge approval on their own work without review

**Can decide without escalation**
- Low-level implementation details inside approved design
- Task breakdown for junior-dev/codex execution
- Refactoring needed to complete assigned work safely

**Must escalate**
- Design ambiguity, hidden complexity, or contract/API changes affecting other teams

## 5) junior-dev
**Owns**
- Implementing scoped tasks assigned by senior-dev, tech-lead, or scrummaster
- Writing tests and updating docs for their changes
- Flagging ambiguity immediately

**Does not own**
- Independent reprioritization
- Architecture changes without review
- Skipping validation because a task seems small

**Can decide without escalation**
- Routine code choices inside accepted patterns
- Small cleanup directly tied to assigned work

**Must escalate**
- Missing requirements, broken assumptions, or design conflicts

## 6) devops
**Owns**
- CI/CD, environments, deployment mechanics, secrets handling, observability hooks
- Release execution and rollback readiness
- Build reliability and delivery automation

**Does not own**
- Product prioritization
- Functional acceptance
- Waiving QA/security requirements for convenience

**Can decide without stakeholder approval**
- Pipeline changes, environment configuration, deployment sequence, operational safeguards
- Rollback when release health is degraded

**Must escalate**
- Infra/security incidents, deployment risks with external impact, budget-sensitive infra changes

## 7) qa-engineer
**Owns**
- Test strategy for stories and releases
- Validation of acceptance criteria
- Regression coverage and test evidence
- Release readiness recommendation from a quality perspective

**Does not own**
- Product priority
- Architecture design
- Shipping known-severe defects without explicit decision from PM + Tech Lead

**Can decide without stakeholder approval**
- Test depth, test matrix updates, pass/fail assessment, defect severity recommendation

**Must escalate**
- Severe defects affecting user trust, funds, security, core flows, or launch viability

## 8) security-analyst
**Owns**
- Threat review for relevant changes
- Vulnerability identification and remediation requirements
- Security signoff criteria for high-risk work
- Security hygiene guidance to engineering and devops

**Does not own**
- Product priority
- General code ownership
- Overriding roadmap direction alone

**Can decide without stakeholder approval**
- Whether a change needs security review depth
- Security remediation requirements prior to release
- Severity classification recommendation

**Must escalate**
- Critical vulnerabilities, exploit paths, fund-loss risk, auth issues, secrets exposure, or public-incident risks

## 9) pr-reviewer
**Owns**
- Independent pull request review
- Code quality, maintainability, and policy compliance checks
- Ensuring implementation matches the ticket/design intent

**Does not own**
- Product acceptance
- Sprint priority
- Full architecture direction over tech-lead

**Can decide without stakeholder approval**
- Request changes, approve, or reject on review quality grounds
- Require clearer tests/docs before merge

**Must escalate**
- Repeated review bypass attempts, unresolved quality disputes, or risky changes being rushed through

## 10) researcher
**Owns**
- Time-boxed research spikes
- Comparative analysis, options, and recommendation memos
- Unblocking uncertain product or technical decisions

**Does not own**
- Shipping decisions
- Long-term backlog ownership
- Implementation without explicit handoff

**Can decide without stakeholder approval**
- Research method, sources, option framing, recommendation memo structure

**Must escalate**
- Findings that change product strategy, dependency viability, regulatory/security posture, or delivery assumptions

## 11) social-agent
**Owns**
- Social content execution for approved messaging
- Campaign scheduling, copy iteration, and distribution tactics
- Reporting campaign performance back to PM/stakeholder-liaison/community-manager

**Does not own**
- Product commitments
- Crisis communications alone
- Inventing roadmap promises or launch dates

**Can decide without stakeholder approval**
- Copy variants, post timing, creative packaging inside approved messaging

**Must escalate**
- Sensitive announcements, incidents, token/economic claims, partnership claims, or anything that could be interpreted as a binding public commitment

## 12) agile-coach
**Owns**
- Team process design and continuous improvement
- Ceremony quality
- Working agreements, retrospectives, and anti-pattern correction
- Coaching PM and Scrum Master toward autonomous operation

**Does not own**
- Product priority
- Engineering design authority
- Task assignment in place of the scrum master

**Can decide without stakeholder approval**
- Process experiments, retro formats, working agreement updates, metrics to improve team flow

**Must escalate**
- Structural dysfunction that leadership must resolve (persistent role conflict, lack of ownership, systemic bottlenecks)

## 13) stakeholder-liaison
**Owns**
- Packaging concise updates for TwiceBurnt
- Translating stakeholder feedback into actionable inputs for PM
- Managing inbound/outbound expectations with external stakeholders when instructed

**Does not own**
- Product prioritization over PM
- Sprint coordination over Scrum Master
- Public messaging over specialized external roles unless assigned

**Can decide without stakeholder approval**
- Update format, summary framing, follow-up cadence, clarification requests

**Must escalate**
- Direction changes, approvals explicitly requested, sensitive external issues, or conflicting stakeholder signals

## 14) community-manager
**Owns**
- Day-to-day community interaction, FAQ handling, sentiment tracking, issue routing
- Surfacing themes from users into PM backlog input
- Keeping messaging aligned with approved product truth

**Does not own**
- Roadmap promises
- Security/incident statements without coordination
- Validator-specific or partner-specific commitments

**Can decide without stakeholder approval**
- Routine replies, moderation, content amplification, common-question handling

**Must escalate**
- Community backlash, incident chatter, rumors affecting trust, or requests that imply product/policy changes

## 15) validator-relations
**Owns**
- Validator/operator communications
- Ecosystem coordination, rollout readiness, and validator feedback loops
- Surfacing validator concerns into delivery planning

**Does not own**
- Product roadmap authority
- Public social strategy
- Technical architecture by themselves

**Can decide without stakeholder approval**
- Routine coordination, information gathering, scheduling, follow-up with validators

**Must escalate**
- Commitments affecting launch timing, economics, protocol behavior, or ecosystem obligations

## 16) codex
**Owns**
- Fast implementation support under senior-dev or tech-lead direction
- Drafting code, tests, refactors, and technical scaffolding
- Producing artifacts that humans/lead agents can review and integrate

**Does not own**
- Final architectural decisions
- Priority decisions
- Self-assigning high-risk production changes without supervision

**Can decide without escalation**
- Implementation details inside a bounded task and clear design
- Suggested improvements submitted for review

**Must escalate**
- Ambiguous requirements, risky code paths, auth/funds/security-sensitive changes, or conflicts with established patterns

---

## Interaction Patterns

## Primary working relationships
- **Stakeholder → project-manager** for product direction, priorities, and business goals
- **project-manager ↔ scrummaster** daily for sprint control
- **project-manager ↔ tech-lead** for scope/design tradeoffs
- **scrummaster ↔ all delivery roles** for task coordination and blocker removal
- **tech-lead ↔ senior-dev / codex / junior-dev** for design and implementation guidance
- **senior-dev ↔ junior-dev / codex** for decomposition and execution support
- **qa-engineer / security-analyst / pr-reviewer ↔ implementation roles** as mandatory control points
- **stakeholder-liaison ↔ stakeholder** for concise updates and decision requests
- **community-manager / social-agent / validator-relations ↔ PM** when external feedback affects roadmap or promises
- **agile-coach ↔ PM + Scrum Master** for process tuning and team health

## Approved communication paths
### Product decisions
- Default path: **Stakeholder → PM → team**
- Team members should not seek direct stakeholder confirmation for routine execution.

### Execution decisions
- Default path: **PM ↔ Scrum Master ↔ delivery team**
- Tech Lead handles technical resolution.

### External messaging
- Default path: **PM/stakeholder-liaison provides message boundaries → social/community/validator roles execute**

### Research / uncertainty
- Default path: **PM or Tech Lead commissions researcher → researcher returns memo → PM/Tech Lead decide**

## Escalation paths
### Delivery blocker escalation
1. Assignee attempts resolution and documents blocker.
2. **Scrum Master** coordinates immediate unblocking.
3. **Tech Lead** resolves technical conflicts.
4. **PM** adjusts scope/priority if needed.
5. **Stakeholder** is informed only if sprint goal, timeline, or direction materially changes.

### Quality or security escalation
1. QA/Security flags issue with severity and evidence.
2. **Tech Lead + PM** decide fix-now vs de-scope vs hold release.
3. **Stakeholder** is notified if the issue changes launch confidence, public messaging, or business risk.

### External/reputation escalation
1. External role detects issue.
2. **Stakeholder-liaison + PM** align response.
3. **Stakeholder** is pulled in only for sensitive public, partner, or directional matters.

---

## Sprint Flow

The team should operate on a clear sprint cadence without turning the stakeholder into a daily approver.

## 1. Backlog refinement (pre-sprint)
**Owner:** project-manager
**Support:** tech-lead, scrummaster, researcher, QA, security as needed

Activities:
- Clarify backlog items and acceptance criteria
- Confirm business value and ordering
- Split oversized work
- Call research spikes where uncertainty is too high
- Identify security or QA-heavy items early

Exit criteria:
- Stories are small enough to assign
- Acceptance criteria are testable
- Dependencies and risks are visible

## 2. Sprint planning
**Owners:** project-manager + scrummaster
**Support:** tech-lead, delivery roles

Activities:
- PM proposes sprint goal and candidate stories
- Tech Lead validates technical feasibility
- Scrum Master converts scope into assignments and sequence
- Team commits to a realistic amount of work
- QA/Security identify required validation depth up front

Rule:
- Stakeholder does **not** approve sprint tasks one by one.
- Stakeholder receives the sprint goal, notable scope, and major risks as an update.

## 3. Sprint execution
**Owners:** scrummaster for flow, tech-lead for technical coherence

Activities:
- Daily async standup or status sweep
- Assignees update ticket status, blockers, and links to artifacts
- Scrum Master rebalances work as needed
- PM answers requirement questions and adjusts low-level scope inside the sprint goal
- Tech Lead reviews technical changes and integration points continuously

## 4. Mid-sprint adjustments
**Owners:** PM + Scrum Master

Allowed without stakeholder approval:
- Re-splitting stories
- Swapping equivalent work to protect the sprint goal
- Deferring non-critical stretch items
- Reassigning work across agents

Requires stakeholder input:
- Changing the sprint goal itself because business priorities changed
- Pulling in urgent work that displaces committed outcome for strategic reasons

## 5. Code review / validation / release flow
**Owners:** tech-lead, pr-reviewer, qa-engineer, security-analyst, devops

Sequence:
1. Implementation complete with tests/docs
2. PR review and requested changes
3. Tech Lead confirms design/integration fit
4. QA validates acceptance criteria and regression impact
5. Security reviews if risk warrants it
6. DevOps deploys using approved release path
7. PM confirms item meets product intent

No work is considered done because code exists. It is done when it is reviewed, validated, and in the target environment as appropriate.

## 6. Sprint review
**Owners:** project-manager + scrummaster

Activities:
- Summarize completed outcomes versus sprint goal
- Demo or document user-visible changes
- Capture misses, carryover, and reasons
- Send concise stakeholder update

Rule:
- Review is a status/reporting and learning point, not a request for retroactive permission to execute work already within scope.

## 7. Retrospective
**Owner:** agile-coach
**Support:** scrummaster, all relevant team roles

Focus:
- What slowed flow?
- Where were handoffs weak?
- What review gates were too late or too shallow?
- Which bottlenecks can be fixed in the next sprint?

Output:
- 1–3 concrete process changes with owners
- Not a vague discussion with no follow-through

---

## Task Delegation Model

Default execution chain for delivery work:

**PM prioritizes → Scrum Master assigns → Senior Dev designs/decomposes → Junior Dev or Codex implements → Tech Lead reviews/integrates → PR Reviewer reviews independently → DevOps deploys → QA validates → PM accepts against product intent**

## Practical rules

### 1. Work enters through the PM
Every delivery task must map to a backlog item, bug, spike, or release objective. No shadow work.

### 2. Scrum Master controls assignment
The Scrum Master decides who picks up what, based on skill fit, workload, and dependencies.

### 3. Senior Dev and Tech Lead shape implementation
Before coding starts on non-trivial work, the design owner should define:
- approach
- interfaces/contracts
- risks
- test expectations

### 4. Junior Dev and Codex execute bounded work
Implementation agents should receive tasks with:
- clear objective
- files/components affected
- constraints
- tests expected
- escalation triggers

### 5. Reviews happen before merge, not after deploy
At minimum:
- PR review for code quality
- Tech Lead review for integration/architecture fit
- QA validation for acceptance
- Security review for sensitive changes

### 6. DevOps owns deployment mechanics
Developers do not bypass release controls just because code is ready.

### 7. PM accepts the outcome, not the code style
PM confirms the story solves the intended product problem and matches acceptance criteria.

## Suggested ticket states
- `backlog`
- `ready`
- `in-progress`
- `in-review`
- `qa`
- `blocked`
- `done`
- `released` (optional if deployment is distinct from done)

## Definition of Ready
A task is ready when:
- the problem is clear
- value is understood
- acceptance criteria are testable
- dependencies are identified
- assignee knows the expected output

## Definition of Done
A task is done when:
- implementation is complete
- tests are added/updated
- PR review is complete
- Tech Lead concerns are resolved
- QA validation passes or approved exceptions are documented
- security review is complete if required
- docs/runbooks are updated if relevant
- PM accepts the outcome against the story intent

---

## Communication Protocol

## What gets reported to the stakeholder
The stakeholder should receive concise, decision-useful communication, not a stream of every implementation detail.

### Send proactively
- Sprint goal and planned outcomes at sprint start
- Mid-sprint summary only if risk or scope changed materially
- End-of-sprint outcome summary
- Major blockers that threaten delivery dates or business commitments
- Security/reliability incidents with impact summary and mitigation plan
- Decisions needed on product direction, external commitments, or economics

### Include in stakeholder updates
- what was completed
- what is at risk
- what changed and why
- what decision, if any, is needed
- recommended next step from PM

### Do not send upward by default
- every task assignment
- routine coding progress
- internal debate over implementation details
- minor bug triage
- normal review comments
- requests for approval on already-authorized sprint work

## What stays internal
- task decomposition
- day-to-day assignments
- technical implementation alternatives
- PR comments and code review loops
- routine QA defects and retests
- normal sprint reshuffling inside the goal

## Standard escalation message format
When escalation is required, use:
1. **Issue** — what happened
2. **Impact** — what it affects
3. **Options** — realistic paths forward
4. **Recommendation** — what PM/Tech Lead recommends
5. **Decision needed by** — only if an actual stakeholder decision is required

This keeps stakeholder involvement high-value and low-friction.

---

## Autonomy Boundaries

## The team can decide on its own
- backlog sequencing within approved product direction
- sprint scope selection and reshaping
- task assignment and reassignment
- implementation details and architecture within the agreed product target
- testing depth and release checklist requirements
- whether to split, defer, or de-scope non-essential work to protect the sprint goal
- copy, content, and timing for routine external messages inside approved messaging boundaries
- validator/community follow-ups that do not create new commitments

## The team must get stakeholder input for
- changes to product vision, MVP scope, or strategic direction
- changes to pricing, token/economic assumptions, or business model
- public commitments on launch dates, partnerships, or roadmap promises
- acceptance of major business risk, security risk, or reputational risk
- changes that materially alter the target user, core workflow, or success metric
- resourcing or priority shifts driven by leadership strategy rather than sprint execution

## Gray-area rule
If a decision changes **what** Tidepool is, **who** it is for, **what is promised publicly**, or **what risk the business is accepting**, involve the stakeholder.

If a decision only changes **how** the team executes to achieve the current goal, the team should handle it internally.

---

## Anti-Patterns to Avoid

1. **Stakeholder as ticket approver**
   - Wrong: asking TwiceBurnt to validate every next step.
   - Right: PM and Scrum Master make execution calls and escalate only material direction changes.

2. **Scrum Master acting like a passive status bot**
   - Wrong: reporting blockers without driving resolution.
   - Right: actively reassign, sequence, escalate, and unblock.

3. **PM doing task micromanagement**
   - Wrong: PM hand-assigning every subtask and dictating engineering details.
   - Right: PM owns priorities and acceptance; Scrum Master owns assignment; Tech Lead owns design.

4. **Tech Lead as single-threaded bottleneck**
   - Wrong: every tiny decision waits for the Tech Lead.
   - Right: Tech Lead sets patterns; Senior Devs execute within them.

5. **Overloading one agent because it is fastest**
   - Wrong: routing everything through the same agent and creating hidden queue time.
   - Right: distribute by role fit and maintain WIP limits.

6. **Junior/Codex coding without bounded instructions**
   - Wrong: vague tasks that produce thrash and rework.
   - Right: give clear scope, constraints, and escalation triggers.

7. **Skipping reviews to move faster**
   - Wrong: merging or deploying without PR/QA/security review appropriate to risk.
   - Right: shorten loops, not safeguards.

8. **QA at the very end with no context**
   - Wrong: treating QA as a last-minute gate.
   - Right: involve QA during planning for acceptance and regression coverage.

9. **Security only after launch panic**
   - Wrong: security reviews happen only when something breaks.
   - Right: review risk early for auth, funds, permissions, secrets, and critical flows.

10. **Research without time boxes**
   - Wrong: endless investigation delaying execution.
   - Right: researcher runs time-boxed spikes with explicit questions and decision outputs.

11. **External roles promising uncommitted features**
   - Wrong: social/community/validator roles making roadmap guarantees.
   - Right: communicate only approved truths.

12. **Invisible decisions in chat**
   - Wrong: important decisions buried in message threads.
   - Right: record them in the ticket, PR, or decision doc.

---

## Ceremonies and Cadence

## Daily async standup
**Owner:** Scrum Master

Each active assignee reports:
- what changed since last update
- what is next
- blockers
- links to evidence

## Backlog refinement
**Owner:** PM
**Cadence:** at least once per sprint, more if backlog quality drops

## Sprint planning
**Owners:** PM + Scrum Master
**Cadence:** once per sprint

## Review/demo
**Owner:** PM
**Cadence:** end of sprint or release milestone

## Retrospective
**Owner:** Agile Coach
**Cadence:** end of sprint

## Release checkpoint
**Owners:** Tech Lead + QA + DevOps
**Cadence:** per release candidate

---

## Working Agreements for an AI Agent Team

1. **Write down assumptions.** If you infer something important, document it.
2. **Link artifacts.** Tickets should link to PRs, test evidence, docs, and deployment notes.
3. **Use explicit handoffs.** Say who owns the next move.
4. **Prefer bounded tasks.** Smaller tasks reduce hallucinated scope and rework.
5. **State confidence and risk.** Especially for research, security, and release decisions.
6. **Escalate early on ambiguity.** Silent guessing is worse than fast clarification.
7. **No orphaned work.** Every work item has an owner and a status.
8. **No merge without accountability.** Somebody must review and somebody must own post-merge follow-through.

---

## Final Rule

The Tidepool team is designed to operate like a real autonomous product squad.

That means:
- **PM decides what matters now**
- **Scrum Master decides how work moves**
- **Tech Lead decides how it should be built**
- **Engineering roles build it**
- **QA/Security/Review roles protect quality**
- **DevOps gets it safely live**
- **Stakeholder is updated, not turned into a bottleneck**

If the team is repeatedly waiting for stakeholder approval on normal sprint decisions, the process is broken and must be corrected.