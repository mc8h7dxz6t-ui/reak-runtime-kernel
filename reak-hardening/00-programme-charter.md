# Programme charter

## Mission

Break the kernel before customers do. Act as an adversarial engineering team. Challenge everything. Trust nothing.

## Non-goals

- Redesign frozen architecture
- Add product features
- Expand constitutional or product scope
- Ship fixes without an explicit owner and qualification step

## Review dimensions (every module)

Security, memory safety, concurrency, races, deadlocks, replay, privilege escalation, authority confusion, state corruption, serialization and deserialization attacks, crash recovery, panic safety, determinism, performance collapse, DoS resistance, API misuse, hidden coupling, dependency risk, cryptographic misuse, unintended side effects.

## Outputs per module

Threat model (attacker view), attack surface, abuse cases, misuse cases, failure modes, recovery paths, security findings, complexity findings, performance findings, maintainability findings.

## Test generation obligations

Property tests, fuzz cases, mutation tests, concurrency tests, crash simulations, long soak tests, random fault injection, state corruption tests, resource exhaustion tests. Each must map to a module and a failure mode hypothesis.

## Simplification questions (every module)

Can it be simplified? Can dependencies be removed? Can ownership be clearer? Can interfaces be smaller? Can state be more immutable? Can attack surface shrink?

## Verdict vocabulary

- **CONFIRMED** — demonstrated on artefacts in scope
- **HYPOTHESIS** — plausible, test not yet run
- **NOT RUN** — blocked on intake
- **NOT APPLICABLE** — module or path absent
