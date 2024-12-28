# Contributing Guidelines

Thank you for your interest in contributing to this project! Please follow these guidelines to ensure consistency.

## Branch Naming Convention

Branch names should follow this format:
`<type>/<description>`

Types match our commit types:
- `feat/` - for new features
- `fix/` - for bug fixes
- `docs/` - for documentation
- `style/` - for style changes
- `refactor/` - for refactoring
- `test/` - for test updates
- `chore/` - for maintenance

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages: <type>[optional scope]: <description>
[optional body]
[optional footer(s)]

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, missing semicolons, etc)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

## Pull Requests

1. Fork the repository
2. Create a new branch from `main`
3. Make your changes
4. Write/update tests if needed
5. Ensure all tests pass
6. Submit a pull request

### PR Title

Follow the same conventional commit format for PR titles.

## Release Management

wip

<!-- We use [Release Please](https://github.com/googleapis/release-please) for automated version management and changelog generation. The tool:
- Automatically creates release PRs based on conventional commits
- Updates version numbers according to semantic versioning
- Generates changelogs from commit messages
- Creates GitHub releases when merged -->

### Versioning

This project follows the [CalVer](https://calver.org/) versioning scheme in the format `YYYY.MM.MICRO`:

- `YYYY`: Full year (e.g. 2024)
- `MM`: Month (01-12)
- `MICRO`: Patch version, starting at 0 (e.g. 0, 1, 2...)

Example: `2024.03.0`

> [!NOTE]
> The MICRO number resets to 0 with each new month.

## Questions?

Feel free to open an issue if you have any questions!