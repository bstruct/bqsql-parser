use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct BqsqlFunction {
    name: &'static str,
    snippet: &'static str,
    url: &'static str,
}

impl BqsqlFunction {
    pub(crate) fn get_all() -> Vec<BqsqlFunction> {
        vec![
        //aggregate_functions
        //https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions

        BqsqlFunction {
            name: "ANY_VALUE",
            snippet: "ANY_VALUE(${0:some_column}) AS ${1:any_value},",
            url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#any_value"
        },
        BqsqlFunction {
            name: "ANY_VALUE OVER",
            snippet: "ANY_VALUE(${0:some_column}) OVER (ORDER BY ${1:some_column} ROWS BETWEEN ${2:1} PRECEDING AND CURRENT ROW) AS ${3:any_value},",
            url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#any_value"
        },
        BqsqlFunction {
            name: "ARRAY_AGG",
            snippet: "ARRAY_AGG(x) AS array_agg,",
            url: ""
        },
        BqsqlFunction {
            name: "ARRAY_CONCAT_AGG",
            snippet: "ARRAY_CONCAT_AGG(x) AS array_concat_agg,",
            url: ""
        },
        BqsqlFunction {
            name: "AVG",
            snippet: "AVG(x) as avg,",
            url: ""
        },
        BqsqlFunction {
            name: "BIT_AND",
            snippet: "BIT_AND(x) as bit_and,",
            url: ""
        },
        BqsqlFunction {
            name: "BIT_OR",
            snippet: "BIT_OR(x) as bit_or,",
            url: ""
        },
        BqsqlFunction {
            name: "BIT_XOR",
            snippet: "BIT_XOR(x) AS bit_xor,",
            url: ""
        },
        BqsqlFunction {
            name: "COUNT",
            snippet: "COUNT(*) AS count_star,",
            url: ""
        },
        BqsqlFunction {
            name: "COUNTIF",
            snippet: "COUNTIF(x>0) AS num_positive,",
            url: ""
        },
        BqsqlFunction {
            name: "LOGICAL_AND",
            snippet: "LOGICAL_AND(x) AS logical_and,",
            url: ""
        },
        BqsqlFunction {
            name: "LOGICAL_OR",
            snippet: "LOGICAL_OR(x) AS logical_or,",
            url: ""
        },
        BqsqlFunction {
            name: "MAX",
            snippet: "MAX(x) AS max,",
            url: ""
        },
        BqsqlFunction {
            name: "MIN",
            snippet: "MIN(x) AS min,",
            url: ""
        },
        BqsqlFunction {
            name: "STRING_AGG,",
            snippet: "STRING_AGG(fruit) AS string_agg",
            url: ""
        },
        BqsqlFunction {
            name: "SUM",
            snippet: "SUM(x) AS sum,",
            url: "https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#sum"
        }
        
        ]
    }
}
