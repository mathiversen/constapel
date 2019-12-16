# Constapel

**This is a work in progress**

A program to maintain constants throughout a web application.

### Example

```shell
    constapel constants.yaml
```

### TODO

Cleanup

- [ ] Cleanup file_creator
- [ ] Cleanup the different filetypes, concatenate some logic, maybe create struct(s) for the with different display formatting

New functionality

- [ ] Better YAML support, making it possible to use & and \* to reference another group.
- [ ] Support for advanced constant refering, ex `key: 'rgba(*colors.red, *opacity)'`

Tests

- [ ] JS-files
- [ ] SCSS-files
- [ ] CSS-files
- [ ] References
- [ ] Constant groups with dashes and/or capitalized letters
- [ ] Different types of yaml values (String, Number)
- [ ] Nested constant groups (works how in css? -> `--group-nested-nested-key: 'value'`)
