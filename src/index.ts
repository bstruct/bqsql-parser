import { BqsqlParser } from "./bqsqlParser";

const parsed = BqsqlParser.parse(`-- test lasdjflasjdf

\t   SELECT 
    pimExportDate, 
    Combi_number,
    
    -- Flavour_Copy
FROM \`damiao-project-1.PvhTest.PimExport\` pim
WHERE 
    pimExportDate = "2022-03-23"
    -- AND (
    --     Combi_number = '0000F3223E001'
    --     OR Combi_number = "0000F2934E101"
    -- )
LIMIT 101;`);