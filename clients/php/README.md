# Neutral TS IPC Client PHP

This package provides a minimal PHP client for Neutral TS templates using the IPC server.

The client exposes `NeutralIpcTemplate` which lets you render a template by sending a schema
and template path (or source) to the IPC server and receiving the rendered content.

You can send the schema in JSON or MsgPack format:

- JSON: `$schemaType = NeutralIpcRecord::CONTENT_JSON` (default)
- MsgPack: `$schemaType = NeutralIpcRecord::CONTENT_MSGPACK`

Example (JSON)
-------

```php
// Include NeutralIpcTemplate: https://github.com/FranBarInstance/neutral-ipc
include 'neutral_ipc_template/NeutralIpcTemplate.php';

// The schema contains the data and variables for the template
$schema = [
    'data' => [
        'hello' => 'Hello World',
    ],
];

// Determine the template full path
$template = __DIR__ . '/template.ntpl';

// Create an instance of NeutralIpcTemplate (JSON format - default)
$ipc_template = new NeutralIpcTemplate($template, $schema);

// Render the template
$contents = $ipc_template->render();

// e.g.: 200
$status_code = $ipc_template->get_status_code();

// e.g.: OK
$status_text = $ipc_template->get_status_text();

// empty if no error
$status_param = $ipc_template->get_status_param();

// Act according to your framework to display the content
// for this example, simply output
echo $contents;

```

Example (MsgPack)
-------

```php
// Include NeutralIpcTemplate: https://github.com/FranBarInstance/neutral-ipc
include 'neutral_ipc_template/NeutralIpcTemplate.php';

// The schema contains the data and variables for the template
$schema = [
    'data' => [
        'hello' => 'Hello World',
    ],
];

// Determine the template full path
$template = __DIR__ . '/template.ntpl';

// Create an instance of NeutralIpcTemplate (MsgPack format)
$ipc_template = new NeutralIpcTemplate($template, $schema, NeutralIpcRecord::CONTENT_PATH, NeutralIpcRecord::CONTENT_MSGPACK);

// Render the template
$contents = $ipc_template->render();

// e.g.: 200
$status_code = $ipc_template->get_status_code();

// e.g.: OK
$status_text = $ipc_template->get_status_text();

// empty if no error
$status_param = $ipc_template->get_status_param();

// Act according to your framework to display the content
// for this example, simply output
echo $contents;

```

Links
-----

Neutral TS template engine.

- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [Repository](https://github.com/FranBarInstance/neutralts)
- [Crate](https://crates.io/crates/neutralts)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples)
