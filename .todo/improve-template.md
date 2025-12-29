# Task 1.5: Template System Upgrade - Nested Parsing & Concatenation

**Status:** Planned

**User Input:** 模板系统需要支持字符串拼接和嵌套解析

**Blocker:** None (can implement independently)

## Current Limitation

```yaml
# Current: Only single-level replacement
upstream: "{{kv.backend_host}}"  # Works

# Not supported yet: Concatenation
upstream: "{{conn.ip}}:{{conn.port}}"  # Fails

# Not supported yet: Nested resolution
backend: "{{kv.{{conn.protocol}}_backend}}"  # Fails
```

## Required Features

### 1. String Concatenation Support

```yaml
# Example 1: IP + Port
upstream: "{{conn.ip}}:{{conn.port}}"
# If conn.ip = "1.2.3.4", conn.port = "8080"
# Result: "1.2.3.4:8080"

# Example 2: URL construction
url: "https://{{kv.domain}}/api/{{req.path}}"
# If kv.domain = "example.com", req.path = "users"
# Result: "https://example.com/api/users"

# Example 3: Multiple replacements in text
log_message: "Request from {{conn.ip}} to {{req.path}} returned {{resp.status}}"
```

### 2. Nested Template Resolution

```yaml
# Example 1: Dynamic key lookup
backend: "{{kv.{{conn.protocol}}_backend}}"
# Step 1: {{conn.protocol}} → "http"
# Step 2: "kv.http_backend"
# Step 3: {{kv.http_backend}} → "backend-01"
# Result: "backend-01"

# Example 2: Multi-level nesting
value: "{{kv.{{geo.{{conn.ip}}}}_server}}"
# Step 1: {{conn.ip}} → "1.2.3.4"
# Step 2: {{geo.1.2.3.4}} → "US"
# Step 3: {{kv.US_server}} → "us-east-1"
# Result: "us-east-1"

# Example 3: Nested in URL
url: "https://{{kv.{{geo.country}}_domain}}/api"
# Step 1: {{geo.country}} → "US"
# Step 2: {{kv.US_domain}} → "api.example.com"
# Step 3: Concatenate with text
# Result: "https://api.example.com/api"
```

## Parsing Rules

1. **Parse from innermost to outermost**: Start with deepest `{{...}}` first
2. **Support text mixing**: Inner results can concatenate with plain text before outer resolution
3. **Arbitrary nesting depth**: No artificial limit on nesting levels
4. **Fail-fast on missing keys**: If any resolution fails, entire template fails (no partial replacement)

## Implementation Plan

### Phase 1: Lexer/Parser Refactor
- [ ] Current: Simple regex replacement (`{{key}}` → value)
- [ ] New: Build AST (Abstract Syntax Tree) for templates
  - Text nodes: Plain string segments
  - Variable nodes: `{{...}}` with nested children
  - Support recursive parsing

### Phase 2: Resolver Engine

```rust
pub enum TemplateNode {
    Text(String),                    // Plain text
    Variable {                       // {{...}}
        path: Vec<TemplateNode>,     // Can contain nested nodes
    },
}

pub fn resolve_template(
    template: &str,
    context: &dyn TemplateContext,
) -> Result<String> {
    // 1. Parse template into AST
    let ast = parse_template(template)?;

    // 2. Resolve from innermost to outermost
    let resolved = resolve_ast(ast, context)?;

    Ok(resolved)
}

fn resolve_ast(
    node: TemplateNode,
    context: &dyn TemplateContext,
) -> Result<String> {
    match node {
        TemplateNode::Text(s) => Ok(s),
        TemplateNode::Variable { path } => {
            // Recursively resolve nested nodes first
            let mut key_parts = Vec::new();
            for part in path {
                key_parts.push(resolve_ast(part, context)?);
            }
            let key = key_parts.join("");

            // Then lookup in context
            context.get(&key)
        }
    }
}
```

### Phase 3: Update Template Context Trait

```rust
pub trait TemplateContext {
    fn get(&self, key: &str) -> Result<String>;
}

// Implement for KV Store
impl TemplateContext for KvStore {
    fn get(&self, key: &str) -> Result<String> {
        self.get(key).ok_or_else(|| anyhow!("Key not found: {}", key))
    }
}

// Implement for hijack keywords (L7+HTTPX)
impl TemplateContext for HttpHijackContext {
    fn get(&self, key: &str) -> Result<String> {
        match key {
            "req.body" => self.buffer_request_body().await,
            "req.headers.Host" => self.get_header("Host"),
            _ => self.kv.get(key),  // Fallback to KV
        }
    }
}
```

### Phase 4: Testing

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_concatenation() {
        let mut kv = KvStore::new();
        kv.set("conn.ip", "1.2.3.4");
        kv.set("conn.port", "8080");

        let result = resolve_template("{{conn.ip}}:{{conn.port}}", &kv)?;
        assert_eq!(result, "1.2.3.4:8080");
    }

    #[test]
    fn test_nested() {
        let mut kv = KvStore::new();
        kv.set("conn.protocol", "http");
        kv.set("kv.http_backend", "backend-01");

        let result = resolve_template("{{kv.{{conn.protocol}}_backend}}", &kv)?;
        assert_eq!(result, "backend-01");
    }

    #[test]
    fn test_complex() {
        let mut kv = KvStore::new();
        kv.set("geo.country", "US");
        kv.set("kv.US_domain", "api.example.com");

        let result = resolve_template("https://{{kv.{{geo.country}}_domain}}/api", &kv)?;
        assert_eq!(result, "https://api.example.com/api");
    }
}
```

## Edge Cases to Handle

- Empty templates: `""` → `""`
- No variables: `"plain text"` → `"plain text"`
- Adjacent variables: `"{{a}}{{b}}"` → concat both
- Missing keys: Return error (don't silently fail)
- Circular references: Detect and error (e.g., `{{kv.{{kv.x}}}}` where `kv.x = "x"`)
- Escape sequences: How to write literal `{{` in template? (e.g., `\{{` or `{{{{`)

## Migration

- Backward compatible: Existing simple templates still work
- No breaking changes: `{{key}}` behavior unchanged
- Gradual adoption: Users can use new features when needed

## Benefits

- More expressive configuration (no need for workarounds)
- Dynamic routing based on runtime values
- Reduce need for custom plugins for simple transformations

## Impact

Enhances template system without breaking existing configs

## Complexity

Medium (parser design + careful testing for edge cases)

## Estimated Time

3-5 days
