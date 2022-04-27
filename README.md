![Tests Passing](https://github.com/amjerm/branchify/actions/workflows/run-tests.yml.badge.svg)

# branchify

Git branch name helper written in Rust.

## Usage

Pass a ticket number and summary separated by a tab character via stdin and receive a feature branch name.

```bash
# command
echo "FOO-123\tThis is a ticket" | branchify

#output
feature/FOO-123-this-is-a-ticket
```

Specifying a branch type

```bash
# command
echo "FOO-123\tThis is a ticket" | branchify -t hotfix

#output
hotfix/FOO-123-this-is-a-ticket
```

With a prefix before the branch type

```bash
# command
echo "FOO-123\tThis is a ticket" | branchify -p adam

#output
adam/feature/FOO-123-this-is-a-ticket
```

The branch name will also be truncated to 40 characters

```bash
# command
echo "FOO-123\tThis ticket has a longer name"| branchify

#output
feature/FOO-123-this-ticket-has-a-longer
```

Used in conjunction with [`jira-cli`](https://github.com/ankitpokhrel/jira-cli) and [`fzf`](https://github.com/junegunn/fzf)

```bash
# command
git checkout -b $(jira issue list --plain --columns key, summary | fzf | branchify)
```
