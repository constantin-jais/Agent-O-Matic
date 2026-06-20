# Biscuit Auth

Use Biscuit tokens for attenuable, locally-verifiable authorization. Treat them
as logic programs, not opaque JWT replacements.

## Token model

- Authority block is created only by the auth issuer.
- Tokens carry facts and checks; services provide authorizer facts and policies.
- Closed world: what is not explicitly allowed is denied.
- Attenuation may restrict rights but never expand them.
- Tokens must expire through a token check, not only through service-side logic.

## Required authority facts

```datalog
user("user_id");
tenant("tenant_id");
role("user_id", "role");
check if time($time), $time < 2026-12-31T23:59:59Z;
```

- `tenant()` is mandatory for multi-tenant isolation.
- Do not store PII, passwords, secrets, or emails in tokens.
- Prefer short TTLs and explicit attenuation for delegated operations.

## Authorizer rules

- Inject service context: `time`, `resource`, `operation`, tenant boundary, and
  any request-specific facts.
- Keep policies in the authorizer and checks in the token.
- End with an explicit deny policy.
- Test every policy with allow and deny fixtures.

## Rust integration

- Validate locally using the public key set.
- Expose a typed extractor/middleware that returns a validated principal.
- Redact tokens in logs and tracing spans.
- Cache revocation checks only with short TTLs.
- Rotate keys by accepting old and new public keys until old tokens expire.

## Defense in depth

- Pair token tenant facts with PostgreSQL RLS where persistence is multi-tenant.
- Include authorization scenarios in contract/integration tests.
- Version `.datalog` policy fixtures and include them in release evidence.

## Forbidden

- JWT as the default internal authorization format.
- Client-side token creation.
- Tokens without expiration.
- Tokens without tenant facts for tenant-scoped systems.
- Shared private keys across services.
- Logging full token contents.
