const rust = import('./pkg/bqsql_parser.js');

rust
    .then(m => {

        const p = m.parse('--my comments\nSELECT 2+2');

        console.info(JSON.stringify(p));

    })
    .catch(console.error);