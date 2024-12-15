# Prompts Directory

This directory contains YAML files that define prompts for the MCP protocol.

## Structure

Each prompt is defined in a YAML file with the following structure:

```yaml
name: prompt_name
description: "Description of what the prompt does"
arguments:
  - name: arg1
    description: "Description of argument 1"
    required: true
  - name: arg2
    description: "Description of argument 2"
    required: false
messages:
  - role: system
    content:
      type: text
      text: "System message that sets up the context"
  - role: user
    content:
      type: resource
      uri: "{arg1}"
```

## Available Prompts

- `code_review.yaml` - Performs a code review on a given file