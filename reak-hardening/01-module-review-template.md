# Module review template

Copy one file per module under `reak-hardening/modules/<name>.md`.

## Module

- Name:
- Path:
- Commit:
- Owner:

## Trust boundary

- Holds secrets:
- Talks to network:
- Parses untrusted input:
- Persists state:

## Threat model (adversarial)

| Actor | Goal | Capability assumed |
| --- | --- | --- |
| Local caller | Misuse API | Can call public API in any order |
| Network peer | Forge or replay | As documented for this module |
| Operator | Break glass | As documented |
| Compromised dependency | Supply malicious input | As documented |

## Attack surface

List every entry point: functions, RPCs, files read, env vars, config keys.

## Abuse cases

Deliberate hostile use that stays within documented API.

## Misuse cases

Benign caller mistakes that cause harm.

## Failure modes

Crash, hang, corrupt state, silent wrong answer, duplicate effect, lost record.

## Recovery paths

After crash, panic, partial write, or split brain. Cite observed behaviour, not intent.

## Findings

### Security

| ID | Severity | Verdict | Summary | Evidence |
| --- | --- | --- | --- | --- |

### Complexity

| ID | Verdict | Summary | Evidence |
| --- | --- | --- | --- |

### Performance

| ID | Verdict | Summary | Evidence |
| --- | --- | --- | --- |

### Maintainability

| ID | Verdict | Summary | Evidence |
| --- | --- | --- | --- |

## Tests to add

| Kind | Target | Property or oracle |
| --- | --- | --- |
| Property | | |
| Fuzz | | |
| Mutation | | |
| Concurrency | | |
| Crash | | |
| Soak | | |
| Fault injection | | |
| Corruption | | |
| Exhaustion | | |

## Simplification evaluation

Answer each: yes / no / blocked, with evidence.
