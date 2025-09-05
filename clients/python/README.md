Neutral TS Python IPC Client
============================================

Requires the IPC server: [Neutral TS IPC Server](https://github.com/FranBarInstance/neutral-ipc/releases)

See: [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/python)

Use:

```
from neutral_ipc_template import NeutralIpcTemplate

schema = """
{
    "config": {
        "cache_prefix": "neutral-cache",
        "cache_dir": "",
        "cache_on_post": false,
        "cache_on_get": true,
        "cache_on_cookies": true,
        "cache_disable": false,
        "disable_js": false,
        "filter_all": false
    },
    "inherit": {
        "snippets": {},
        "declare": {},
        "locale": {
            "current": "en",
            "trans": {
                "en": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "es": {
                    "Hello nts": "Hola",
                    "ref:greeting-nts": "Hola"
                },
                "el": {
                    "Hello nts": "Γεια σας",
                    "ref:greeting-nts": "Γεια σας"
                }
            }
        }
    },
    "data": {
        "CONTEXT": {
            "ROUTE": "",
            "HOST": "",
            "GET": {},
            "POST": {},
            "HEADERS": {},
            "FILES": {},
            "COOKIES": {},
            "SESSION": {},
            "ENV": {}
        },
        "hello": "Hello",
        "arr": {
            "hello": "Hello"
        }
    }
}
"""

template = NeutralIpcTemplate("file.ntpl", schema)
contents = template.render()

# e.g.: 200
status_code = template.get_status_code()

# e.g.: OK
status_text = template.get_status_text()

# empty if no error
status_param = template.get_status_param()

# act accordingly at this point according to your framework
```

Links
-----

Neutral TS template engine Python Package.

- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [Repository](https://github.com/FranBarInstance/neutraltemplate)
- [Crate](https://crates.io/crates/neutralts)
- [PYPI Package](https://pypi.org/project/neutraltemplate/)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/python)
