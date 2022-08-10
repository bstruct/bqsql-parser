const rust = import('./pkg/bqsql_parser.js');

rust
    .then(m => {

        const p = m.parse('--my comments\nSELECT 2+2');

        console.info(JSON.stringify(p));

        const p2 = m.get_all_functions();
        console.info(JSON.stringify(p2));        


    })
    .catch(console.error);