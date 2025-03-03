---
description: Accepts `log` events and allows you to transform events with a full embedded Lua engine.
---

<!--
     THIS FILE IS AUTOGENERATED!

     To make changes please edit the template located at:

     scripts/generate/templates/docs/usage/configuration/transforms/lua.md.erb
-->

# lua transform

![][assets.lua_transform]

{% hint style="warning" %}
The `lua` transform is in beta. Please see the current
[enhancements][urls.lua_transform_enhancements] and
[bugs][urls.lua_transform_bugs] for known issues.
We kindly ask that you [add any missing issues][urls.new_lua_transform_issue]
as it will help shape the roadmap of this component.
{% endhint %}

The `lua` transform accepts [`log`][docs.data-model.log] events and allows you to transform events with a full embedded [Lua][urls.lua] engine.

## Example

{% code-tabs %}
{% code-tabs-item title="vector.toml (simple)" %}
```coffeescript
[transforms.my_transform_id]
  type = "lua" # must be: "lua"
  inputs = ["my-source-id"]
  source = """
require("script") # a `script.lua` file must be in your `search_dirs`

if event["host"] == nil then
  local f = io.popen ("/bin/hostname")
  local hostname = f:read("*a") or ""
  f:close()
  hostname = string.gsub(hostname, "\n$", "")
  event["host"] = hostname
end
"""
```
{% endcode-tabs-item %}
{% code-tabs-item title="vector.toml (advanced)" %}
```coffeescript
[transforms.my_transform_id]
  # REQUIRED
  type = "lua" # must be: "lua"
  inputs = ["my-source-id"]
  source = """
require("script") # a `script.lua` file must be in your `search_dirs`

if event["host"] == nil then
  local f = io.popen ("/bin/hostname")
  local hostname = f:read("*a") or ""
  f:close()
  hostname = string.gsub(hostname, "\n$", "")
  event["host"] = hostname
end
"""
  
  # OPTIONAL
  search_dirs = ["/etc/vector/lua"] # no default
```
{% endcode-tabs-item %}
{% endcode-tabs %}

## Options

### search_dirs

`optional` `no default` `type: [string]` `example: ["/etc/vector/lua"]`

A list of directories search when loading a Lua file via the `require` function. See [Search Directories](#search-directories) for more info.

### source

`required` `type: string` `example: """
require("script") # a `script.lua` file must be in your `search_dirs`

if event["host"] == nil then
  local f = io.popen ("/bin/hostname")
  local hostname = f:read("*a") or ""
  f:close()
  hostname = string.gsub(hostname, "\n$", "")
  event["host"] = hostname
end
"""`

The inline Lua source to evaluate. See [Global Variables](#global-variables) for more info.

## Input/Output

{% tabs %}
{% tab title="Add fields" %}
Add a field to an event. Supply this as a the `source` value:

```lua
# Add root level field
event["new_field"] = "new value"

# Add nested field
event["parent.child"] = "nested value"
```

{% endtab %}
{% tab title="Remove fields" %}
Remove a field from an event. Supply this as a the `source` value:

```lua
# Remove root level field
event["field"] = nil

# Remove nested field
event["parent.child"] = nil
```

{% endtab %}
{% tab title="Drop event" %}
Drop an event entirely. Supply this as a the `source` value:

```lua
# Drop event entirely
event = nil
```

{% endtab %}
{% endtabs %}

## How It Works

### Dropping Events

To drop events, simply set the `event` variable to `nil`. For example:

```lua
if event["message"].match(str, "debug") then
  event = nil
end
```

### Environment Variables

Environment variables are supported through all of Vector's configuration.
Simply add `${MY_ENV_VAR}` in your Vector configuration file and the variable
will be replaced before being evaluated.

You can learn more in the [Environment Variables][docs.configuration#environment-variables]
section.

### Global Variables

When evaluating the provided `source`, Vector will provide a single global
variable representing the event:

| Name    |           Type           | Description                                                                                                                                                                       |
|:--------|:------------------------:|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `event` | [`table`][urls.lua_table] | The current [`log` event]. Depending on prior processing the structure of your event will vary. Generally though, it will follow the [default event schema][docs.data-model.log#default-schema]. |

Note, a Lua `table` is an associative array. You can read more about
[Lua types][urls.lua_types] in the [Lua docs][urls.lua_docs].

### Nested Fields

As described in the [Data Model document][docs.data_model], Vector flatten
events, representing nested field with a `.` delimiter. Therefore, adding,
accessing, or removing nested fields is as simple as added a `.` in your key
name:

```lua
# Add nested field
event["parent.child"] = "nested value"

# Remove nested field
event["parent.child"] = nil
```

### Search Directories

Vector provides a `search_dirs` option that allows you to specify absolute
paths that will searched when using the [Lua `require`
function][urls.lua_require].

## Troubleshooting

The best place to start with troubleshooting is to check the
[Vector logs][docs.monitoring#logs]. This is typically located at
`/var/log/vector.log`, then proceed to follow the
[Troubleshooting Guide][docs.troubleshooting].

If the [Troubleshooting Guide][docs.troubleshooting] does not resolve your
issue, please:

1. Check for any [open `lua_transform` issues][urls.lua_transform_issues].
2. If encountered a bug, please [file a bug report][urls.new_lua_transform_bug].
3. If encountered a missing feature, please [file a feature request][urls.new_lua_transform_enhancement].
4. If you need help, [join our chat/forum community][urls.vector_chat]. You can post a question and search previous questions.


### Alternatives

Finally, consider the following alternatives:

* [`lua` transform][docs.transforms.lua]

## Resources

* [**Issues**][urls.lua_transform_issues] - [enhancements][urls.lua_transform_enhancements] - [bugs][urls.lua_transform_bugs]
* [**Source code**][urls.lua_transform_source]
* [**Lua Reference Manual**][urls.lua_manual]


[assets.lua_transform]: ../../../assets/lua-transform.svg
[docs.configuration#environment-variables]: ../../../usage/configuration#environment-variables
[docs.data-model.log#default-schema]: ../../../about/data-model/log.md#default-schema
[docs.data-model.log]: ../../../about/data-model/log.md
[docs.data_model]: ../../../about/data-model
[docs.monitoring#logs]: ../../../usage/administration/monitoring.md#logs
[docs.transforms.lua]: ../../../usage/configuration/transforms/lua.md
[docs.troubleshooting]: ../../../usage/guides/troubleshooting.md
[urls.lua]: https://www.lua.org/
[urls.lua_docs]: https://www.lua.org/manual/5.3/
[urls.lua_manual]: http://www.lua.org/manual/5.1/manual.html
[urls.lua_require]: http://www.lua.org/manual/5.1/manual.html#pdf-require
[urls.lua_table]: https://www.lua.org/manual/2.2/section3_3.html
[urls.lua_transform_bugs]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22transform%3A+lua%22+label%3A%22Type%3A+bug%22
[urls.lua_transform_enhancements]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22transform%3A+lua%22+label%3A%22Type%3A+enhancement%22
[urls.lua_transform_issues]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22transform%3A+lua%22
[urls.lua_transform_source]: https://github.com/timberio/vector/tree/master/src/transforms/lua.rs
[urls.lua_types]: https://www.lua.org/manual/2.2/section3_3.html
[urls.new_lua_transform_bug]: https://github.com/timberio/vector/issues/new?labels=transform%3A+lua&labels=Type%3A+bug
[urls.new_lua_transform_enhancement]: https://github.com/timberio/vector/issues/new?labels=transform%3A+lua&labels=Type%3A+enhancement
[urls.new_lua_transform_issue]: https://github.com/timberio/vector/issues/new?labels=transform%3A+lua
[urls.vector_chat]: https://chat.vector.dev
