import { BqsqlDocument, BqsqlDocumentComment, BqsqlDocumentItem, BqsqlDocumentItemType, BqsqlDocumentQuery } from "./bqsqlDocument";
import { TextPosition } from "./textPosition";

export class BqsqlParser {

    private static dashCharCode: number = 45;

    private static regNonWhite = RegExp(/\S/);
    private static regSelect = RegExp(/^select\W+/i);
    private static regAll = RegExp(/^all\W+/i);
    private static regDistinct = RegExp(/^distinct\W+/i);
    private static regAsValue = RegExp(/^as\W+value\W+/i);
    private static regAsStruct = RegExp(/^as\W+struct\W+/i);
    private static regStar = RegExp(/^\*\W+/i);
    private static regAnyWord = RegExp(/^([A-z]+|[0-9]+|_)(\W+|,)/i);
    private static regFrom = RegExp(/^from\W+/i);

    public static parse(bsqlStatement: string): BqsqlDocument {

        const items: BqsqlDocumentItem[] = [];
        if ((!bsqlStatement) || bsqlStatement.length === 0) { return { items }; }

        const length = bsqlStatement.length;
        // let _line = 0;
        // let _character = 0;
        let textPosition = new TextPosition(0, 0, 0);
        while (textPosition.textIndex < length) {
            const charCode = bsqlStatement.charCodeAt(textPosition.textIndex);

            if (charCode === BqsqlParser.dashCharCode) {
                if (textPosition.textIndex + 1 < length && bsqlStatement.charCodeAt(textPosition.textIndex + 1) === BqsqlParser.dashCharCode) {
                    const result = BqsqlParser.handleComment(bsqlStatement, textPosition);
                    items.push(result[0]);
                    textPosition = result[1];
                    continue;
                }
            }

            //only supported at the moment: 
            // comments, 
            // DECLARE (Procedural language), 
            // WITH (query), 
            // SELECT (query)

            let char = String.fromCharCode(charCode).toUpperCase();
            switch (char) {
                case 'D':
                    //try to find DECLARE
                    break;
                case 'W':
                    //try to find WITH
                    break;
                case 'S': //try to find SELECT
                    const result = BqsqlParser.handleSelect(bsqlStatement, textPosition);
                    items.push(result[0]);
                    textPosition = result[1];
                    break;
                default:
                    const regMatch = BqsqlParser.regNonWhite.exec(char);
                    if (regMatch && regMatch.length > 0) {
                        //not expected character. give up until full language is supported.
                        return { items };
                    } else {
                        textPosition = BqsqlParser.handleNonCharacters(bsqlStatement, textPosition);
                    }
                    break;
            }

        }

        return { items };
    }

    private static handleComment(bsqlStatement: string, textPosition: TextPosition): [BqsqlDocumentItem, TextPosition] {

        const item = {
            type: BqsqlDocumentItemType.comment
        } as BqsqlDocumentComment;

        const newIndex = bsqlStatement.indexOf('\n', textPosition.textIndex);

        item.content = bsqlStatement.substring(textPosition.textIndex + 2, newIndex);

        const newPosition = new TextPosition(newIndex + 1, textPosition.line + 1, 0);

        return [item, newPosition];
    }

    private static handleNonCharactersAfter(bsqlStatement: string, textPosition: TextPosition, initialKeywordSize: number): TextPosition {
        return BqsqlParser.handleNonCharacters(bsqlStatement, new TextPosition(textPosition.textIndex + initialKeywordSize, textPosition.line, textPosition.character + initialKeywordSize));
    }

    private static handleNonCharacters(bsqlStatement: string, textPosition: TextPosition): TextPosition {

        let newTextIndex = textPosition.textIndex;
        let newLine = textPosition.line;
        let newCharacter = textPosition.character;
        let char = bsqlStatement.charAt(newTextIndex);

        while (!BqsqlParser.regNonWhite.exec(char)) {

            switch (char) {
                case '\n':
                    newTextIndex++;
                    newLine++;
                    newCharacter = 0;
                    break;
                case '\t':
                case ' ':
                    newTextIndex++;
                    newCharacter++;
                    break;
                default:
                    const charCode = bsqlStatement.charCodeAt(newTextIndex);
                    throw new Error(`Unexpected white space character with code :${charCode}`);
            }

            char = bsqlStatement.charAt(newTextIndex);
        }

        const newPosition = new TextPosition(newTextIndex, newLine, newCharacter);

        return newPosition;
    }

    private static handleUnknown(bsqlStatement: string, textPosition: TextPosition): [BqsqlDocumentItem, TextPosition] {

        throw new Error('not implemented');

    }

    private static handleSelect(bsqlStatement: string, textPosition: TextPosition): [BqsqlDocumentItem, TextPosition] {

        let newPosition: TextPosition = textPosition;

        // if (textPosition.textIndex + 6 < bsqlStatement.length) {
        const confirmSelect = bsqlStatement.substring(textPosition.textIndex);
        const matchSelect = BqsqlParser.regSelect.exec(confirmSelect);
        if (matchSelect && matchSelect.length && matchSelect.index === 0) {
            newPosition = BqsqlParser.handleNonCharactersAfter(bsqlStatement, newPosition, 6);
        } else {
            return BqsqlParser.handleUnknown(bsqlStatement, textPosition);
        }
        // } else {
        //     return BqsqlParser.handleUnknown(bsqlStatement, textPosition);
        // }

        const item = {
            type: BqsqlDocumentItemType.query,
            all: false,
            distinct: false,
            asValue: false,
            asStruct: false
        } as BqsqlDocumentQuery;

        while (true) {
            
            const next = bsqlStatement.substring(newPosition.textIndex);
            debugger;

            if (BqsqlParser.regFrom.exec(next)?.length === 1) { break; }

            switch (1) {
                case BqsqlParser.regAll.exec(next)?.length:
                    item.all = true;
                    newPosition = BqsqlParser.handleNonCharactersAfter(bsqlStatement, newPosition, 3);
                    break;
                case BqsqlParser.regDistinct.exec(next)?.length:
                    item.distinct = true;
                    newPosition = BqsqlParser.handleNonCharactersAfter(bsqlStatement, newPosition, 8);
                    break;
                case BqsqlParser.regAsValue.exec(next)?.length:
                    break;
                case BqsqlParser.regAsStruct.exec(next)?.length:
                    break;
                case BqsqlParser.regStar.exec(next)?.length:
                    break;
                case BqsqlParser.regAnyWord.exec(next)?.length:
                    const match = BqsqlParser.regAnyWord.exec(next);
                    debugger;
                    newPosition = BqsqlParser.handleNonCharactersAfter(bsqlStatement, newPosition, 8);
                    break;
                default:
                    throw new Error('handleSelect unexpected');
            }
        }

        //inside select
        //*
        //

        // const newIndex = bsqlStatement.indexOf('\n', textPosition.textIndex);

        // item.content = bsqlStatement.substring(textPosition.textIndex + 2, newIndex);

        // const newPosition = new TextPosition(1, textPosition.line + 1, 0);

        return [item, newPosition];
    }

}