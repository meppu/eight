# Eight

Modular asynchronous embedded key-value database.

Eight is a modular asynchronous embedded key-value database. I said modular because it has something much powerful that can change eight into anything, Storages. If something implements Storage trait, it can be used **Eight Server**, can be hosted with **Eight Expose** and then can be used with **Eight Client**. You can implement LRU Cache, Redis Storage, and even use with some database like MySQL, MongoDB, SurrealDB and so on... You can make your own storage implementation and take advantages of **Eight Server**: Redis-like query language, Asynchronous command execution, User permissions etc... This is why eight is not only a simple embedded database, it is a modular embedded database.

Eight currently ships two default storage implementations: In-memory storage and Filesystem based storage. If you don't like to use them, Make your own storage and publish it as a crate!

Visit `examples/` directory for more examples about eight.

- For more information about embedded database itself, please view [README.md](https://github.com/meppu/eight/blob/main/eight/README.md) on `eight/` directory.
- For more information about `eight-serve`, please view [README.md](https://github.com/meppu/eight/blob/main/eight-serve/README.md) on `eight-serve/` directory.
- For implementing an **Eight Client** yourself, [view official implementation](https://github.com/meppu/eight/tree/main/eight/src/client) and also visit [expose module](https://github.com/meppu/eight/tree/main/eight/src/expose).

## Commands

There are currently 9 different commands avaible:

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

The first argument is processed as command and then following by arguments until it hits `;`.

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

You can use strings for more complex values. Strings starts with `"` and ends with `"`.

When you type `simple value` it will be processed as `["simple", "value"]`. This is where strings become useful. You can simply put `"` to avoid this issue: `"simple value"`.

```
set test hello world; # this will not work!
set test "hello world"; # you should use this instead.
```

### Variables

Eight query language also supports variables for way more complex data. You also should use variables to prevent an injection attack. Variables start with `$`. If a variable doesn't exists, it will simply return itself.

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

To execute a command without waiting it is result, add `?` to end of the command. You will not receive any responses about these commands.

```
set? point 10; # we don't know the result
```

## Contributing

- Report bugs or request features via issues.
- Format your code.

## License

Eight is licensed under the BSD-3-Clause license.
