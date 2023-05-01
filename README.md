<div align="center">

![banner](.github/assets/banner.webp)

# Eight

Modular asynchronous embedded key-value database.

</div>

> ⚠️ You are currently viewing main branch ⚠️

Eight is a modular asynchronous embedded key-value database. It's modular because it has something much powerful that can change eight into anything: storages. If something implements Storage trait, it can be used with **Eight Server**, that can be hosted with **Eight Expose** and then can be used with **Eight Client**. You can implement LRU Cache, Redis Storage, and even use with some database like MySQL, MongoDB, SurrealDB and so on...

You can make your own storage implementation and take advantages of **Eight Server**: Redis-like query language, asynchronous command execution, user permissions etc... This is why eight is not just a simple embedded database.

Eight currently ships two default storage implementations: In-memory storage and Filesystem based storage. If you don't like to use them, make your own storage and publish it as a crate!

- For a quick start, visit [Introducing Eight](https://meppu.boo/blog/introducing-eight/).
- For more information about embedded database itself, please visit [eight/README.md](eight/README.md).
- For more information about `eight-serve`, please visit [eight-serve/README.md](eight-serve/README.md).
- For implementing an **Eight Client** yourself, visit [official implementation](eight/src/client/) and [expose module](eight/src/expose).
- For examples, visit [examples](examples/) directory.

## Commands

There are currently 9 different commands available:

- `set [key] [value]`: Create or update a value. Returns `ok` on success.
- `get [key]`: Get value from key. Returns value as `string` on success.
- `delete [key]`: Delete value from database. Returns `ok` on success.
- `exists [key]`: Check if key exists in database. Returns `boolean` on success.
- `incr [key] [number]`: Increment the value by given number. Returns update value as `number` on success.
- `decr [key] [number]`: Decrement the value by given number. Returns update value as `number` on success.
- `search [key]`: Search keys. Returns list of `string` on success.
- `flush`: Flush database. Returns `ok` on success.
- `downgrade`: Downgrade permission. Returns `ok` on success.

## Syntax

The first word is processed as a command, followed by arguments until it hits `;`.

```
set bob 10;
get bob;
```

### Comments

You can add comments using `#`. It will skip the characters until new line.

```
set bob 10; # this is a comment
# get bob; (this command is commented out)
```

### Strings

You can use strings for more complex values. Strings starts and ends with `"`.

When you type `simple value` it will be processed as `["simple", "value"]`. This is where strings become useful. You can simply put `"` to avoid this issue: `"simple value"`.

```
set test hello world; # this will not work! (value is `["simple", "value"]`)
set test "hello world"; # you should use this instead. (value is `"simple value"`)
```

### Variables

Eight query language also supports variables for way more complex data. You also should use variables to prevent an injection attack. Variables start with `$`. If a variable doesn't exist, it will simply return itself.

```json
{
  "user": "bob",
  "point": "10"
}
```

```
set $user $point; # ok
get $user; # 10
```

### Asynchronous Execution

To execute a command without waiting its result, add `?` at end of the command. You will not receive any response on these commands.

```
set? point 10; # we don't know the result
```

## Contributing

You can always report bugs and request features via [GitHub Issues](/issues).

For pull requests, make sure your code is well-formatted and at least can explain itself.

## License

Eight is licensed under the BSD-3-Clause license.
