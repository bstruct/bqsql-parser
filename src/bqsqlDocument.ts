export interface BqsqlDocument {

    items: BqsqlDocumentItem[];
}

export enum BqsqlDocumentItemType {
    unknown = 0,
    comment = 1,
    query = 2,
    //Data definition language (DDL)
    //Data manipulation language (DML)
    //Data control language (DCL)
    //Procedural language
    //Debugging statements
    //Other statements in Standard SQL

}

export interface BqsqlDocumentItem {
    type: BqsqlDocumentItemType;
}

export interface BqsqlDocumentComment extends BqsqlDocumentItem {
    content: string;
}

export interface BqsqlDocumentQuery extends BqsqlDocumentItem {
    all: boolean;
    distinct: boolean;
    asValue: boolean;
    asStruct: boolean;
    selectList: string[];
    from: string | null;
    joins: string[];
    where: string | null;
    groupBy: string | null;
    having: string | null;
    qualify: string | null;
    window: string | null;
    limit: number | null;
    offset: number | null;
}