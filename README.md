# Zed SML Extension

This extension adds Standard ML language support to the [Zed](https://zed.dev) text editor.

- Tree Sitter: [tree-sitter-sml](https://github.com/MatthewFluet/tree-sitter-sml)
- Language Server: [millet](https://github.com/azdavis/millet)

## Known Issues

There are currently a few known issues, primarily with the language server integration in this extension:

- Formatting with millet does not currently work (see below to use `smlfmt` as an external formatter configured in Zed)
- Project diagnostics do not auto resolve

You can disable the language server by configuring Zed:

```json
{
  "languages": {
    "Standard ML": {
      "enable_language_server": false
    }
  }
}
```

You can also configure Zed to use [`smlfmt`](https://github.com/shwestrick/smlfmt) to format your buffer when you save. Install `smlfmt` and make sure it's accessible in your path, and add the following configuration:

```json
{
  "languages": {
    "Standard ML": {
      "enable_language_server": false,
      "formatter": {
        "external": {
          "command": "smlfmt"
        }
      }
    }
  }
}
```

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

### Dependency Licenses

- `tree-sitter-sml` licensed under [MIT](https://github.com/MatthewFluet/tree-sitter-sml/blob/main/LICENSE)
- `millet` lincesed under either [MIT or Apache](https://github.com/azdavis/millet#license)
