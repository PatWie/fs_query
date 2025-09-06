# fs-query: The AI Assistant's Best Friend 🤖

*Finally, a tool that doesn't read your entire codebase just to find one function*

## The Problem

Picture this: You ask your AI assistant a simple question like "show me the main function in this project." What happens? Most MCP tools treat code files like plain text and proceed to ingest your entire codebase like a digital vacuum cleaner having an existential crisis.

**Traditional MCP file reading:**
1. "Oh, you want one function? Let me read ALL the files!"
2. *Slurps up 50,000 lines of code as plain text*
3. *Burns through your token budget like a crypto miner*
4. "Here's your function... and also the complete works of your test suite, documentation, and that random TODO.txt from 2019"

**fs-query's approach:**
1. "You want functions? Here are the functions."
2. *Returns exactly what you asked for using proper code parsing*
3. *Uses 47 tokens instead of 47,000*
4. *Actually understands code structure*

## What Is This Magical Contraption?

fs-query is a **Model Context Protocol (MCP) server** that speaks fluent "symbol extraction" to AI assistants. Think of it as a translator between your AI and your codebase - one that actually understands the difference between a function definition and a function call in a comment.

It's powered by Tree-sitter, which is basically the difference between a surgeon and someone with a chainsaw when it comes to parsing code.

## MCP Integration: The Main Event 🎭

### For AI Assistants (The VIPs)

Your AI assistant can now ask intelligent questions like:
- "Show me all the classes in this project"
- "Find functions matching this pattern"
- "What structs are defined in the networking module?"

Instead of getting a novel-length response, it gets exactly what it needs. Revolutionary, I know.

### Configuration for Q Chat

Drop this into your MCP config and watch the magic happen:

```json
{
    "mcpServers": {
        "fs_query": {
            "command": "/path/to/fs_query",
            "args": ["mcp"],
            "env": {},
            "transportType": "stdio",
            "timeout": 120000,
            "disabled": false
        }
    }
}
```

### What Your AI Can Do Now

**Before fs-query:**
- AI: "Let me read your entire project to find that one function..."
- *5 minutes later*
- AI: "Here's your function, plus 10,000 lines of context you didn't ask for"

**After fs-query:**
- AI: "Let me query the symbols..."
- *0.1 seconds later*
- AI: "Here's exactly what you wanted, no more, no less"

## The Technical Bits (For Humans Who Still Code)

### Languages We Actually Understand
- **Rust** (because we have taste)
- **Python** (for the data scientists)
- **C++** (for the masochists)
- **JavaScript/TypeScript** (for the web people)
- **Go** (for the Google fans)

### Symbol Types We Can Find
- Functions (the workhorses)
- Classes (the organizers)
- Structs (the honest ones)
- Variables (the state keepers)
- Enums (the option providers)
- Traits/Interfaces (the contract writers)

### CLI Usage (The Side Quest)

Sure, you can use it from the command line too, if you're into that sort of thing:

```bash
# Find all functions with "handle" in the name
./fs_query extract-symbols --file-path "src/" --symbols function --name-regex ".*handle.*"

# Get all classes, because you're curious
./fs_query extract-symbols --file-path "**/*.py" --symbols class --pretty
```

But honestly, the real magic happens when your AI uses it.

## Why This Exists (A Brief Rant)

I got tired of watching AI assistants read entire codebases just to answer simple questions. It's like asking someone what time it is and having them explain how clocks work, the history of timekeeping, and their personal relationship with punctuality.

**The traditional MCP problem:**
- Asks for a function → Gets the entire file
- Asks for class info → Gets the whole project
- Asks for a variable → Gets a dissertation on software architecture

**The fs-query solution:**
- Asks for a function → Gets the function
- Asks for class info → Gets the classes
- Asks for a variable → Gets the variables

Mind-blowing, right?

## Performance Philosophy

**Other tools:** "Let me index your entire project first..."
**fs-query:** "Nah, I'll just parse what you need, when you need it"

**Other tools:** "Here's 50MB of context!"
**fs-query:** "Here's exactly what you asked for"

**Other tools:** "Loading... please wait... still loading..."
**fs-query:** "Done. What's next?"

## Real Talk: Why MCP Matters

MCP (Model Context Protocol) is like having a universal translator between AI assistants and your development tools. Instead of every AI reinventing the wheel with custom integrations, they can all speak the same language.

fs-query plugs into this ecosystem and makes your AI assistant actually useful for code navigation instead of just a very expensive grep replacement.

## Installation & Setup

```bash
# Install from crates.io
cargo install fs_query

# Or build from source
cargo build --release

# Test it works
fs_query extract-symbols --file-path "." --pretty

# Add to your AI assistant's MCP config
# (See the JSON above)
```

## The Bottom Line

If you're tired of your AI assistant reading War and Peace when you just want to know what functions are in a file, fs-query is for you.

If you think parsing code with regex is a reasonable approach in 2024, this tool is definitely for you.

If you believe AI assistants should be precise surgical instruments rather than digital bulldozers, welcome to the club.

---

**Built with:** Tree-sitter (the good stuff), Rust (obviously), and a healthy disdain for inefficient tooling.

**License:** MIT (because life's too short for complicated licenses)

**Warranty:** None. But it probably won't delete your code. Probably.
