---
description: "Base project rules - common rules for all project types"
alwaysApply: true
---

# AI Rules for chromamancer

## Sudo Command Handling

**CRITICAL RULE**: Sudo commands should be requested for execution by the user along with a validation command. Once the user runs the sudo command and informs the agent, the agent should validate that the command was successful with the validation command and continue work.

**Process**:
1. Identify that a command requires sudo
2. Present the sudo command to the user with a validation command
3. Wait for user confirmation that sudo command was executed
4. Run the validation command to confirm success
5. Continue with the workflow

**Example**:
- Required: `sudo apt-get install package`
- Validation: `which package` or `package --version`
- Agent presents both commands, waits for user, then validates

## Project Context

This is chromamancer, a desktop-theme project.

Spec-driven desktop theming: live apply, Nix/HM, scheme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## General Best Practices

- Follow clean code principles
- Write comprehensive tests
- Document code and APIs
- Use meaningful variable and function names
- Keep functions small and focused
- Handle errors gracefully
- Avoid code duplication
- Prefer composition over inheritance
- Write self-documenting code

## Development Workflow

- Create feature branches: `feature/`, `bug/`, `polish/`, `design/`
- Test locally before committing
- Use conventional commits (feat, fix, docs, style, refactor, test, chore)
- Create PRs for review before merging
- Ensure all tests pass before committing
- Run linters and formatters before committing

## Code Quality

- Write code that is easy to read and understand
- Keep functions focused on a single responsibility
- Use appropriate abstractions
- Avoid premature optimization
- Refactor when code smells are detected
- Write tests alongside code, not after

## Documentation

- Document public APIs
- Keep README up to date
- Update CHANGELOG for user-facing changes
- Document architectural decisions in ARCHITECTURE.md
- Use inline comments sparingly (code should be self-documenting)
- Write clear commit messages

## Security

- Never commit secrets or API keys
- Use environment variables for sensitive configuration
- Validate and sanitize user input
- Follow security best practices for the technology stack
- Keep dependencies up to date
- Review security advisories regularly
