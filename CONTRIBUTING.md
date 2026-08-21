# Contributing to the benchmarks

We welcome all interested contributors, and appreciate the work you put in.

# Maintainers

These benchmarks are maintained by a small group of volunteers. You may reach
out to them for support, but avoid @-ing them if possible. If you feel ignored,
it's more likely that we just haven't gotten to you yet. We want you to succeed!

# Updating crates

Updating crates that are already part of the benchmark is an easy way to improve
the benchmarks. When updating crates that are part of the benchmark:

- The version of the updated crate should be the most recently published
  semver-compatible version. Do not update to prerelease versions.
- The version of the updated crate should at least a week old.
- Any updates should be as minimal as possible. Only update one framework at a
  time, and only update the crate.

Crates tend to make many incremental improvements soon after being published, as
new users report issues or suggest improvements. This semver and age
requirements help reduce the code churn and review burden associated with
updating a crate. Additionally, this age requirement acts as a dependency
cooldown for improved repo security.

# Adding crates

Adding new crates to the benchmark merits a higher level of scrutiny than
updating existing crates. When adding new crates to the benchmark:

- Do not add your temporary or one-off projects that are not maintained. Adding
  these simultaneously adds a maintenance burden, while also promoting a crate
  that is not useful to evaluate.
- If possible, please include a GitHub handle to someone who is willing to help
  maintain the benchmark integration for the added crate. We sometimes run into
  issues updating other developers' crates, and having someone we can ping is
  useful for us. Do not sign anyone up for maintenance without their consent.

# Removal policy

Unmaintained crates are subject to removal from the benchmarks.

At times, we may update the crates included in the benchmark ourselves. If we
run into significant issues with a crate, we may reach out to its contact if
one was provided when the crate was added. If we still can't get the integration
issues resolved, we may remove the crate from the benchmarks.

# AI policy

- Maintainers expect to interact with humans. Write your commit titles and
  messages by hand, and do not use AI to communicate on pull requests. Do not
  copy and paste output from an AI.
- Disclose your use of AI upfront. This helps reviewers look for specific kinds
  of problems that are more common in AI-generated code.
- We have the same standards for human-authored and AI-generated code: it should
  be clean, readable, and not overly verbose or complicated.

We reserve the right to enforce this policy without warning when appropriate.
