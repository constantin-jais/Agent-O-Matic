# Local LLM testing with LM Studio

Local LLM tests should not require provider secrets and should not be wired into
public GitHub Actions. The recommended local model endpoint is LM Studio on the
operator workstation.

## Recommended local model

- Provider/runtime: LM Studio
- Base URL: `http://127.0.0.1:1234/v1`
- Model: `google/gemma-4-26b-a4b-qat`

## Smoke test

Start LM Studio, load the model, enable the local server, then run:

```sh
curl http://127.0.0.1:1234/v1/chat/completions \
  -H 'Content-Type: application/json' \
  -d '{
    "model": "google/gemma-4-26b-a4b-qat",
    "messages": [
      {"role": "system", "content": "You are a concise local smoke-test assistant."},
      {"role": "user", "content": "Reply with: bolt local llm ok"}
    ],
    "temperature": 0
  }'
```

Expected: a local response containing `bolt local llm ok` or a very close variant.

## Current integration boundary

`bolt-cosmatic loop --dry-run` and `fixer=stub` do not use an LLM.

The current write-capable fixer implementation is a Claude Code CLI seam retained
for historical/live-boundary work. It is not the recommended public demo path and
is not evidence that an Anthropic key exists or is required.

LM Studio/Gemma is the preferred local LLM test target, but it is not yet a
write-capable fixer backend. Do not expose `127.0.0.1:1234` through public CI.

## Secret rules

- LM Studio local tests require no repository secret.
- Do not store local model URLs or prompts in GitHub secrets.
- Do not commit generated local transcripts unless they are scrubbed evidence.
- If a cloud provider fixer is intentionally tested in the future, rotate that
  provider key separately; otherwise there is no provider key to rotate.
