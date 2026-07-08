# Session Replays

Session replays make Qortek inspectable. They show what was attempted, what changed, how it was verified, and what remains unproven.

A replay is not a chat transcript. It is an engineering record.

## Replay template

```md
# Replay: <short name>

- Date:
- Status: planned | scaffolded | implemented | verified
- Scope:
- Repos touched:
- Public/private boundary risk:

## Goal

## Starting state

## Steps

## Commands run

## Expected result

## Actual result

## Files changed

## Verification evidence

## Not verified

## Next action
```

## Rules

- Do not include secrets.
- Do not include private waiver/legal text.
- Do not paste private source unless the replay lives in a private repo.
- Use public-safe summaries for private-track work.
- Mark unverified work clearly.

## Required replays before public alpha

- [x] public scaffold
- [ ] first local memory moat
- [ ] vector alignment journal
- [ ] adapter parity
- [ ] release candidate run
